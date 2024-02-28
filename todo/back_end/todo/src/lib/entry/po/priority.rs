use std::fmt::Display;

use rocket::serde::{Deserialize, Serialize};

/// 表示一个16进制颜色值的结构体。
/// 使用String来存储颜色值。
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HexColor(pub String);

impl HexColor {
    /// 创建一个新的HexColor实例。
    pub fn new(color: &str) -> Self {
        Self(color.to_string())
    }

    /// 获取颜色值的字符串表示。
    pub fn get_color(&self) -> &str {
        &self.0
    }
}

/// 为HexColor实现Serialize特质，允许将HexColor实例序列化为JSON等格式。
impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: rocket::serde::Serializer,
    {
        // 直接将颜色值序列化为字符串
        serializer.serialize_str(self.get_color())
    }
}

/// 实现Display特质，允许将HexColor实例格式化为字符串。
impl Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 直接输出颜色值字符串
        f.write_str(self.get_color())
    }
}

/// 定义一个枚举，表示不同的优先级。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Priorities {
    // 紧急类型
    Emergent,
    // 高优先级
    High,
    // 中等优先级
    Mid,
    // 低优先级(优先级最低, 可放到最后处理)
    Low,
}

/// 为Priorities枚举提供方法，获取与优先级对应的颜色值。
impl Priorities {
    pub fn get_color(&self) -> HexColor {
        let color = match self {
            Priorities::Emergent => "#E86D5E",
            Priorities::High => "#F69D50",
            Priorities::Mid => "#6CB6FF",
            Priorities::Low => "#ADAC9A",
        };
        HexColor::new(color)
    }
}

/// 实现从Priorities到Priority的转换。
impl From<Priorities> for Priority {
    fn from(value: Priorities) -> Self {
        let color = value.get_color();
        Priority::new(color, value)
    }
}

/// 实现Default特质，为Priorities提供默认值。
impl Default for Priorities {
    fn default() -> Self {
        Self::Mid
    }
}

/// 实现Display特质，允许将Priorities枚举格式化为字符串。
impl Display for Priorities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Priorities::Emergent => "Emergency",
            Priorities::High => "High",
            Priorities::Mid => "Mid",
            Priorities::Low => "Low",
        };
        f.write_str(res)
    }
}

/// 定义一个结构体，表示带有颜色的优先级。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Priority {
    // 存储优先级对应的16进制颜色值
    color: HexColor,
    // 优先级的枚举值
    name: Priorities,
}

/// 为Priority提供默认实例。
impl Default for Priority {
    fn default() -> Self {
        Priorities::default().into()
    }
}

impl Priority {
    /// 创建一个新的Priority实例。
    pub fn new(color: HexColor, name: Priorities) -> Self {
        Priority { color, name }
    }

    /// 获取颜色值的字符串表示。
    pub fn color(&self) -> &str {
        self.color.get_color()
    }

    /// 获取优先级名称的字符串表示。
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

/// 实现Display特质，允许将Priority实例格式化为字符串。
impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 格式化输出优先级详情
        f.write_fmt(format_args!(
            "Priority{}\n\tcolor:{}\n\tname:{}\n{}",
            "{",
            self.color(),
            self.name(),
            "}"
        ))
    }
}
