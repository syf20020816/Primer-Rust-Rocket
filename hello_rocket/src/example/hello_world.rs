//第2章/main.rs
//导入外部crate
#[macro_use] extern crate rocket;

//编写API
#[get("/index")]
fn index()->&'static str{
    "🙂hello world"
}

//主函数入口
#[launch]
fn rocket()->_{
    //启动程序并绑定API路由
    rocket::build().mount("/apiV1_4",routes![index])
}
