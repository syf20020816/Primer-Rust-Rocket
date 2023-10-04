//第3章/main.rs
#[macro_use] extern crate rocket;

// 这依然使用GET请求，但发起HEAD请求
#[get("/index")]
fn index()->&'static str{
    "🙂hello world"
}

#[launch]
fn rocket()->_{
    rocket::build().mount("/apiV1_4",routes![index])
}
