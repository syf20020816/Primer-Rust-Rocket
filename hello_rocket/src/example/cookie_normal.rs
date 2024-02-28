#[macro_use]
extern crate rocket;

use rocket::http::{Cookie, CookieJar};

#[get("/cookie/add")]
fn add_cookie(cookies: &CookieJar<'_>) -> () {
    cookies.add(Cookie::new("my_cookie", "rocket_cookie"));
}

#[get("/cookie/get")]
fn get_cookie(cookies: &CookieJar<'_>) -> String {
    let my_cookie = cookies.get("my_cookie").unwrap();
    String::from(my_cookie.value())
}

#[get("/cookie/del")]
fn del_cookie(cookies: &CookieJar<'_>) -> () {
    cookies.remove(Cookie::named("my_cookie"));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![add_cookie, get_cookie, del_cookie])
}
