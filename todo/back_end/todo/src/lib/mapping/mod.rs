mod team;
mod todo;
mod user;

use rocket::serde::{Deserialize, Serialize}; // 引入用于序列化和反序列化的特性
use surrealdb::sql::Thing; // 引入surrealdb中的Thing类型，常用于表示数据库中的一个记录的ID
pub use team::*; // 将team模块的所有公开内容重新导出
pub use todo::*; // 将todo模块的所有公开内容重新导出
pub use user::*; // 将user模块的所有公开内容重新导出

// 定义一个通用的Record结构体，用于表示数据库记录
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(crate = "rocket::serde")] // 指定使用rocket框架的serde实现进行序列化和反序列化
struct Record<T> {
    pub id: Thing, // 记录的ID，使用surrealdb的Thing类型
    #[serde(flatten)] // 属性平铺，将T的字段直接嵌入到Record结构体中
    pub data: T, // 泛型参数T，表示记录中的数据部分
}

// 为Record结构体实现方法
impl<T> Record<T> {
    // 将Record实例转换为一个包含ID字符串和数据部分的元组
    pub fn to_record(self) -> (String, T) {
        let Record { id, data } = self; // 解构self以获取id和data

        (id.id.to_string(), data) // 将id转换为字符串，并与data一起返回
    }
}
