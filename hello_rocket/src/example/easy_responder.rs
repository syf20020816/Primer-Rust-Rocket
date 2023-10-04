#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::http::Status;

#[get("/accept")]
fn accept() -> status::Accepted<String>{
    status::Accepted(Some(String::from("accept")))
}

#[get("/bad_req")]
fn bad_req() -> status::BadRequest<String>{
    status::BadRequest(Some(String::from("bad request")))
}

#[get("/custom")]
fn custom() -> status::Custom<String>{
    status::Custom(Status::Ok,String::from("Custom OK"))
}
#[get("/not_found")]
fn not_found() -> status::NotFound<String>{
    status::NotFound(String::from("not found"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![accept,bad_req,custom,not_found])
}
