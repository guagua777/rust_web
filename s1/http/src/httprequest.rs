use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}


impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninitialized,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}







#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

// 所有字段都持有 String（包括 Resource::Path 内部），它们需要拥有数据的所有权，而不是借用。
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msd_body: String,
}


// 没有再使用切片
// From trait 的语义要求
// From<String> 意味着：

// 输入的 String 会被消费（consumed）
// 函数获得参数的所有权，可以自由处理
// 如果用 From<&str>，只能借用数据，无法将借用的数据存储到需要所有权的字段中
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msd_body: parsed_msg_body.to_string(),
        }
    }
}


fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let methods = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        methods.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() {
        key = k.trim().to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.trim().to_string();
    }
    (key, value)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_method_from_str() {
        assert_eq!(Method::Get, "GET".into());
        assert_eq!(Method::Post, "POST".into());
        assert_eq!(Method::Uninitialized, "OTHER".into());
    }


    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }


    #[test]
    fn test_read_http() {
        let s =
            String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n'");
        let mut expected_header: HashMap<String, String> = HashMap::new();
        expected_header.insert("Host".into(), "localhost".into());
        expected_header.insert("Accept".into(), "*/*".into());
        expected_header.insert("User-Agent".into(), "curl/7.71.1".into());

        println!("{:?}", expected_header);
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(expected_header, req.headers);
    }


}
