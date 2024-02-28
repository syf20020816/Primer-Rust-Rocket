use rocket::serde::{self, Deserialize, Serialize};

// 定义一个Date结构体，用于表示日期区间及其持续时间
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")] // 使用rocket框架的serde来进行序列化和反序列化
pub struct Date {
    // 开始日期
    start: String,
    // 结束日期
    end: String,
    // 持续时间，以某种单位表示（此处根据实现逻辑应为小时数，但由于计算方式的问题，可能需要调整）
    during: usize,
}

// 为Date结构体实现Default trait，提供一个默认实例
impl Default for Date {
    fn default() -> Self {
        Self {
            // 默认开始日期为空字符串
            start: String::new(),
            // 默认结束日期为空字符串
            end: String::new(),
            // 默认持续时间为0
            during: 0_usize,
        }
    }
}

impl Date {
    // 提供一个新建Date实例的方法，接受开始和结束日期为字符串
    pub fn new(start: &str, end: &str) -> Self {
        // 将开始日期转换为String类型
        let start = start.to_string();
        // 将结束日期转换为String类型
        let end = end.to_string();
        // 计算持续时间，此处逻辑有误，因为parse::<usize>()尝试将日期字符串解析为数字，这在常规日期格式下不可行
        // 应该使用专门的日期处理库来计算日期之间的差异
        let during = start.parse::<usize>().unwrap() - end.parse::<usize>().unwrap();
        Date { start, end, during }
    }
}
