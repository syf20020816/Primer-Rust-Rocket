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
    let _jwt = DB
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

    //使用命名空间和数据库
    DB.use_ns("test").use_db("test").await?;
    //设置变量
    DB.set("target_user", "Matt").await?;
    //使用变量进行查询
    DB.query("SELECT * FROM user WHERE username = $target_user;")
        .await?;
    Ok(())
}
