use rocket::serde::{self, Deserialize, Serialize};

// 定义一个用于用户登录的结构体，包含用户名和密码字段
// 指定使用rocket框架的serde进行序列化和反序列化
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Signin {
    // 用户名
    username: String,
    // 密码
    password: String,
}

impl Signin {
    // 获取用户名的方法，返回用户名的不可变引用
    pub fn username(&self) -> &str {
        &self.username
    }
    // 获取密码的方法，返回密码的不可变引用
    pub fn password(&self) -> &str {
        &self.password
    }
}

// 定义一个用于用户注册的结构体，包含用户名、姓名、电子邮件和密码字段
// 指定使用rocket框架的serde进行序列化和反序列化
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(crate = "rocket::serde")]
pub struct Signup {
    // 用户名
    username: String,
    // 姓名
    name: String,
    // 电子邮件地址
    email: String,
    // 密码
    password: String,
}

impl Signup {
    // 获取用户名的方法，返回用户名的不可变引用
    pub fn username(&self) -> &str {
        &self.username
    }
    // 获取密码的方法，返回密码的不可变引用
    pub fn password(&self) -> &str {
        &self.password
    }
    // 获取电子邮件地址的方法，返回电子邮件地址的不可变引用
    pub fn email(&self) -> &str {
        &self.email
    }
    // 获取姓名的方法，返回姓名的不可变引用
    pub fn name(&self) -> &str {
        &self.name
    }
}
