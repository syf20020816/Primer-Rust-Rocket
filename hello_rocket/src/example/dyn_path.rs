//第4章/main.rs
#[macro_use]
extern crate rocket;

// 使用`{}`是错误的❌
// #[get("/index/{say}")]
// 在Rocket中应该使用`<>`
#[get("/index/<say>")]
fn index(say: &str) -> String {
    format!("🙂{}", say)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/apiV1_4", routes![index])
}
