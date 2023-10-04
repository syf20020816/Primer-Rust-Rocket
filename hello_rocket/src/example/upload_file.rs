//第4章/main.rs
#[macro_use]
extern crate rocket;

use rocket::fs::TempFile;
use rocket::form::Form;
use std::time::{SystemTime, Duration, UNIX_EPOCH};


#[post("/upload", format = "multipart/form-data", data = "<img>")]
async fn upload_img(mut img: Form<TempFile<'_>>) -> std::io::Result<()> {
    // 获取文件扩展名
    let ext_name = img.content_type().unwrap().extension().unwrap();
    // 获取时间戳
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    // 格式化名字
    let name = format!("{}_{}.{}", img.name().unwrap(), timestamp, ext_name);
    // 格式化存储地址
    let path = format!("{}/{}", "tmp/imgs", &name);
    img.persist_to(path).await?;
    Ok(())
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![upload_img])
}
