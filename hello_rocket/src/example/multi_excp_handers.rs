#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::{status::*, Redirect};
use rocket::Request;

// 匹配更长更具体的路径
#[get("/excp/<code>")]
fn active_long_excp(code: u16) -> Status {
    println!("match longer route /api/excp/:{}", code);
    Status::new(code)
}

// 匹配较短的路径
#[get("/<code>")]
fn active_short_excp(code: u16) -> Status {
    println!("match shorter route /api/:{}", code);
    Status::new(code)
}

#[catch(499)]
fn handle_long_499(state: Status, _req: &Request) -> Custom<String> {
    Custom(state, String::from("Handle Long Request : 499!"))
}

#[catch(500)]
fn handle_long_500(_req: &Request) -> Custom<String> {
    Custom(
        Status::InternalServerError,
        String::from("Handle Long Request : 500!"),
    )
}

#[catch(404)]
fn handle_long_404(_req: &Request) -> NotFound<String> {
    NotFound(String::from("Handle Long Request : 404!"))
}

#[catch(499)]
fn handle_short_499(state: Status, _req: &Request) -> Custom<String> {
    Custom(state, String::from("Handle Short Request : 499!"))
}

#[catch(500)]
fn handle_short_500(_req: &Request) -> Custom<String> {
    Custom(
        Status::InternalServerError,
        String::from("Handle Short Request : 500!"),
    )
}

#[catch(404)]
fn handle_short_404(_req: &Request) -> NotFound<String> {
    NotFound(String::from("Handle Short Request : 404!"))
}

// 一个默认的错误处理器
#[catch(default)]
fn default_excp_handler(status: Status, req: &Request) -> Custom<String> {
    Custom(status, format!("url:{:?}", req.uri()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![active_short_excp, active_long_excp])
        .register("/", catchers![default_excp_handler])
        .register(
            "/api/excp",
            catchers![handle_long_499, handle_long_404, handle_long_500],
        )
        .register(
            "/api",
            catchers![handle_short_499, handle_short_404, handle_short_500],
        )
}
