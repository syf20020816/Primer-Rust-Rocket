use std::fmt::Display;

use rocket::serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

/// 定义一个枚举`Status`来表示待办事项的状态。
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// 表示待办事项尚未开始。
    NotStart,
    /// 表示待办事项正在进行中。
    InProgress,
    /// 表示待办事项已经完成。
    Completed,
    /// 表示待办事项被暂时阻塞。
    Pending,
    /// 表示待办事项失败。
    Failed,
}

/// 为`Status`实现`Default`特质，指定默认状态为`InProgress`。
impl Default for Status {
    fn default() -> Self {
        Status::InProgress
    }
}

/// 实现`Serialize`特质，允许`Status`枚举值被序列化为字符串。
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        // 将状态转换为字符串并序列化
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// 自定义访问者`StatusVisitor`用于反序列化。
struct StatusVisitor;

impl<'de> Visitor<'de> for StatusVisitor {
    type Value = Status;

    /// 指定期望的输入格式。
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "expected one of `not start`, `in progress`, `completed`, `pending`, `failed`",
        )
    }

    /// 实现处理字符串输入的逻辑。
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        // 根据字符串值匹配对应的状态枚举。
        let value = v.to_lowercase();
        match value.as_str() {
            "not start" => Ok(Status::NotStart),
            "in progress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "pending" => Ok(Status::Pending),
            "failed" => Ok(Status::Failed),
            _ => Err(rocket::serde::de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &self,
            )),
        }
    }
}

/// 实现`Deserialize`特质，允许从字符串反序列化为`Status`枚举。
impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        // 使用自定义访问者进行反序列化
        deserializer.deserialize_str(StatusVisitor)
    }
}

/// 实现`Display`特质，使得`Status`枚举可以被直接转换为字符串。
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 根据枚举值返回对应的字符串表示。
        let res = match self {
            Status::NotStart => "not start",
            Status::InProgress => "in progress",
            Status::Completed => "completed",
            Status::Pending => "pending",
            Status::Failed => "failed",
        };
        f.write_str(res)
    }
}
