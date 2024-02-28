use rocket::serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use std::fmt::Display;

// 定义一个枚举来表示不同类型的标签。
#[derive(Debug, Clone, PartialEq)]
pub enum Tags {
    Info,
    Success,
    Warning,
    Default,
    Danger,
}

// 为Tags枚举提供一个默认值，这里使用Default作为默认值。
impl Default for Tags {
    fn default() -> Self {
        Tags::Default
    }
}

// 自定义访问者结构体，用于在反序列化过程中根据字符串值创建Tags枚举实例。
struct TagsVisitor;

impl<'de> Visitor<'de> for TagsVisitor {
    type Value = Tags;

    // 定义期待的输入格式。
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .write_str("a string containing 'info', 'success', 'warning', 'default', or 'danger'")
    }

    // 实现如何从字符串转换为Tags枚举值。
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: rocket::serde::de::Error,
    {
        let value = v.to_lowercase();
        match value.as_str() {
            "info" => Ok(Tags::Info),
            "success" => Ok(Tags::Success),
            "" => Ok(Tags::Default),
            "danger" => Ok(Tags::Danger),
            "warning" => Ok(Tags::Warning),
            _ => Err(rocket::serde::de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &self,
            )),
        }
    }
}

// 实现Deserialize特性以支持从字符串反序列化Tags枚举。
impl<'de> Deserialize<'de> for Tags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: rocket::serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TagsVisitor)
    }
}

// 实现Serialize特性以支持将Tags枚举序列化为字符串。
impl Serialize for Tags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

// 实现Display特性以支持将Tags枚举转换为字符串。
impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Tags::Info => "info",
            Tags::Success => "success",
            Tags::Warning => "warning",
            Tags::Default => "",
            Tags::Danger => "danger",
        };
        f.write_str(res)
    }
}

// 定义另一个枚举来表示标签的视觉效果。
#[derive(Debug, Clone, PartialEq)]
pub enum Effects {
    Dark,
    Light,
    Plain,
}

// 实现Serialize特性以支持将Effects枚举序列化为字符串。
impl Serialize for Effects {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        let res = match self {
            Effects::Dark => "dark",
            Effects::Light => "light",
            Effects::Plain => "plain",
        };
        serializer.serialize_str(res)
    }
}

// 自定义访问者结构体，用于在反序列化过程中根据字符串值创建Effects枚举实例。
struct EffectsVisitor;

impl<'de> Visitor<'de> for EffectsVisitor {
    type Value = Effects;

    // 定义期待的输入格式。
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing `dark`, `light`, `plain`'")
    }

    // 实现如何从字符串转换为Effects枚举值。
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = v.to_lowercase();
        match value.as_str() {
            "dark" => Ok(Effects::Dark),
            "plain" => Ok(Effects::Plain),
            "light" => Ok(Effects::Light),
            _ => Err(rocket::serde::de::Error::invalid_value(
                de::Unexpected::Str(&value),
                &self,
            )),
        }
    }
}

// 实现Deserialize特性以支持从字符串反序列化Effects枚举。
impl<'de> Deserialize<'de> for Effects {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(EffectsVisitor)
    }
}

/// 定义一个结构体来表示ElementPlus中el-tag组件的属性。
/// 这个结构体用于在Rust后端与前端框架如Vue或React中的ElementPlus组件进行数据交换。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ITagProps {
    // 使用serde重命名特性来避免Rust关键字冲突。
    #[serde(rename(serialize = "type"))]
    r#type: Tags,
    effect: Effects,
    label: String,
}

// 提供一个构造函数来简化ITagProps实例的创建。
impl ITagProps {
    pub fn new(r#type: Tags, effect: Effects, label: String) -> Self {
        ITagProps {
            r#type,
            effect,
            label,
        }
    }
}
