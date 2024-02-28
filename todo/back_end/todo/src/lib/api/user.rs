// 导入所需的模块和类型
use crate::lib::entry::po::Avatars;
use crate::lib::entry::po::{Signin, Signup, User};
use crate::lib::entry::dto;
use crate::lib::entry::dto::UserPersonalSetting;
use crate::lib::error::Error;
use crate::lib::mapping::{
    check_user_by_username, create_user, select_user_by_username, select_user_by_username_password,
    update_user_avatar, update_user_by_personal_settings,
};
use crate::lib::response::ResultJsonData;
use rocket::serde::json::Json;

// 定义登录接口，接受JSON格式的登录数据
#[post("/signin", format = "application/json", data = "<user>")]
pub async fn signin(user: Json<Signin>) -> ResultJsonData<dto::User> {
    // 从请求体中提取用户名和密码
    let username = user.0.username();
    let password = user.0.password();

    // 查询数据库，检查用户名和密码是否匹配
    let query = select_user_by_username_password(username, password).await;
    if let Some(mut user) = query {
        // 忽略密码字段，不返回给客户端
        // 登录成功，返回用户信息
        let user = dto::User::from(user).await;
        return ResultJsonData::success(user);
    }
    // 登录失败，返回错误信息
    let e = Error::IdentityAuthentication;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

// 定义注册接口，接受JSON格式的注册数据
#[post("/signup", format = "application/json", data = "<user>")]
pub async fn signup(user: Json<Signup>) -> ResultJsonData<dto::User> {
    // 从请求体中提取用户信息
    let user = user.0;
    let username = user.username();
    // 检查用户名是否已存在
    let exist = check_user_by_username(username).await;
    if !exist {
        // 用户名已存在，返回错误信息
        let error = Error::ExistAccount;
        let (e_code, e_msg) = error.get();
        return ResultJsonData::define_failure(e_code, &e_msg);
    } else {
        // 用户名不存在，创建新用户
        let user = User::quick_init(user.name(), user.username(), user.password(), user.email());

        // 将新用户信息保存到数据库
        let query = create_user(user).await;
        if let Some(user) = query {
            // 注册成功，忽略密码返回用户信息
            // user.skip_pwd();
            let user = dto::User::from(user).await;
            return ResultJsonData::success(user);
        }
        // 数据库操作失败，返回错误信息
        return ResultJsonData::failure("Server data error: api::signup");
    }
}

// 定义获取用户信息接口
#[get("/info/<username>", format = "application/json")]
pub async fn get_user_info(username: &str) -> ResultJsonData<dto::User> {
    // 根据用户名查询用户信息
    let query = select_user_by_username(username).await;
    if let Some(user) = query {
        // 查询成功，返回用户信息
        let user = dto::User::from(user).await;
        return ResultJsonData::success(user);
    }
    // 用户不存在，返回错误信息
    let e = Error::IdentityAuthentication;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

// 定义更新用户个人设置接口
#[post("/info/<username>", format = "application/json", data = "<user>")]
pub async fn set_user_setting(
    username: &str,
    user: Json<UserPersonalSetting>,
) -> ResultJsonData<dto::User> {
    // 从请求体中提取用户个人设置信息
    let user = user.0;
    // 更新数据库中的用户个人设置信息
    let query = update_user_by_personal_settings(user, username).await;
    if let Some(user) = query {
        // 更新成功，返回更新后的用户信息
        let user = dto::User::from(user).await;
        return ResultJsonData::success(user);
    }
    // 更新失败，返回错误信息
    let e = Error::ChangeUserSetting;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

// 定义更新用户头像接口
#[get("/info/<username>/<avatar>")]
pub async fn set_user_avatar(username: &str, avatar: &str) -> ResultJsonData<bool> {
    // 从请求参数中提取头像信息
    let avatar = Avatars::from(avatar);
    // 更新数据库中的用户头像信息
    let query = update_user_avatar(username, avatar).await;
    if query {
        // 更新成功，返回成功标志
        return ResultJsonData::success(true);
    } else {
        // 更新失败，返回错误信息
        let e = Error::ChangeUserAvatar;
        let (e_code, e_msg) = e.get();
        return ResultJsonData::define_failure(e_code, &e_msg);
    }
}
