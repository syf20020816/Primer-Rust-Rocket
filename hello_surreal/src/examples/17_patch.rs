use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::{
        auth::{Root, Scope},
        PatchOp,
    },
    Surreal,
};

//使用lazy static 宏
lazy_static! {
    static ref DB: Surreal<Client> = Surreal::init();
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: &str, age: u8) -> Self {
        User {
            name: String::from(name),
            age,
        }
    }
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
    //使用patch修改数据
    let res: Option<User> = DB
        .update(("user", "1"))
        .patch(PatchOp::replace("/name", "Jarry")) //替换name为Jarry
        .patch(PatchOp::add("/age", 5)) //由于age不是迭代器形式，无法使用add附加，会替代
        .await?;
    dbg!(res);
    Ok(())
}
