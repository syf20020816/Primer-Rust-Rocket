// 第1章 Web/http_parser/src/httprequest.rs
// 本模块负责解析HTTP请求
use std::collections::HashMap;

/// 定义HttpRequest结构体，用于存储解析后的HTTP请求信息
#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,  // 表示HTTP请求方法（如GET, POST等）
    pub version: Version,  // 表示HTTP版本（如HTTP/1.1）
    pub resource: String,  // 请求的资源路径
    pub headers: HashMap<String, String>, // 存储请求头的键值对
}

impl HttpRequest {
    /// 构造函数，返回一个新的HttpRequest实例
    pub fn new() -> Self {
        HttpRequest {
            method: Method::None, // 默认方法为None
            version: Version::None, // 默认版本为None
            resource: String::new(), // 默认资源路径为空字符串
            headers: Default::default(), // 默认请求头为空
        }
    }

    /// 解析HTTP请求的起始行
    pub fn parse_line(&mut self, req: &str) {
        let mut lines = req.split_whitespace();
        let method = lines.next().unwrap();
        let resource = lines.next().unwrap();
        let version = lines.next().unwrap();

        self.method = method.into(); // 将方法字符串转换为Method枚举
        self.version = version.into(); // 将版本字符串转换为Version枚举
        self.resource = resource.into(); // 存储资源路径
    }

    /// 解析HTTP请求头
    pub fn parse_header_line(&mut self, s: &str) {
        let mut header_items = s.split(":");
        let key = header_items.next().unwrap_or("").trim().to_string();
        let value = header_items.next().unwrap_or("").trim().to_string();

        self.headers.insert(key, value); // 将请求头键值对存储到HashMap中
    }

    /// 将字符串形式的HTTP请求转化为HttpRequest结构体
    pub fn from_str(&mut self, value: String) -> Self {
        let mut http_req = HttpRequest::new();
        for line in value.lines() {
            if line.contains("HTTP") {
                http_req.parse_line(line); // 解析请求行
            } else if line.contains(":") {
                http_req.parse_header_line(line); // 解析请求头
            } // 忽略其他情况
        }
        http_req
    }
}

/// 定义HTTP请求方法的枚举
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    None, // 默认值，表示未指定方法
}

impl From<&str> for Method {
    /// 将字符串转换为Method枚举
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => Method::None, // 不匹配任何已知方法时返回None
        }
    }
}

/// 定义HTTP版本的枚举
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_0,
    V1_1,
    None, // 默认值，表示未指定版本
}

impl From<&str> for Version {
    /// 将字符串转换为Version枚举
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.0" => Version::V1_0,
            "HTTP/1.1" => Version::V1_1,
            _ => Version::None, // 不匹配任何已知版本时返回None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_http() {
        let s = String::from("GET /hello_world HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        let mut req: HttpRequest = HttpRequest::new();
        req = req.from_str(s); // 使用测试字符串构造HttpRequest
        dbg!(&req);
        assert_eq!(Method::Get, req.method); // 验证方法是否正确解析
        assert_eq!(Version::V1_1, req.version); // 验证版本是否正确解析
        assert_eq!("/hello_world".to_string(), req.resource); // 验证资源路径是否正确解析
    }
}
