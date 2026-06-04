use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;


// 这个问题涉及到 Rust 的所有权系统，让我来详细解释一下。

// 核心原因：避免所有权转移
// 在 Router::route 方法中，req: HttpRequest 是通过值传递进来的。如果直接写 match req.resource 而不是 match &req.resource，会发生以下情况：

// 不使用引用的后果

// rust
// // 如果这样写...
// match req.resource {  // 这里会把 resource 的所有权从 req 中移走
//     httprequest::Resource::Path(s) => {
//         // ...
//         WebServiceHandler::handle(&req);  // ❌ 错误！req 已经不完整了
//     }
// }
// 因为 resource 是 HttpRequest 的一个字段，当你直接 match req.resource 时，resource 的所有权会被转移到 match 分支中，导致 req 变得不完整（部分被移动），后续无法再使用 &req。

// 使用引用的好处

// rust
// match &req.resource {  // 只借用引用，不转移所有权
//     httprequest::Resource::Path(s) => {
//         // ...
//         WebServiceHandler::handle(&req);  // ✅ 正确！req 仍然完整
//     }
// }
// 通过 &req.resource，我们只是借用了 resource 的引用，而不是转移它的所有权。这样：

// req 保持完整，可以继续在后续代码中使用（如第 18、22、29 行的 handle(&req) 调用）
// 我们仍然可以检查 resource 的值来进行路由判断
// 代码流程验证
// 看一下你的代码流程：


// rust
// pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
//     match req.method {                     // 方法1：直接 match（method 是 Copy 类型）
//         httprequest::Method::Get => match &req.resource {  // 方法2：match 引用
//             httprequest::Resource::Path(s) => {
//                 // ...
//                 WebServiceHandler::handle(&req);  // 还需要使用完整的 req！
//             }
//         },
//         _ => {
//             PageNotFoundHandler::handle(&req);  // 还需要使用完整的 req！
//         }
//     }
// }
// 注意：req.method 可以直接 match 是因为 Method 很可能实现了 Copy trait（基本类型或简单枚举通常会实现 Copy），所以不会转移所有权。


// 总结
// 写法	效果	是否可用
// match req.resource	转移所有权，req 变得不完整	❌ 不可用
// match &req.resource	仅借用引用，req 保持完整	✅ 正确
// 这是 Rust 所有权系统的典型应用场景：当你需要检查一个字段但又不想失去对整个结构体的访问权时，就使用引用进行匹配。





pub struct Router;

impl Router {
    // https://doc.rust-lang.org/book/ch10-02-traits.html#using-traits-as-parameters
    // using trait as parameter
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // match 的是一个引用，为什么？
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}