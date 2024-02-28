use rocket::serde::{Deserialize, Serialize};

// 附件类型
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Annex {
    /// 附件名称
    name: String,
    /// 附件数据(Base64)
    data: String,
}
