#[macro_use]
extern crate rocket;

use std::io::Cursor;
use rocket::response::{status, Responder, Response};
use rocket::http::{Status, ContentType};
use rocket::Request;
use rocket::serde::json::{Json, serde_json};
use rocket::serde::{Serialize, Deserialize};

// 自定义一个JSON形式的统一Responder
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct ResultJsonData<T: Serialize> {
    //返回码
    code: u16,
    //响应数据
    data: Option<T>,
    //响应消息
    msg: String,
}

impl<'r, T: Serialize> Responder<'r, 'static> for ResultJsonData<T> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();
        //返回响应
        Response::build()
            //仅表示服务器返回响应状态
            .status(Status::Ok)
            //设置响应的ContentType
            .header(ContentType::JSON)
            //通过序列化计算
            .sized_body(json.len(), Cursor::new(json))
            //完成构建
            .ok()
    }
}


impl<T: Serialize> ResultJsonData<T> {
    //常规构建
    pub fn new(code: u16, data: T, msg: &str) -> Self {
        ResultJsonData {
            code,
            data: Some(data),
            msg: String::from(msg),
        }
    }
    //提供响应成功的快速构建方式
    pub fn success(data: T) -> Self {
        ResultJsonData::new(200, data, "success")
    }
    //提供响应失败的快速构建方式
    pub fn failure(data: T, msg: &str) -> Self {
        ResultJsonData::new(500, data, msg)
    }
    pub fn define_failure(code: u16, msg: &str) -> Self {
        ResultJsonData {
            code,
            data: None,
            msg: msg.to_string(),
        }
    }
}


// 采用Rocket框架提供给的serde进行序列化与反序列化
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: &str, age: u8) -> Self {
        User {
            name: String::from(name),
            age,
        }
    }
}

#[get("/test")]
fn define_excp() -> Status {
    //....
    Status::InternalServerError
}

#[catch(500)]
fn define_excp_handler() -> status::Custom<ResultJsonData<Option<String>>> {
    println!("Define Exception");
    status::Custom(
        Status::InternalServerError,
        ResultJsonData::define_failure(
            10001, "Define Exception Handler!",
        )
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![define_excp])
        .register("/api",catchers![define_excp_handler])
}
