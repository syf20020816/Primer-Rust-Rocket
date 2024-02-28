#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build()
        //挂载static目录
        .mount("/static", FileServer::from("static/"))
}
