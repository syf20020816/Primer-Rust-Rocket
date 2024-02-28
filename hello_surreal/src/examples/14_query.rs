use lazy_static::lazy_static;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Root, Scope},
    Surreal,
};

//使用lazy static 宏
lazy_static! {
    static ref DB: Surreal<Client> = Surreal::init();
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<Ws>("127.0.0.1:10086").await?;
    //登录数据库
    let jwt = DB
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

    //使用命名空间和数据库
    DB.query("USE NS test;").await?;
    //链式调用
    DB.query("USE DB test").query("SELECT * FROM user").await?;
    Ok(())
}
