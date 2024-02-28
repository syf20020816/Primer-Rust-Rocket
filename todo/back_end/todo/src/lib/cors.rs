// 引入 `rocket_cors` 库中相关的模块和类型
use rocket_cors::{AllowedMethods, AllowedOrigins, Cors, CorsOptions, Method};
// 以及标准库中的 `HashSet` 和 `FromStr` 用于处理集合和字符串转换。
use std::{collections::HashSet, str::FromStr};

// 定义一个初始化 CORS 设置的函数，该函数返回 `Result` 类型，成功时包含配置好的 `Cors` 对象，失败时包含 `rocket_cors::Error` 错误。
pub fn init_cors() -> Result<Cors, rocket_cors::Error> {
    // 定义允许的来源列表。`AllowedOrigins::some_exact` 方法用于指定一个精确匹配的来源列表。
    // 这里列出了本地开发常用的端口号，允许这些来源的请求通过 CORS 验证。
    // 保证127.0.0.1和locahost端口的一致性
    // 4000是vite使用npx常用启动端口
    // 5173是vite在常规方式npm run dev时的启动端口
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:4000",
        "http://localhost:5173",
        "http://127.0.0.1:4000",
        "http://127.0.0.1:5173",
    ]);

    // 定义允许的 HTTP 方法。这里通过映射一个字符串列表到 `Method` 枚举，允许 "Get", "Post", "Delete", "Put" 方法。
    // 使用 `HashSet` 是为了确保方法的唯一性，避免重复。
    let allowed_methods: AllowedMethods = vec!["Get", "Post", "Delete", "Put"]
        .into_iter()
        .map(|s| Method::from_str(s).unwrap()) // 将字符串转换为 `Method` 枚举，`unwrap` 用于处理转换可能失败的情况（这里假设不会失败）。
        .collect::<HashSet<Method>>(); // 收集转换结果到一个 `HashSet` 中。

    // 使用默认的 CORS 配置作为基础，通过链式调用方法来自定义配置。
    // `.allowed_methods(allowed_methods)` 设置允许的 HTTP 方法。
    // `.allowed_origins(allowed_origins)` 设置允许的来源。
    // `.allow_credentials(true)` 允许跨域请求携带凭证（如 Cookies）。
    let cors_options = CorsOptions::default()
        .allowed_methods(allowed_methods)
        .allowed_origins(allowed_origins)
        .allow_credentials(true);

    // 通过上面定义的配置选项创建一个 `Cors` 实例。
    let cors = Cors::from_options(&cors_options);
    cors // 返回配置好的 `Cors` 对象。
}
