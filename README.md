# rust_web

## 项目介绍
是关于rust的一个web项目

内容参考自：b站视频：BV1RP4y1G7KF

代码部分参考自：https://github.com/BloomingDream/rust_web_server_from_yang

## 总结：
1. web共享数据：actix_web::web::Data, AppState
2. 路由：参数：共享数据和各自的参数
3. 错误处理：
    - 定义自己的错误类型，可以使用枚举
    - 实现自定义错误 到 web response的转换，impl actix_web::error::ResponseError
    - 实现底层错误到自定义错误的转换
    - 底层错误 ---> 自定义错误 ---> web response