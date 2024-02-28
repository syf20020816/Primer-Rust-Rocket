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
    //使用Define语句创建操作实体
    //操作实体包括：命名空间、数据库、数据表
    //首先创建命名空间
    let _ = db.query("DEFINE NS test;").await?;
    //选择操作实体
    //首先选择命名空间：test
    db.use_ns("test").await?;
    //然后创建数据库：test
    let _ = db.query("DEFINE DB test;").await?;
    db.use_db("test").await?;
    //定义使用的数据表
    let _ = db.query("DEFINE TABLE person;").await?;
    //查看person表的信息
    let table_info = db.query("INFO FOR TABLE person;").await?;
    println!("{:#?}", table_info);
    Ok(())
}
