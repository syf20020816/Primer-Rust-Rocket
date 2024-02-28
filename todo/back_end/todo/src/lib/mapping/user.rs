use super::Record;
use crate::lib::{
    db::DB,
    entry::{
        po::{Avatars, User},
        dto::UserPersonalSetting,
    },
};
use rocket::serde::{Deserialize, Serialize};
use surreal_use::core::{
    sql::{Cond, CreateData, SetField, UpdateData},
    Stmt,
};
use surrealdb::sql::{Operator, Output};

// 通过用户ID查询用户
pub async fn select_user_by_id(id: &str) -> Option<User> {
    let table = format!("user:{}", id);
    let sql = Stmt::select().table(table.as_str().into()).to_string();

    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let mut res = sql_result[0].clone();
        let _ = res.skip_pwd();
        Some(res)
    } else {
        None
    }
}

// 通过用户名查询用户(携带记录的ID)
pub async fn select_user_record_by_username(username: &str) -> Option<(String, User)> {
    let sql = Stmt::select()
        .table("user".into())
        .field_all()
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<Record<User>> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

// 使用用户名查询用户
pub async fn select_user_by_username(username: &str) -> Option<User> {
    let query = select_user_record_by_username(username).await;
    if let Some((_id, user)) = query {
        Some(user)
    } else {
        None
    }
}

// 检查是否已经有用户了
pub async fn check_user_by_username(username: &str) -> bool {
    // 这里见到那写个零时的结构体用于获取用户
    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct TmpUser {
        username: String,
    }

    // 检查是否已经有相同的用户名
    let sql = Stmt::select()
        .table("user".into())
        .fields(vec!["username".into()])
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .to_string();

    let mut result = DB.query(sql).await.unwrap();
    let check_result: Vec<TmpUser> = result.take(0_usize).unwrap();
    check_result.len().eq(&0_usize)
}

// 创建一个新用户
pub async fn create_user(user: User) -> Option<User> {
    let sql = Stmt::create()
        .table("user".into())
        .data(CreateData::content(user))
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 0 {
        return None;
    }
    let res = sql_result[0].clone();
    Some(res)
}

// 通过用户名密码查询用户
// 使用Where子句进行过滤
pub async fn select_user_by_username_password(username: &str, password: &str) -> Option<User> {
    // 创建用户名过滤条件
    let username_cond = Cond::new()
        .left("username")
        .op(Operator::Equal)
        .right(username.into())
        .to_origin()
        .0;
    // 创建密码过滤条件
    let password_cond = Cond::new()
        .left("password")
        .op(Operator::Equal)
        .right(password.into())
        .to_origin()
        .0;

    // 结果类似: SELECT * FROM user WHERE username = 'matt000' AND password = 'matt000'
    let sql = Stmt::select()
        .table("user".into())
        .field_all()
        .cond(
            Cond::new()
                .left_value(username_cond)
                .op(Operator::And)
                .right(password_cond),
        )
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res)
    } else {
        None
    }
}

// 更新用户设置
pub async fn update_user_by_personal_settings(
    user: UserPersonalSetting,
    username: &str,
) -> Option<User> {
    let name = user.name();
    let email = user.email();
    let send_email = user.send_email();
    let send_msg = user.send_msg();

    // 由于这里只需要更新四个字段
    // 使用SET方式构建更新会更加简单
    let update_set = vec![
        SetField::new("name", None, name),
        SetField::new("email", None, email),
        SetField::new("sendEmail", None, send_email),
        SetField::new("sendMsg", None, send_msg),
    ];

    let sql = Stmt::update()
        .table("user".into())
        .data(UpdateData::Set(update_set))
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res)
    } else {
        None
    }
}

// 更新用户的头像通过用户名获取目标用户进行过滤
pub async fn update_user_avatar(username: &str, avatar: Avatars) -> bool {
    let avatar = avatar.to_string();
    let sql = Stmt::update()
        .table("user".into())
        .data(UpdateData::set().push(SetField::new("avatar", None, avatar)))
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        return true;
    } else {
        return false;
    }
}
// 通过用户名过滤更新用户
pub async fn update_user_by_username(user: User) -> Option<User> {
    let username = user.clone().username;
    let sql = Stmt::update()
        .table("user".into())
        .data(UpdateData::content(user))
        .cond(
            Cond::new()
                .left_easy("username")
                .op(Operator::Equal)
                .right(username.into()),
        )
        .output(Output::After)
        .to_string();
    let mut result = DB.query(sql).await.unwrap();
    let sql_result: Vec<User> = result.take(0_usize).unwrap();
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res)
    } else {
        None
    }
}
