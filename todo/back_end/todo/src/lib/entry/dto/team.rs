use crate::lib::{
    entry::po::{self, TeamAvatars}, // 引入dto模块下的结构和枚举，用于处理与数据传输相关的对象
    mapping::select_todo_record,     // 引入select_todo_record函数，用于查询待办事项记录
};

use super::{Todo, User};

use rocket::serde::{Deserialize, Serialize};

// 定义Team结构体，并为其字段实现序列化和反序列化功能
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    pub id: String,          // 团队唯一标识符
    pub name: String,        // 团队名称
    pub members: Vec<User>,  // 团队成员列表
    pub owner: String,       // 团队拥有者标识符
    pub avatar: TeamAvatars, // 团队头像
    pub description: String, // 团队描述
    pub date: String,        // 创建日期
    pub todos: Vec<Todo>,    // 关联的待办事项列表
}

impl Team {
    // 设置团队ID的方法
    pub fn set_id(&mut self, id: &str) -> () {
        self.id = String::from(id);
    }
    // 设置团队成员的方法
    pub fn set_members(&mut self, members: Vec<User>) {
        self.members = members;
    }
    // 从dto::Team类型创建Team实例的异步方法
    pub async fn from(value: po::Team) -> Self {
        let mut todos = Vec::new();

        for todo_id in value.todos {
            // 异步查询待办事项记录
            let query = select_todo_record(&todo_id).await;
            if let Some((id, todo)) = query {
                let todo = Todo::from(todo, id).await;
                // 将待办事项添加到列表中
                todos.push(todo);
            }
        }
        // 返回构造的Team实例
        Team {
            id: String::new(),
            name: value.name,
            members: Vec::new(),
            owner: value.owner,
            avatar: value.avatar,
            description: value.description,
            date: value.date,
            todos,
        }
    }
}
