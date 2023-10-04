//第4章/main.rs
#[macro_use]
extern crate rocket;


// 这依然使用GET请求，但发起HEAD请求
#[get("/index/<say>")]
fn index(say: &str) -> String {
    format!("🙂{}", say)
}

/// 旧API
///```code
/// #[get("/old/1")]
/// fn old_api1() -> &'static str {
///     "Old API 1"
/// }
///
/// #[get("/old/2")]
/// fn old_api2() -> &'static str {
///     "Old API 2"
/// }
///
/// #[get("/old/3")]
/// fn old_api3() -> &'static str {
///     "Old API 3"
/// }
/// ```
#[get("/old/<_..>")]
fn drop_old_api() -> &'static str {
    "The old API has been abandoned"
}

/// 获取id但忽略后续其他参数
#[get("/user/<id>/<_>")]
fn easy_restful(id: &str) -> String {
    format!("User: id -> {}", id)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/apiV1_4", routes![index,easy_restful])
        .mount("/apiV1_0", routes![drop_old_api])
}
