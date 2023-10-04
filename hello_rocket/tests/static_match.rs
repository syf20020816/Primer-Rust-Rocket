#[macro_use]
extern crate rocket;

use std::path::PathBuf;
use rocket::fs::FileServer;

// 这依然使用GET请求，但发起HEAD请求
#[get("/index/<say>")]
fn index(say: &str) -> String {
    format!("🙂{}", say)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/apiV1_4", routes![index])
        .mount("/static", FileServer::from("static/"))
        .mount("/pages", FileServer::from("static/src/pages").rank(10))
        .mount("/components", FileServer::from("static/src/components"))
        .mount("/pages", FileServer::from("static/src/components").rank(1))
}
