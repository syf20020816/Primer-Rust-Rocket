use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

/// 职业的枚举
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Jobs {
    Worker,
    Teacher,
    Doctor,
}

// person结构体对应person表的存储字段
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    job: Jobs,
}

// 实现new方法来快速创建一个Person结构体
impl Person {
    pub fn new(name: &str, age: u8, job: Jobs) -> Self {
        Person {
            name: String::from(name),
            age,
            job,
        }
    }
}

// SurrealDB返回响应的结构体
// 其中Thing包含了返回的数据表的名称和记录的ID
#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

// 定义一个PersonDTO作为Person的数据传输对象
// 增加id字段来得到每条记录的ID
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PersonDTO {
    id: Thing,
    name: String,
    age: u8,
    job: Jobs,
}

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
    //使用test命名空间，使用test数据库
    db.use_ns("test").use_db("test").await?;
    let _create_res: Option<Record> = db
        .create(("person", "002"))
        .content(Person::new("Jany", 32, Jobs::Worker))
        .await?;
    //更新新增的id为002的数据
    let update_res: Option<Record> = db
        .update(("person", "002"))
        .content(Person::new("Jany", 66, Jobs::Doctor))
        .await?;
    dbg!(update_res);
    //查询所有数据
    //通过PersonDTO进行单条接收
    let select_res: Option<PersonDTO> = db.select(("person", "002")).await?;
    dbg!(select_res);
    Ok(())
}
