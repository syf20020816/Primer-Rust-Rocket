use std::fmt::Display;

use rocket::serde::{Deserialize, Serialize};

// 定义一个枚举类型，用于表示不同的头像
// 指定使用rocket框架的serde进行序列化和反序列化
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Avatars {
    Worker,
    Miner,
    Adventurer,
}

// 为Avatars枚举实现Default trait，提供默认值
impl Default for Avatars {
    fn default() -> Self {
        // 默认头像设置为Worker
        Avatars::Worker
    }
}

impl Avatars {
    // 新建一个Avatars实例，使用默认值
    pub fn new() -> Self {
        Self::default()
    }
    // 提供构造函数，方便直接创建特定头像的实例
    pub fn worker() -> Self {
        Avatars::Worker
    }
    pub fn miner() -> Self {
        Avatars::Miner
    }
    pub fn adventurer() -> Self {
        Avatars::Adventurer
    }
}

// 为Avatars实现Display trait，使其可以更方便地转换为字符串形式输出
impl Display for Avatars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Avatars::Worker => "Worker",
            Avatars::Miner => "Miner",
            Avatars::Adventurer => "Adventurer",
        };
        f.write_str(res)
    }
}

// 实现从&str到Avatars的转换，允许从字符串直接创建Avatars枚举实例
impl From<&str> for Avatars {
    fn from(value: &str) -> Self {
        // 将输入字符串转换为小写，以忽略大小写差异
        let value = value.to_lowercase();
        match value.as_str() {
            "worker" => Avatars::Worker,
            "miner" => Avatars::Miner,
            "adventurer" => Avatars::Adventurer,
            // 如果输入不匹配任何已知头像，抛出panic
            _ => panic!("Invalid Avatars"),
        }
    }
}

// 定义另一个枚举类型，用于表示不同的团队头像
// 同样指定使用rocket框架的serde
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TeamAvatars {
    Team1,
    Team2,
    Team3,
    Team4,
}

// 为TeamAvatars实现Default trait，提供默认值
impl Default for TeamAvatars {
    fn default() -> Self {
        // 默认团队头像设置为Team1
        TeamAvatars::Team1
    }
}
