use lazy_static::lazy_static;
use serde::Serialize;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Root, Scope},
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

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<Ws>("127.0.0.1:10086").await?;
    //登录数据库
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    //使用命名空间和数据库
    DB.use_ns("test").use_db("test").await?;
    // let res = DB.authenticate("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJpYXQiOjE3MDUxMzg0OTcsIm5iZiI6MTcwNTEzODQ5NywiZXhwIjoxNzA1MjI0ODk3LCJpc3MiOiJTdXJyZWFsREIiLCJOUyI6InRlc3QiLCJEQiI6InRlc3QiLCJTQyI6InRlc3Rfc2MiLCJJRCI6InNjb3BlQ3JlZGVudGlhbDo2bHVzYzMxamJhbXdtaWthcDcxMyJ9.tUjS0LNkM5OfmDzKjyP1UCG0LT_YVyC12dGEcVWVuKLxlFggqmDi7G0rK4WZTQCfFaoUFTwTI45O6ZT6AqfAIg").await;
    let res = DB.authenticate("token").await;
    dbg!(res);
    Ok(())
}
