//第4章/main.rs
#[macro_use]
extern crate rocket;

use rocket::http::{CookieJar, Cookie};

#[get("/cookie/add")]
fn add_cookie(cookies: &CookieJar<'_>) -> () {
    cookies.add_private(Cookie::new("my_secret", "123456"))
}

#[get("/cookie/get")]
fn get_cookie(cookies: &CookieJar<'_>) -> String {
    //密文
    let my_cookie = cookies.get("my_secret").unwrap();
    let my_secret = cookies.get_private("my_secret").unwrap();
    format!("密文:{} \n明文:{}", my_cookie, my_secret)
}

#[get("/cookie/del")]
fn del_cookie(cookies: &CookieJar<'_>) -> () {
    cookies.remove_private(Cookie::named("my_secret"));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![add_cookie,get_cookie,del_cookie])
}
