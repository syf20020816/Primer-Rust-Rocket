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
    // 这里是准备工作
    // 首先使用ROOT根用户登录到数据库然后创建一个Scope作用域
    // DB.signin(Root {
    //     username: "root",
    //     password: "root",
    // })
    // .await?;
    //使用test命名空间和test数据库
    // DB.use_ns("test").use_db("test").await?;
    //使用query()方法定义test_sc这个作用域设置时效为24小时
    //设置登录鉴权方式使用scopeCredential这个表
    //而scopeCredential表的实际字段则和上方的scopeCredential结构体相同
    // let scope_res = DB.query("DEFINE SCOPE test_sc SESSION 24h SIGNUP ( CREATE scopeCredential SET email ='test001', pass = 'test001' , name = 'test001' ) SIGNIN ( SELECT * FROM scopeCredential WHERE email = 'test001' AND pass = 'test001' AND name = 'test001' );").await?;
    // dbg!(scope_res);
    //成功后会打印如下响应
    //[src\main.rs:34] scope_res = Response(
    //     {
    //         0: Ok(
    //             [],
    //         ),
    //     },
    // )
    //使用signup方法进行注册到test_sc这个作用域
    //params字段则是scopeCredential这个结构体
    let jwt = DB
        .signup(Scope {
            namespace: "test",
            database: "test",
            scope: "test_sc",
            params: ScopeCredential {
                email: "Matt@gmail.com",
                pass: "Matt001",
                name: "Matt",
            },
        })
        .await?;
    //使用as_insecure_token方法获取到jwt令牌
    dbg!(jwt.as_insecure_token());
    Ok(())
}
