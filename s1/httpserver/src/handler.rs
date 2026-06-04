use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;



// 在 Rust 中，trait 的方法不需要显式加 pub，这是 Rust 的一个特殊设计。

// 具体规则
// 当一个 trait 被声明为 pub 时（如你的代码第 7 行 pub trait Handler），它内部所有的方法默认都是公开的，即使没有写 pub 关键字。


// rust
// pub trait Handler {
//     // 这里不需要 pub，因为 trait 本身是 pub 的
//     fn handle(req: &HttpRequest) -> HttpResponse;
//     fn load_file(file_name: &str) -> Option<String> {
//         // ...
//     }
// }
// 背后的原因
// 这是因为：

// 一致性原则：当你实现一个 trait 时，必须实现它的所有方法。如果 trait 是公开的，那么它的方法也应该是公开的，否则外部代码无法调用这些方法。
// 简化语法：避免在每个方法前都重复写 pub，让代码更简洁。
// 对比普通结构体方法
// 与之形成对比的是，普通结构体的方法必须显式加 pub 才能对外公开：


// rust
// pub struct MyStruct;

// impl MyStruct {
//     // 这个方法是私有的
//     fn private_method() {}
    
//     // 这个方法是公开的
//     pub fn public_method() {}
// }


// 想一想为什么定义trait，相当于定义接口
pub trait Handler {
    // 不需要用 pub fn 吗？
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

// 订单状态
#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Tvpe", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                // 取其引用 &Self::load_json()
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}