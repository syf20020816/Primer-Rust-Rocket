use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Root, Scope},
    sql::Thing,
    Surreal,
};

//使用lazy static 宏
lazy_static! {
    static ref DB: Surreal<Client> = Surreal::init();
}

//设置一个对应Scope的结构体ScopeCredential
//对应Scope中的params字段
//由于Scope实现了serde的Serialize和Debug trait，所以自己编写的ScopeCredential也需要去实现
#[derive(Serialize, Debug)]
struct ScopeCredential<'a> {
    email: &'a str,
    pass: &'a str,
    name: &'a str,
}

// SurrealDB返回响应的结构体
// 其中Thing包含了返回的数据表的名称和记录的ID
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
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
    DB.use_ns("test").use_db("test").await?;
    //使用as_insecure_token方法获取到jwt令牌进行验证
    DB.authenticate(jwt.as_insecure_token()).await?;
    //使得当前的连接无效
    DB.invalidate().await?;
    //再次验证,查询结果为空
    //不使用invalidate方法时
    //     [src\main.rs:52] r = [
    //     Record {
    //         id: Thing {
    //             tb: "scopeCredential",
    //             id: String(
    //                 "6lusc31jbamwmikap713",
    //             ),
    //         },
    //     },
    // ]
    let r: Vec<Record> = DB.select("scopeCredential").await?;
    dbg!(r);
    Ok(())
}
