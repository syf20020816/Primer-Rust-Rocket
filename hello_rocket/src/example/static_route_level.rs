#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

//等级越低，优先级越高
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/static", FileServer::from("static/ "))
        .mount("/pages", FileServer::from("static/src/pages").rank(10))
        .mount("/components", FileServer::from("static/src/components"))
        .mount("/pages", FileServer::from("static/src/components").rank(1))
}
