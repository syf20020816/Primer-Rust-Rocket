use lazy_static::lazy_static;
use surreal_use::{
    config::{auth::Root, parser::Parsers, AuthBridger},
    core::Stmt,
};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

//使用lazy static 宏
lazy_static! {
    static ref DB: Surreal<Client> = Surreal::init();
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    //使用surreal_use获取项目包下的surrealdb.config.json的配置
    let config = Parsers::Json.parse_to_config(None);
    DB.connect::<Ws>(config.url()).await?;
    //转换为扩展后的凭证
    let credentail: Root = config.get_auth().into();
    //使用ROOT方式进行登录
    //返回Jwt结构体
    let _ = DB.signin(credentail.to_lower_cast()).await?;
    let _ = DB.use_ns("test").use_db("test").await?;
    let select = Stmt::select().table("user".into()).field_all().to_string();
    let query = DB.query(&select).await?;
    dbg!(query);
    Ok(())
}
