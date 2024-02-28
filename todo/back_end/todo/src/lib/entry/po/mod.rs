// 声明模块，每个模块负责不同的功能区域
mod annex; // 附件模块，可能用于处理文件附件相关功能
mod avatar; // 头像模块，定义用户和团队的头像处理功能
mod date; // 日期模块，提供日期相关的功能
mod login; // 登录模块，处理登录和注册相关的功能
mod priority; // 优先级模块，定义任务优先级相关功能
mod status; // 状态模块，用于定义任务或其他实体的状态
mod tag; // 标签模块，提供标签相关的功能，如分类和标记
mod team; // 团队模块，处理团队创建、管理等功能
mod todo; // 待办事项模块，核心功能模块，处理待办事项的创建、更新等
mod user; // 用户模块，定义用户相关的数据和操作

// 将各个模块中定义的公共结构体或枚举等公开，以便于外部调用
pub use annex::Annex; // 公开附件模块中的Annex结构体
pub use avatar::{Avatars, TeamAvatars}; // 公开头像模块中的Avatars和TeamAvatars结构体
pub use date::Date; // 公开日期模块中的Date结构体
pub use login::{Signin, Signup}; // 公开登录模块中的Signin和Signup结构体
pub use priority::{Priorities, Priority}; // 公开优先级模块中的Priorities枚举和Priority结构体
pub use status::Status; // 公开状态模块中的Status枚举
pub use tag::{ITagProps, Tags}; // 公开标签模块中的ITagProps接口和Tags枚举
pub use team::Team; // 公开团队模块中的Team结构体
pub use todo::{Todo, TodoBox}; // 公开待办事项模块中的Todo结构体和TodoBox容器
pub use user::User; // 公开用户模块中的User结构体
