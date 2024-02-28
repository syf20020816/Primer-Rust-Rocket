use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

//使用tokio进行标记
//允许main函数变为异步函数
#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    //使用WebSocket协议连接SurrealDB数据库
    //设置连接地址为127.0.0.1:10086
    let db = Surreal::new::<Ws>("127.0.0.1:10086").await?;
    //登录SurrealDB数据库
    //使用账号：root
    //使用密码：root
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    //获得SurrealDB数据库的版本信息
    let version = db.version().await?;
    println!("SurrealDB Version:\n{}", version.to_string());
    Ok(())
}
