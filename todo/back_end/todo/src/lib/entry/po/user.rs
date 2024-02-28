use crate::lib::entry::dto;

use super::Avatars;
use super::Priorities;
use super::Team;
use super::Todo;
use super::TodoBox;
use rocket::serde::{self, Deserialize, Serialize};

/// 定义一个用户结构体，使用Rocket框架的serde进行序列化和反序列化
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct User {
    // 用户名
    pub username: String,
    // 用户的真实姓名
    pub name: String,
    // 密码字段，在序列化时会被忽略，保护用户隐私
    pub password: String,
    // 用户头像
    pub avatar: Avatars,
    // 用户邮箱
    pub email: String,
    #[serde(rename(serialize = "teamNumber"))]
    #[serde(rename(deserialize = "teamNumber"))]
    pub team_number: u8, // 用户所属团队数量
    #[serde(rename(serialize = "todoNumber"))]
    #[serde(rename(deserialize = "todoNumber"))]
    pub todo_number: u16, // 用户待处理的Todo数量
    #[serde(rename(serialize = "totalTodo"))]
    #[serde(rename(deserialize = "totalTodo"))]
    pub total_todo: u16, // 用户总Todo数量
    pub todos: TodoBox,             // 用户的Todo箱
    pub teams: Option<Vec<String>>, // 用户所属的团队ID列表
    // 是否发送邮件提醒
    #[serde(rename(serialize = "sendEmail"))]
    #[serde(rename(deserialize = "sendEmail"))]
    pub send_email: bool,
    // 是否发送消息提醒
    #[serde(rename(serialize = "sendMsg"))]
    #[serde(rename(deserialize = "sendMsg"))]
    pub send_msg: bool,
}

impl User {
    /// 创建一个新的User实例
    pub fn new() -> User {
        User::default()
    }

    /// 获取用户名
    pub fn username(&self) -> &str {
        &self.username
    }

    /// 快速初始化用户信息
    pub fn quick_init(name: &str, username: &str, password: &str, email: &str) -> Self {
        User {
            username: username.to_string(),
            name: name.to_string(),
            password: password.to_string(),
            avatar: Avatars::default(),
            email: email.to_string(),
            team_number: 0,
            todo_number: 0,
            total_todo: 0,
            todos: TodoBox::default(),
            teams: None,
            send_email: false,
            send_msg: true,
        }
    }

    /// 将密码字段置为空，用于在不需要密码的场合保护用户隐私
    pub fn skip_pwd(&mut self) {
        self.password = String::new();
    }

    /// 添加一个Todo项
    pub fn add_todo(&mut self, todo_id: String, priority: Priorities) {
        self.todo_number += 1;
        self.total_todo += 1;
        match priority {
            Priorities::Emergent | Priorities::High => self.todos.fatal.push(todo_id),
            Priorities::Mid => self.todos.mid.push(todo_id),
            Priorities::Low => self.todos.low.push(todo_id),
        };
    }

    /// 删除一个Todo项
    pub fn delete_todo(&mut self, id: &str) {
        self.todo_number -= 1;
        self.total_todo -= 1;
        self.todos.remove(id);
    }

    /// 完成一个Todo项，将其从待办事项中移除，并添加到历史记录中
    pub fn complete_todo(&mut self, id: &str) {
        self.todo_number -= 1;
        let _ = self.todos.remove(id);
        self.todos.history.push(id.to_string());
    }

    /// 将一个Todo项标记为失败，实际上调用complete_todo方法处理
    pub fn failed_todo(&mut self, id: &str) {
        dbg!(id);
        self.complete_todo(id);
    }

    /// 创建一个新的团队
    pub fn create_team(&mut self, id: &str) {
        match &mut self.teams {
            Some(teams) => teams.push(id.to_string()),
            None => {
                let _ = self.teams.replace(vec![id.to_string()]);
            }
        };
        self.team_number += 1;
    }
}

impl Default for User {
    /// 提供User的默认实现，用于快速创建一个带有默认值的User实例
    fn default() -> Self {
        Self {
            username: Default::default(),
            name: Default::default(),
            password: Default::default(),
            avatar: Default::default(),
            email: Default::default(),
            team_number: 0,
            todo_number: 0,
            total_todo: 0,
            todos: Default::default(),
            teams: None,
            send_email: false,
            send_msg: true,
        }
    }
}
