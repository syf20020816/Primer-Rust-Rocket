use surreal_use::core::{
    sql::{CreateData, SetField, SurrealTable, UpdateData}, // 引入用于创建和更新数据的结构，以及设置字段的工具
    Stmt,                                                  // 引入用于构建SQL语句的结构
};
use surrealdb::sql::Output; // 引入用于指定SQL查询结果输出类型的枚举

use crate::lib::{
    db::DB,                     // 引入数据库操作的封装
    entry::po::{Status, Todo}, // 引入待办事项（Todo）的数据传输对象和状态（Status）
};

use super::{select_user_record_by_username, Record}; // 引入用户记录查询函数和Record结构体

// 根据用户名创建待办事项记录
pub async fn create_todo_by_username(username: &str, mut todo: Todo) -> Option<(String, Todo)> {
    // 首先根据用户名查询用户记录，获取用户ID
    let query = select_user_record_by_username(username).await;
    if let Some((id, _)) = query {
        // 如果用户存在，设置待办事项的所有者为该用户ID
        todo.set_owner(id);
    } else {
        // 如果用户不存在，直接返回None
        return None;
    }

    // 构建创建待办事项记录的SQL语句
    let sql = Stmt::create()
        .table("todo".into()) // 指定表名
        .data(todo.into()) // 设置待创建的数据
        .output(Output::After) // 指定返回创建后的数据
        .to_string(); // 将Stmt对象转换为字符串
                      // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则返回该记录
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

// 根据团队ID创建待办事项记录
pub async fn create_todo_by_team(team_id: &str, mut todo: Todo) -> Option<(String, Todo)> {
    // 设置待办事项的所有者为团队ID
    let _ = todo.set_owner(team_id.to_string());

    // 构建创建待办事项记录的SQL语句，逻辑与根据用户名创建待办事项类似
    let sql = Stmt::create()
        .table("todo".into())
        .data(todo.into())
        .output(Output::After)
        .to_string();
    // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则返回该记录
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

// 根据ID查询待办事项记录
pub async fn select_todo_record(id: &str) -> Option<(String, Todo)> {
    // 构建查询待办事项记录的SQL语句
    let sql = Stmt::select()
        .table(("todo", id).into()) // 指定表名和ID
        .field_all() // 查询所有字段
        .to_string(); // 将Stmt对象转换为字符串
                      // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则返回该记录
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

// 根据ID删除待办事项记录
pub async fn delete_todo_by_id(id: &str) -> bool {
    // 构建删除待办事项记录的SQL语句
    let sql = Stmt::delete()
        .table(("todo", id).into()) // 指定表名和ID
        .output(Output::Before) // 指定返回删除前的数据
        .to_string(); // 将Stmt对象转换为字符串
                      // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则认为删除成功
    sql_result.len() == 1
}

// 根据ID更新待办事项记录
pub async fn update_todo_by_id(id: &str, todo: Todo) -> bool {
    // 构建更新待办事项记录的SQL语句，逻辑与删除待办事项类似
    let sql = Stmt::update()
        .table(("todo", id).into())
        .data(UpdateData::content(todo))
        .output(Output::After)
        .to_string();
    // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则认为更新成功
    sql_result.len() == 1
}

// 根据ID更新待办事项的状态
pub async fn update_todo_status_by_id(id: &str, status: &str) -> bool {
    // 构建更新待办事项状态的SQL语句
    let sql = Stmt::update()
        .table(("todo", id).into()) // 指定表名和ID
        .data(UpdateData::set().push(SetField::new("status", None, status))) // 设置新的状态值
        .output(Output::After) // 指定返回更新后的数据
        .to_string(); // 将Stmt对象转换为字符串
                      // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Todo>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则认为状态更新成功
    sql_result.len() == 1
}
