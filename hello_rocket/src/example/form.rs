// 导入Rocket宏，用于简化代码的书写
#[macro_use]
extern crate rocket;

// 导入Rocket的form和serde模块，用于表单处理和序列化/反序列化
use rocket::form::{Form, FromForm};
use rocket::serde::{json::Json, Deserialize, Serialize};

// 定义User结构体，用于示例中的数据传输对象
#[serde(crate = "rocket::serde")] // 指定serde的来源为rocket框架
#[derive(Debug, Serialize, Deserialize, FromForm)] // 自动实现Debug, Serialize, Deserialize, 以及FromForm traits
struct User {
    id: String,
    username: String,
    password: String,
    #[field(name = "userAge")] // 自定义表单字段名映射，当表单字段名与结构体字段名不一致时使用
    user_age: u8, // 用户年龄
    verified: Verified, // 用户验证信息
}

// 定义Verified结构体，用于User中的验证信息
#[serde(crate = "rocket::serde")]
#[derive(Debug, Serialize, Deserialize, FromForm)]
struct Verified {
    email: String, // 邮箱
    phone: String, // 电话
}

// 定义处理JSON格式数据的路由
#[post("/form/json", format = "application/json", data = "<user>")]
fn json_form(user: Json<User>) -> String {
    format!("{:?}", user) // 直接格式化打印Json<User>实例
}

// 定义处理multipart/form-data格式数据的路由
#[post("/form/data", format = "multipart/form-data", data = "<user>")]
fn form_data(user: Form<User>) -> String {
    format!("{:?}", user) // 直接格式化打印Form<User>实例
}

// 定义处理application/x-www-form-urlencoded格式数据的路由
#[post(
    "/form/urlencoded",
    format = "application/x-www-form-urlencoded",
    data = "<user>"
)]
fn urlencoded_form(user: Form<User>) -> String {
    format!("{:?}", user) // 直接格式化打印Form<User>实例
}

// Rocket框架的启动函数，用于构建和挂载路由
#[launch]
fn rocket() -> _ {
    // 构建Rocket实例并挂载定义的路由
    rocket::build().mount("/api", routes![json_form, form_data, urlencoded_form])
}
