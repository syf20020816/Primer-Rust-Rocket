//第4章/main.rs
#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket::request::{FromRequest, Outcome};
// 使用转发重定向功能
use rocket::response::Redirect;
use rocket_dyn_templates::{Template};

// 具体逻辑：
// 1. 用户登录需要调用/login请求
// 2. 请求守卫会判断请求中的header
// 3. 若request header中admin为"true"时，跳转到admin否则为user
struct LoginGuard(bool);

///进行请求守卫逻辑编写
#[rocket::async_trait]
impl<'r> FromRequest<'r> for LoginGuard {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        //检查请求头
        let req = request.headers();
        match req.get_one("admin").unwrap() {
            "true" => Outcome::Success(LoginGuard(true)),
            _ => {
                Outcome::Success(LoginGuard(false))
            }
        }
    }
}

#[get("/login/<token>")]
fn login(token: String, guard: LoginGuard) -> Redirect {
    match guard.0 {
        true => { Redirect::to(uri!(admin())) }
        false => { Redirect::to(uri!(user())) }
    }
}

#[get("/admin")]
fn admin() -> Template {
    //admin模板页面
}

#[get("/user")]
fn user() -> Template {
    // user模板页面
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![login,admin,user])
}
