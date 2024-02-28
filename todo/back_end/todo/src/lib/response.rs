use rocket::http::{ContentType, Status};
use rocket::response::{status, Responder, Response};
use rocket::serde::json::{serde_json, Json};
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;
use std::io::Cursor;

// 自定义一个JSON形式的统一响应结构体，用于统一后端响应格式
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")] // 使用Rocket框架的serde实例进行序列化和反序列化
pub struct ResultJsonData<T: Serialize> {
    // 响应时的状态码
    code: u16,
    // 响应的具体数据，使用泛型支持多种数据类型
    data: Option<T>,
    // 响应时的消息提示
    msg: String,
}

// 为ResultJsonData实现Responder trait，以便在Rocket框架中作为响应返回
impl<'r, T: Serialize> Responder<'r, 'static> for ResultJsonData<T> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        // 将数据序列化为JSON字符串
        let json = serde_json::to_string(&self).unwrap();
        // 构建响应对象
        Response::build()
            // 设置响应状态码
            .status(Status::Ok)
            // 设置响应内容类型为JSON
            .header(ContentType::JSON)
            // 设置响应体及其长度，使用Cursor包装以支持异步写入
            .sized_body(json.len(), Cursor::new(json))
            // 完成响应构建
            .ok()
    }
}

impl<T: Serialize> ResultJsonData<T> {
    // 通用的构造器方法，允许自定义状态码、数据和消息
    pub fn new(code: u16, data: T, msg: &str) -> Self {
        ResultJsonData {
            code,
            data: Some(data),
            msg: String::from(msg),
        }
    }
    // 快速创建成功响应的方法
    pub fn success(data: T) -> Self {
        ResultJsonData::new(200, data, "success")
    }
    // 快速创建失败响应的方法，不带数据
    pub fn failure(msg: &str) -> Self {
        ResultJsonData {
            code: 500,
            data: None,
            msg: String::from(msg),
        }
    }
    // 允许定义特定的失败状态码和消息
    pub fn define_failure(code: u16, msg: &str) -> Self {
        ResultJsonData {
            code,
            data: None,
            msg: msg.to_string(),
        }
    }
    // 提供一个方法获取内部状态码和数据，主要用于逻辑处理
    pub fn get(self) -> (u16, Option<T>) {
        (self.code, self.data)
    }
}

// 定义一个500错误处理器，返回自定义的错误响应
#[catch(500)]
pub fn define_excp_handler() -> status::Custom<ResultJsonData<Option<String>>> {
    println!("Define Exception");
    // 创建并返回一个自定义的500错误响应
    status::Custom(
        Status::InternalServerError,
        ResultJsonData::define_failure(10001, "Define Exception Handler!"),
    )
}
