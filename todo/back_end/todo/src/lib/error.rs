// 引入标准库的Display特性，用于自定义类型的显示格式
use std::fmt::Display;

// 定义一个公开的Error枚举，包含各种可能的错误类型
#[derive(Debug)]
pub enum Error {
    /// 身份认证失败错误，通常表示登录时的账号或密码错误
    IdentityAuthentication,
    /// 账户已存在错误，当尝试创建一个已存在的账户时返回
    ExistAccount,
    /// 账户不存在错误，当尝试操作一个不存在的账户时返回
    AccountUnExist,
    /// 更改用户设置失败错误
    ChangeUserSetting,
    /// 更改用户头像失败错误
    ChangeUserAvatar,
    /// 更新用户数据失败错误
    UpdateUser,
    /// 创建待办事项失败错误
    CreateTodo,
    /// 删除待办事项失败错误
    DeleteTodo,
    /// 更新待办事项失败错误
    UpdateTodo,
    /// 完成待办事项失败错误
    CompleteTodo,
    /// 创建团队失败错误
    CreateTeam,
    /// 更新团队失败错误
    UpdateTeam,
    /// 创建团队待办事项失败错误
    CreateTeamTodo,
}

// 为Error枚举实现get方法，用于返回错误的代码和描述
impl Error {
    pub fn get(self) -> (u16, String) {
        match self {
            // 每个错误类型返回其对应的错误代码和通过Display trait转换成的字符串描述
            Error::IdentityAuthentication => (1001, self.to_string()),
            Error::ExistAccount => (1002, self.to_string()),
            Error::AccountUnExist => (1006, self.to_string()),
            Error::ChangeUserSetting => (1003, self.to_string()),
            Error::ChangeUserAvatar => (1004, self.to_string()),
            Error::UpdateUser => (1005, self.to_string()),
            Error::CreateTodo => (1101, self.to_string()),
            Error::DeleteTodo => (1102, self.to_string()),
            Error::UpdateTodo => (1103, self.to_string()),
            Error::CompleteTodo => (1104, self.to_string()),
            Error::CreateTeam => (1201, self.to_string()),
            Error::UpdateTeam => (1202, self.to_string()),
            Error::CreateTeamTodo => (1203, self.to_string()),
        }
    }
}

// 为Error枚举实现Display trait，以自定义错误信息的输出格式
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 根据错误类型匹配对应的错误描述
        let alert_msg = match self {
            Error::IdentityAuthentication => {
                "Identity Authentication Failed: Incorrect username or password"
            }
            Error::ExistAccount => "The current account already exists. Please change the username",
            Error::AccountUnExist => "Account is not exist, please check!",
            Error::ChangeUserSetting => "Failed to modify user configuration",
            Error::ChangeUserAvatar => "Failed to modify user avatar",
            Error::UpdateUser => "Update user data failed",
            Error::CreateTodo => "Create a new todo failed",
            Error::DeleteTodo => "Delete todo failed",
            Error::UpdateTodo => "Update todo failed",
            Error::CompleteTodo => "Complete todo failed",
            Error::CreateTeam => "Create team failed",
            Error::UpdateTeam => "Update team failed",
            Error::CreateTeamTodo => "Create team todo failed",
        };
        // 使用Formatter将错误描述写入到输出中
        f.write_str(alert_msg)
    }
}

// 实现标准库的Error trait，为Error类型提供标准错误支持
impl std::error::Error for Error {
    // 定义错误的原因，这里简单地返回自己，因为我们没有嵌套错误
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}
