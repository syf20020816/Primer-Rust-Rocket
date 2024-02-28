// 引入rocket框架宏
#[macro_use]
extern crate rocket;

// 引入标准库中的路径处理模块
use std::path::{Path, PathBuf};
// 引入rocket框架的文件服务和响应模块
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
// 引入rocket框架的序列化模块，用于JSON处理
use rocket::serde::json::{serde_json, Json};
use rocket::serde::{Deserialize, Serialize};
// 引入rocket框架的文件路径处理函数
use rocket::fs::relative;

// 定义一个用户结构体，包含用户的名字和年龄
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u8,
}

impl User {
    // 实现User的构造函数，方便创建User实例
    pub fn new(name: &str, age: u8) -> Self {
        User {
            name: String::from(name),
            age,
        }
    }
}

// 定义一个返回静态字符串的路由处理函数
#[get("/str")]
fn test_str() -> &'static str {
    // 返回一个JSON格式的字符串
    r#"{\"name\":\"Matt1\",\"age\":10}"#
}

// 定义一个返回String的路由处理函数
#[get("/string")]
fn test_string() -> String {
    // 创建一个User实例并将其转换为JSON字符串
    let user = User::new("Matt1", 10);
    serde_json::to_string(&user).unwrap()
}

// 定义一个异步路由处理函数，返回一个文件
#[get("/file")]
async fn test_option() -> Option<NamedFile> {
    // 构建文件的路径并尝试打开该文件
    let path = Path::new(relative!("static")).join("index.html");
    NamedFile::open(path).await.ok()
}

// 定义一个返回结果的路由处理函数，可能返回User的JSON或NotFound错误
#[get("/res/<name>")]
fn test_result(name: &str) -> Result<Json<User>, NotFound<String>> {
    // 当请求的name参数为"Matt"时，返回一个User实例的JSON
    if "Matt".eq(name) {
        let user = User::new("Matt1", 10);
        return Ok(Json(user));
    }
    // 否则返回一个NotFound错误
    Err(NotFound(String::from("only Matt can be responsed")))
}

// rocket框架的启动函数，用于构建和启动web服务
#[launch]
fn rocket() -> _ {
    // 构建rocket实例并挂载路由
    rocket::build().mount(
        "/api",
        routes![test_str, test_option, test_result, test_string],
    )
}
