// 一、super 关键字的含义
// super 在 Rust 中是一个模块路径关键字，用于引用当前模块的父模块（上一级模块）。

// 二、模块层次结构
// 根据目录结构，我们可以理解模块关系：


// plainText
// httpserver/src/
// ├── main.rs          // 根模块 (crate root)
// ├── server.rs        // 当前文件，属于根模块的子模块
// ├── router.rs        // Router 所在的模块
// └── handler.rs
// Rust 的模块层次如下：


// plainText
// crate (根)
// └── server (当前模块，位于 server.rs)
//     └── super -> crate (父模块)
//         └── router -> Router (目标类型)
// 三、代码解释

// rust
// use super::router::Router;
// 部分	含义
// use	导入关键字
// super	引用当前模块的父模块（即 crate 根模块）
// router	父模块下的 router 子模块
// Router	从 router 模块中导入的类型
// 等价写法：


// rust
// use crate::router::Router;  // crate 表示根模块
// 四、super 的使用场景
// 场景1：从父模块导入
// 假设目录结构如下：


// plainText
// src/
// ├── lib.rs
// ├── a/
// │   └── mod.rs      // 模块 a
// └── b/
//     └── mod.rs      // 模块 b
// 如果在 b/mod.rs 中想导入 a 模块：


// rust
// // b/mod.rs
// use super::a::SomeType;  // super 指向 src/
// 场景2：避免硬编码路径
// 使用 super 可以使代码更灵活，当模块结构变化时，相对路径可能仍然有效。

// 五、模块路径对比
// 路径类型	语法	说明
// 绝对路径	use crate::router::Router;	从根模块开始
// 相对路径	use super::router::Router;	从父模块开始
// 当前模块	use self::some_item;	从当前模块开始
// 六、在你的项目中
// 当前文件是 server.rs，它想导入 router.rs 中的 Router 类型：


// plainText
// crate (根模块)
// ├── server.rs    (当前文件，编译为 crate::server 模块)
// └── router.rs    (crate::router 模块，包含 Router)
// 所以：

// super 指向 crate（根模块）
// super::router 就是 crate::router
// super::router::Router 就是 crate::router::Router
// 总结
// super 关键字用于引用当前模块的父模块，它提供了一种相对路径导入的方式。在你的代码中：


// rust
// use super::router::Router;
// 等价于：


// rust
// use crate::router::Router;
// 这两种写法都能正确导入 Router 类型，选择哪种取决于代码风格和模块结构的复杂度。

use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 2083];
            stream.read(&mut read_buffer).unwrap();
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            Router::route(req, &mut stream)
        }
    }
}