use std::collections::HashMap;
use std::io::{Result, Write};

// HttpResponse结构体
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse {
    //版本
    version: String,
    //响应码
    status_code: String,
    //响应消息
    status_msg: String,
    //响应头
    headers: Option<HashMap<String, String>>,
    //响应体
    body: Option<String>,
}

impl HttpResponse {
    //初始化HttpResponse
    pub fn new() -> Self {
        HttpResponse {
            version: String::from("HTTP/1.1"),
            status_code: String::from("200"),
            status_msg: String::from("OK"),
            headers: None,
            body: None,
        }
    }
    //全参构造
    pub fn new_all_args(
        status_code: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
    ) -> HttpResponse {
        let mut response: HttpResponse = HttpResponse::new();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        //匹配响应头
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                Some(h)
            }
        };
        //对各类响应码的处理
        response.status_msg = match response.status_code() {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into()
        };
        response.body = body;
        response
    }
    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }
    fn version(&self) -> &str {
        &self.version
    }
    fn status_code(&self) -> &str {
        &self.status_code
    }
    fn status_msg(&self) -> &str {
        &self.status_msg
    }
    //Option<HashMap<String, String>>转换为String
    fn headers(&self) -> String {
        let map: HashMap<String, String> = self.headers.clone().unwrap();
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

impl From<HttpResponse> for String {
    //通过format宏将HttpResponse转换为String字符串
    fn from(value: HttpResponse) -> Self {
        let value = value.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &value.version(),
            &value.status_code(),
            &value.status_msg(),
            &value.headers(),
            &value.body().len(),
            &value.body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //测试
    #[test]
    fn test_response_struct_creation_200() {
        //测试全参构造
        let response_actual = HttpResponse::new_all_args(
            "200",
            None,
            Some("xxxx".into()),
        );
        //测试正常直接通过struct构造
        let response_expected = HttpResponse {
            version: "HTTP/1.1".to_string(),
            status_code: "200".to_string(),

            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                Some(h)
            },
            body: Some("xxxx".into()),
            status_msg: "OK".to_string(),
        };
        assert_eq!(response_actual, response_expected);
    }
}
