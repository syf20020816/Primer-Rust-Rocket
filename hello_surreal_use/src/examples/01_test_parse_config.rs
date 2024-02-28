use lazy_static::lazy_static;
use surreal_use::config::{auth::Root, parser::Parsers, AuthBridger};
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
    let jwt_root = DB.signin(credentail.to_lower_cast()).await?;
    //使用as_insecure_token方法获取到jwt令牌
    dbg!(jwt_root.as_insecure_token());
    Ok(())
}
