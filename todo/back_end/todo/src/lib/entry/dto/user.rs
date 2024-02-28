use crate::lib::{
    entry::po::{self, Avatars}, // 引入dto模块，包含用于数据传输的结构体和枚举
    mapping::{select_team_record_by_id, select_user_by_username}, // 引入查询团队记录和用户的函数
};
use rocket::serde::{Deserialize, Serialize}; // 引入用于序列化和反序列化的特性

use super::{todo::TodoBox, Team, Todo}; // 引入同级模块中定义的结构体

// 定义用户个人设置的结构体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserPersonalSetting {
    name: String,  // 用户名
    email: String, // 邮箱
    #[serde(rename(serialize = "sendEmail"))] // 定义序列化时的字段名
    #[serde(rename(deserialize = "sendEmail"))] // 定义反序列化时的字段名
    send_email: bool, // 是否发送邮件
    #[serde(rename(serialize = "sendMsg"))] // 定义序列化时的字段名
    #[serde(rename(deserialize = "sendMsg"))] // 定义反序列化时的字段名
    send_msg: bool, // 是否发送消息
}

impl UserPersonalSetting {
    // Getter 方法
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn send_email(&self) -> bool {
        self.send_email
    }
    pub fn send_msg(&self) -> bool {
        self.send_msg
    }
}

// 定义用户的结构体
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    username: String, // 用户名
    name: String,     // 名称
    avatar: Avatars,  // 头像
    email: String,    // 邮箱
    #[serde(rename(serialize = "teamNumber"))] // 定义序列化时的字段名
    #[serde(rename(deserialize = "teamNumber"))] // 定义反序列化时的字段名
    team_number: u8, // 团队数量
    #[serde(rename(serialize = "todoNumber"))] // 定义序列化时的字段名
    #[serde(rename(deserialize = "todoNumber"))] // 定义反序列化时的字段名
    todo_number: u16, // 待办事项数量
    #[serde(rename(serialize = "totalTodo"))] // 定义序列化时的字段名
    #[serde(rename(deserialize = "totalTodo"))] // 定义反序列化时的字段名
    total_todo: u16, // 总待办事项数量
    todos: Option<TodoBox>, // 待办事项盒子（可能为空）
    teams: Option<Vec<Team>>, // 团队列表（可能为空）
    #[serde(rename(serialize = "sendEmail"))] // 定义序列化时的字段名
    #[serde(rename(deserialize = "sendEmail"))] // 定义反序列化时的字段名
    send_email: bool, // 是否发送邮件
    #[serde(rename(serialize = "sendMsg"))] // 定义序列化时的字段名
    #[serde(rename(deserialize = "sendMsg"))] // 定义反序列化时的字段名
    send_msg: bool, // 是否发送消息
}

impl User {
    // Getter 方法
    pub fn username(&self) -> &str {
        &self.username
    }
    // 快速从dto::User创建User实例的方法
    // 其中todos和teams在有些时候是并不需要的,全部转换会导致大量的性能开销
    pub fn easy_from(user: po::User) -> Self {
        User {
            username: user.username,
            name: user.name,
            avatar: user.avatar,
            email: user.email,
            team_number: user.team_number,
            todo_number: user.todo_number,
            total_todo: user.total_todo,
            todos: None, // 初始化为None，表示没有待办事项
            teams: None, // 初始化为None，表示没有团队
            send_email: user.send_email,
            send_msg: user.send_msg,
        }
    }

    // 从dto::User异步创建User实例的方法，包括加载相关的待办事项和团队信息
    pub async fn from(value: po::User) -> Self {
        let todos = TodoBox::from(value.todos).await; // 异步加载待办事项

        let teams = match value.teams {
            Some(teams) => {
                let mut team_vos = Vec::new();
                for team_id in teams {
                    let (id, team) = select_team_record_by_id(&team_id).await.unwrap(); // 异步查询团队记录
                    let members = team.members(); // 获取团队成员
                    let mut team = Team::from(team).await;
                    let _ = team.set_id(&id); // 设置团队ID
                    let mut convert_members = Vec::new();
                    for member in members {
                        let user = select_user_by_username(&member).await.unwrap(); // 异步查询用户信息
                        let user = User::easy_from(user); // 创建User实例
                        convert_members.push(user); // 添加到团队成员列表
                    }

                    let _ = team.set_members(convert_members); // 设置团队成员
                    team_vos.push(team); // 添加到团队列表
                }
                Some(team_vos) // 返回包含团队信息的Option
            }
            None => None, // 如果没有团队信息，返回None
        };

        User {
            username: value.username,
            name: value.name,
            avatar: value.avatar,
            email: value.email,
            team_number: value.team_number,
            todo_number: value.todo_number,
            total_todo: value.total_todo,
            todos: Some(todos), // 设置待办事项
            teams,              // 设置团队信息
            send_email: value.send_email,
            send_msg: value.send_msg,
        }
    }
}
