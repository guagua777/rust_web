use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}


// 为什么这里要加 .into()
// 1. 显式表达类型转换意图
// .into() 调用了 Into trait，明确告诉读者：这里发生了类型转换。代码意图更清晰。

// 2. 代码风格一致性
// 所有三个字符串字段都使用 .into()，保持统一的代码风格：

// rust
// version: "HTTP/1.1".into(),
// status_code: "200".into(),
// status_text: "OK".into(),
// 3. 防御性编程
// 如果未来字段类型发生变化（比如改为 String），.into() 仍然能工作（因为 &str 可以通过 .into() 转换为 String），减少代码修改量。

// 总结
// .into() 在这个场景下不是必须的，但它是一种良好的代码风格：

// 显式表达类型转换意图
// 保持代码一致性
// 增强代码的可维护性
// 实际上，如果去掉 .into()，编译器会自动完成 &'static str 到 &'a str 的强制转换，代码同样能正常工作。



impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}



impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        // 因为 &res.body.unwrap().len(),会借用res，所以这里需要clone
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            // &res1.body.unwrap().len(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        // response.headers = match headers {
        // //    Some(_) => headers,
        //     Some(_h) => headers,
        //     None => {
        //         let mut h = HashMap::new();
        //         h.insert("Content-Type", "text/html");
        //         Some(h)
        //     }
        // };


        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };
        response.body = body;
        response
    }



    // 当前代码使用 clone() 的原因：

    // String::from(res) 需要 HttpResponse 的所有权
    // 而 send_response 方法只接收 &self（引用）
    // 通过 clone() 创建副本获取所有权
    // 优化建议： 这段代码设计不太合理，clone() 是不必要的性能开销。应该直接通过方法获取各字段的值来构建响应字符串，避免不必要的克隆。
    
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }


    // HTTP协议标准
    // 根据 RFC 7230（HTTP/1.1 协议规范），HTTP消息中的行分隔符必须是 CRLF（\r\n）。

    // 换行符解释
    // 符号	名称	ASCII码	作用
    // \r	Carriage Return (回车)	13	将光标移动到行首
    // \n	Line Feed (换行)	10	将光标移动到下一行
    // \r\n	CRLF	13+10	完整的换行序列
    // 为什么HTTP要求使用 CRLF
    // 1. 历史原因
    // 这源于早期的电传打字机时代：

    // \r 让打印头回到行首
    // \n 让纸向上滚动一行
    // 两者结合才能完成一次完整的换行
    // 2. 协议规范要求
    // HTTP/1.1 协议明确规定：


    // plainText
    // HTTP-message = start-line
    //             *( header-field CRLF )
    //             CRLF
    //             [ message-body ]
    // 所有HTTP消息的行分隔符都必须是 \r\n，包括：

    // 请求行/状态行
    // 头部字段
    // 头部与正文之间的空行
    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx"
                .to_string();
        assert_eq!(http_string, actual_string);
    }
}