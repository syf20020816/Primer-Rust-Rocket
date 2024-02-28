// 导入自定义模块
mod lib;

// 引入 rocket 框架宏，用于简化开发
#[macro_use]
extern crate rocket;

// 使用 lib 模块中定义的 API 函数
use lib::api::{
    complete_todo, create_team, create_team_todo, create_todo, delete_todo, failed_todo,
    get_user_info, set_user_avatar, set_user_setting, signin, signup, update_team_info,
    update_team_members, update_todo, update_todo_status,
};

// 导入 CORS 相关配置初始化函数
use lib::cors::init_cors;

// 导入数据库初始化函数
use lib::db::db_init;

// 导入自定义异常处理函数
use lib::response::define_excp_handler;

// 应用的入口点，使用 `launch` 属性标记。这个函数将在应用启动时被调用。
#[launch]
async fn rocket() -> _ {
    // 初始化数据库连接
    let _ = db_init().await;

    // 初始化 CORS 配置
    let cors = init_cors().expect("CORS Configuration Correct");

    // 构建 Rocket 应用实例
    rocket::build()
        // 应用 CORS
        .attach(cors)
        // 配置路由和路径
        .mount(
            "/api/v1/user", // 用户相关的 API 路径
            routes![
                // 绑定路径和处理函数
                signin,           // 登录
                signup,           // 注册
                get_user_info,    // 获取用户信息
                set_user_setting, // 设置用户信息
                set_user_avatar   // 设置用户头像
            ],
        )
        .mount(
            "/api/v1/todo", // 待办事项相关的 API 路径
            routes![
                // 绑定路径和处理函数
                create_todo,        // 创建待办事项
                delete_todo,        // 删除待办事项
                update_todo,        // 更新待办事项信息
                update_todo_status, // 更新待办事项状态
                complete_todo,      // 完成待办事项
                failed_todo         // 待办事项失败
            ],
        )
        .mount(
            "/api/v1/team", // 团队相关的 API 路径
            routes![
                // 绑定路径和处理函数
                create_team,         // 创建团队
                update_team_members, // 更新团队成员
                update_team_info,    // 更新团队信息
                create_team_todo     // 为团队创建待办事项
            ],
        )
        // 注册全局异常处理器
        .register("/api/v1", catchers![define_excp_handler])
}
