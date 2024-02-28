// 引入系统时间和UNIX纪元时间的模块。
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

// 引入项目内部的其他模块或结构。
use crate::lib::entry::dto;
use crate::lib::mapping::select_team_record_by_id;

use super::TeamAvatars;

// 引入Rocket框架的序列化和反序列化特性。
use rocket::serde::{Deserialize, Serialize};

/// 定义`Team`结构体，用于表示一个团队的信息。
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Team {
    // 团队名称
    pub name: String,
    // 团队成员ID集合
    pub members: Vec<String>,
    // 团队的创建人
    pub owner: String,
    // 团队头像
    pub avatar: TeamAvatars,
    // 团队概述说明
    pub description: String,
    // 团队创建日期
    pub date: String,
    // 团队有的TODO的ID集合
    pub todos: Vec<String>,
}

impl Team {
    // 获取成员列表的克隆。
    pub fn members(&self) -> Vec<String> {
        self.members.clone()
    }

    // 向成员列表中添加一个新成员。
    pub fn add_member(&mut self, member: &str) {
        self.members.push(member.to_string());
    }

    // 创建一个具有随机日期的新`Team`实例。
    pub fn new_rand(name: &str, owner: &str) -> Self {
        let date = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Team {
            name: name.to_string(),
            members: vec![owner.to_string()],
            owner: owner.to_string(),
            avatar: TeamAvatars::Team1,
            description: String::new(),
            date: date.to_string(),
            todos: Vec::new(),
        }
    }

    // 异步函数，用于从数据库中获取特定ID的团队记录。
    pub async fn get(id: &str) -> Option<(String, Team)> {
        select_team_record_by_id(id).await
    }

    // 向待办事项列表中添加一个新的待办事项ID。
    pub fn push_todo(&mut self, todo_id: &str) {
        self.todos.push(todo_id.to_string());
    }
}

impl Default for Team {
    // 提供`Team`的默认实例。
    fn default() -> Self {
        Self {
            name: String::new(),
            members: Default::default(),
            owner: Default::default(),
            avatar: Default::default(),
            description: String::new(),
            date: Default::default(),
            todos: Vec::new(),
        }
    }
}

impl From<dto::Team> for Team {
    // 从另一个类型（`dto::Team`）转换为`Team`类型的实例。
    fn from(value: dto::Team) -> Self {
        // 将User结构体集合转换为User的ID集合
        let members = value
            .members
            .into_iter()
            .map(|member| member.username().to_string())
            .collect::<Vec<String>>();
        // 将todo的TODO结构体集合转换为ID集合
        let todos = value
            .todos
            .into_iter()
            .map(|todo| todo.id().to_string())
            .collect::<Vec<String>>();
        Team {
            name: value.name,
            members,
            owner: value.owner,
            avatar: value.avatar,
            description: value.description,
            date: value.date,
            todos,
        }
    }
}
