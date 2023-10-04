//第5章/main.rs
#[macro_use]
extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use rocket::serde::json::{Json, serde_json};
use rocket::serde::{Serialize, Deserialize};
use rocket::fs::relative;

#[derive(Debug, Serialize, Deserialize)]
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

#[get("/str")]
fn test_str() -> &'static str {
    r#"{\"name\":\"Matt1\",\"age\":10}"#
}

#[get("/string")]
fn test_string() -> String {
    let user = User::new("Matt1", 10);
    serde_json::to_string(&user).unwrap()
}

#[get("/file")]
async fn test_option() -> Option<NamedFile> {
    let mut path = Path::new(relative!("static")).join("index.html");
    NamedFile::open(path).await.ok()
}

#[get("/res/<name>")]
fn test_result(name:&str) -> Result<Json<User>, NotFound<String>> {
    if "Matt".eq(name){
        let user = User::new("Matt1", 10);
        return Ok(Json(user));
    }
    Err(NotFound(String::from("only Matt can be responsed")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![test_str,test_option,test_result,test_string])
}
