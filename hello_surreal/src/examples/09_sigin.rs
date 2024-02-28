use lazy_static::lazy_static;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

//使用lazy static 宏
lazy_static! {
    static ref DB: Surreal<Client> = Surreal::init();
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<Ws>("127.0.0.1:10086").await?;
    //使用ROOT方式进行登录
    //返回Jwt结构体
    let jwt_root = DB
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
    //使用as_insecure_token方法获取到jwt令牌
    dbg!(jwt_root.as_insecure_token());
    Ok(())
}
