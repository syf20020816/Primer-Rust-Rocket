use crate::lib::{
    entry::po::{self, Annex, Date, ITagProps, Priorities, Priority, Status},
    mapping::{select_todo_record, select_user_by_username},
};
use rocket::serde::{Deserialize, Serialize};

use super::User;

// 实现序列化与反序列化
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    // TODO的ID
    id: String,
    // 名称
    pub name: String,
    // 优先级
    pub priority: Priorities,
    /// 审核人
    pub reviewers: Vec<User>,
    // 执行人
    pub performers: Vec<User>,
    // 日期
    pub date: Date,
    // 标签
    pub tags: Vec<ITagProps>,
    // TODO状态
    pub status: Status,
    // 概述说明
    pub description: Option<String>,
    // 详细信息
    pub information: Option<String>,
    /// 附件
    pub annexs: Option<Vec<Annex>>,
    #[serde(rename(serialize = "isFocus"))]
    #[serde(rename(deserialize = "isFocus"))]
    pub is_focus: bool,
}

impl Todo {
    pub fn id(&self) -> &str {
        &self.id
    }
    // 判断是否有审核人
    pub fn have_reviewers(&self) -> bool {
        !self.reviewers.is_empty()
    }
    // 判断是否有执行人
    pub fn have_performers(&self) -> bool {
        !self.performers.is_empty()
    }
    //判断是否是自己的TODO
    pub fn is_self_todo(&self) -> bool {
        !self.have_reviewers() && !self.have_performers()
    }
    // 判断是否是团队的TODO
    pub fn is_team_todo(&self) -> bool {
        !self.is_self_todo()
    }
    pub fn priority(&self) -> Priorities {
        self.priority.clone()
    }
    // 从dto::Todo结构体转为vo的Todo结构体
    pub async fn from(value: po::Todo, id: String) -> Self {
        let reviewers = value.reviewers;
        let performers = value.performers;
        //为执行人和审核人的结构体进行转换
        let reviewers = convert_usernames_to_user_instances(reviewers).await;
        let performers = convert_usernames_to_user_instances(performers).await;
        // let result = join(reviewers_f, performer_f);
        // let users = block_on(result);
        let reviewers = reviewers
            .into_iter()
            .map(|item| User::easy_from(item))
            .collect::<Vec<User>>();
        let performers = performers
            .into_iter()
            .map(|item| User::easy_from(item))
            .collect::<Vec<User>>();
        Todo {
            id,
            name: value.name,
            priority: value.priority,
            reviewers,
            performers,
            date: value.date,
            tags: value.tags,
            status: value.status,
            description: value.description,
            information: value.information,
            annexs: value.annexs,
            is_focus: value.is_focus,
        }
    }
}

// 转换用户ID集合为vo::User用户实例
async fn convert_usernames_to_user_instances(usernames: Vec<String>) -> Vec<po::User> {
    let mut res = Vec::new();
    for username in usernames {
        let query = select_user_by_username(&username).await;
        if let Some(user) = query {
            res.push(user);
        }
    }
    res
}

impl Todo {
    fn select_reviewers(&self) -> () {}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoBox {
    pub low: Vec<Todo>,
    pub mid: Vec<Todo>,
    pub fatal: Vec<Todo>,
    //关注
    pub focus: Vec<Todo>,
    pub history: Vec<Todo>,
}

impl TodoBox {
    // 向TodoBox中添加Todo结构体
    pub fn push(&mut self, todo: Todo) -> () {
        let priority = todo.priority();
        let is_focus = todo.is_focus;
        if is_focus {
            self.focus.push(todo.clone());
        }
        match priority {
            Priorities::Emergent | Priorities::High => self.fatal.push(todo),
            Priorities::Mid => self.mid.push(todo),
            Priorities::Low => self.low.push(todo),
        };
    }
    // 将dto的TodoBox转换为vo的TodoBox实例
    pub async fn from(value: po::TodoBox) -> Self {
        let po::TodoBox {
            low,
            mid,
            fatal,
            focus,
            history,
        } = value;
        let mut low = convert_ids_to_todo_instances(low).await;
        let mut mid = convert_ids_to_todo_instances(mid).await;
        let mut fatal = convert_ids_to_todo_instances(fatal).await;
        let history = convert_ids_to_todo_instances(history).await;
        let mut focus = Vec::new();
        focus.append(&mut low.0);
        focus.append(&mut mid.0);
        focus.append(&mut fatal.0);

        TodoBox {
            low: low.1,
            mid: mid.1,
            fatal: fatal.1,
            focus,
            history: history.1,
        }
    }
}

// 将TODOBox中的所有ID的集合转为TODO实例
pub async fn convert_ids_to_todo_instances(ids: Vec<String>) -> (Vec<Todo>, Vec<Todo>) {
    if ids.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut res = Vec::new();
    let mut focus = Vec::new();
    for id in ids {
        let query = select_todo_record(id.as_str()).await;
        if let Some((id, todo)) = query {
            let todo = Todo::from(todo, id).await;
            if (todo.is_focus) {
                focus.push(todo.clone())
            }
            res.push(todo);
        }
    }
    (focus, res)
}
