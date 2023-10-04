//第4章/main.rs
#[macro_use]
extern crate rocket;

use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

// 采用Rocket框架提供给的serde进行序列化与反序列化
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    username: String,
    password: String,
}

#[post("/login", format = "application/json", data = "<user>")]
fn login(user: Json<User>) -> String {
    format!("user : username = {} , password = {}", user.username, user.password)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![login])
}
