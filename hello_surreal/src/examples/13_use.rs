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

struct MyNS<'a> {
    ns: &'a str,
}

impl<'a> From<MyNS<'a>> for String {
    fn from(value: MyNS<'a>) -> Self {
        format!("surrealdb-{}", value.ns)
    }
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
    DB.use_ns(MyNS { ns: "test" }).use_db("test").await?;
    Ok(())
}
