use surrealdb::engine::remote::ws::Wss;
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

//使用tokio进行标记
//允许main函数变为异步函数
#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db_ws = Surreal::new::<Ws>("127.0.0.1:10086").await?;
    let db_wss = Surreal::new::<Wss>("127.0.0.1:10086").await?;
    Ok(())
}
