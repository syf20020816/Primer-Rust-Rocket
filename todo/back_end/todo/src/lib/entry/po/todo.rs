// 引入当前crate中其他模块或结构体的路径
use crate::lib::entry::dto;

// 引入同级或父级目录中的模块或结构体
use super::{Annex, Date, ITagProps, Priorities, Priority, Status, User};
// 引入rocket框架的序列化和反序列化特性
use rocket::serde::{Deserialize, Serialize};

// 定义Todo结构体并为其字段实现序列化和反序列化特性
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Todo {
    // 任务拥有者
    pub owner: String,
    // 任务名称
    pub name: String,
    // 任务优先级
    pub priority: Priorities,
    /// 审核人列表
    pub reviewers: Vec<String>,
    // 执行者列表
    pub performers: Vec<String>,
    // 任务日期
    pub date: Date,
    // 任务标签列表
    pub tags: Vec<ITagProps>,
    // 任务状态
    pub status: Status,
    // 任务描述（可选）
    pub description: Option<String>,
    // 附加信息（可选）
    pub information: Option<String>,
    /// 附件列表（可选）
    pub annexs: Option<Vec<Annex>>,
    // 标记任务是否为焦点任务
    #[serde(rename(serialize = "isFocus"))]
    #[serde(rename(deserialize = "isFocus"))]
    pub is_focus: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            name: Default::default(),
            priority: Default::default(),
            reviewers: Default::default(),
            performers: Default::default(),
            date: Default::default(),
            tags: Default::default(),
            status: Default::default(),
            description: None,
            information: None,
            annexs: None,
            is_focus: false,
            owner: "".to_string(),
        }
    }
}

impl Todo {
    // 设置任务拥有者
    pub fn set_owner(&mut self, id: String) {
        self.owner = id;
    }

    // 检查是否有审核人
    pub fn have_reviewers(&self) -> bool {
        !self.reviewers.is_empty()
    }

    // 检查是否有执行者
    pub fn have_performers(&self) -> bool {
        !self.performers.is_empty()
    }

    // 判断任务是否为个人任务（无审核人和执行者）
    pub fn is_self_todo(&self) -> bool {
        !self.have_reviewers() && !self.have_performers()
    }

    // 判断任务是否为团队任务
    pub fn is_team_todo(&self) -> bool {
        !self.is_self_todo()
    }

    // 获取任务优先级
    pub fn priority(&self) -> Priorities {
        self.priority.clone()
    }

    // 从给定的值创建一个Todo实例
    pub fn from(value: dto::Todo, owner: &str) -> (String, Self) {
        let id = value.id().to_string();
        let reviewers = value
            .reviewers
            .into_iter()
            .map(|x| x.username().to_string())
            .collect::<Vec<String>>();
        let performers = value
            .performers
            .into_iter()
            .map(|x| x.username().to_string())
            .collect::<Vec<String>>();

        let todo = Todo {
            owner: owner.to_string(),
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
        };

        (id, todo)
    }
}

// 定义一个结构体用于管理不同优先级的任务ID集合
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoBox {
    // 低优先级任务ID集合
    pub low: Vec<String>,
    // 中优先级任务ID集合
    pub mid: Vec<String>,
    // 高优先级任务ID集合
    pub fatal: Vec<String>,
    // 关注的任务ID集合
    pub focus: Vec<String>,
    // 历史任务ID集合
    pub history: Vec<String>,
}

impl Default for TodoBox {
    fn default() -> Self {
        Self {
            low: Default::default(),
            mid: Default::default(),
            fatal: Default::default(),
            focus: Default::default(),
            history: Default::default(),
        }
    }
}

impl TodoBox {
    // 从各优先级集合中移除指定的任务ID
    pub fn remove(&mut self, id: &str) {
        let TodoBox {
            low,
            mid,
            fatal,
            focus,
            history,
        } = self;
        // 对low, mid, fatal, focus 进行过滤
        self.low = low.clone().into_iter().filter(|x| x.ne(id)).collect();
        self.mid = mid.clone().into_iter().filter(|x| x.ne(id)).collect();
        self.fatal = fatal.clone().into_iter().filter(|x| x.ne(id)).collect();
        self.focus = focus.clone().into_iter().filter(|x| x.ne(id)).collect();
    }
}
