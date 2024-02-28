// 引入 `lazy_static` 宏，这是一种 Rust 的常用库，用于创建静态生命周期的变量，这些变量只会被初始化一次。
use lazy_static::lazy_static;

// 引入项目内定义的模块，这可能包括认证、解析器和其他配置相关的结构体和枚举。
use surreal_use::config::{auth::Root, parser::Parsers, AuthBridger};
// 引入 `surrealdb` 库，这是一个数据库引擎，这里特别使用了其远程 WebSocket 客户端和核心 `Surreal` 结构体。
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

// 使用 `lazy_static!` 宏定义一个静态的、惰性初始化的 `SurrealDB` 客户端实例。
// 这意味着 `DB` 变量在程序运行期间只会被初始化一次，并且是在首次访问时初始化。
lazy_static! {
    pub static ref DB: Surreal<Client> = Surreal::init();
}

// 定义一个异步函数 `db_init`，用于初始化和配置数据库连接。
// 这个函数在成功时返回一个空的 `Ok` 结果，如果遇到错误则返回 `surrealdb::Result` 类型的错误。
pub async fn db_init() -> surrealdb::Result<()> {
    // 使用配置解析器从给定的源（这里没有指定，所以可能是默认值或环境变量）解析配置，并生成配置对象。
    let configuration = Parsers::Json.parse_to_config(None);
    // 从配置中获取数据库的 URL。
    let url = configuration.url();
    // 使用 WebSocket (`Ws`) 连接到数据库 URL。
    DB.connect::<Ws>(url).await?;
    // 将配置中的认证信息转换为 `Root` 类型的凭证。
    let credentail: Root = configuration.get_auth().into();
    // 使用凭证登录到数据库。
    let _ = DB.signin(credentail.to_lower_cast()).await?;
    // 设置使用的命名空间和数据库为 "todo"。
    let _ = DB.use_ns("todo").use_db("todo").await?;
    // 如果上述操作都成功，返回一个空的 `Ok(())` 表示初始化成功。
    Ok(())
}
