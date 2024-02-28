use lazy_static::lazy_static;
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
    //不建议,失去了静态单例的意义
    let other_db: Surreal<Client> = Surreal::init();
    DB.connect::<Ws>("127.0.0.1:10086").await?;
    other_db.connect::<Ws>("127.0.0.1:10086").await?;
    Ok(())
}
