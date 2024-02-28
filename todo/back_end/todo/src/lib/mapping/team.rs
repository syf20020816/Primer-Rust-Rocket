use surreal_use::core::{
    sql::{CreateData, UpdateData}, // 引入用于创建和更新数据的结构
    Stmt,                          // 引入用于构建SQL语句的结构
};
use surrealdb::sql::Output; // 引入用于指定SQL查询结果输出类型的枚举

use crate::lib::{
    db::DB,                 // 引入数据库操作的封装
    entry::{po::Team, dto}, // 引入数据传输对象（DTO）和值对象（VO）
};

use super::Record; // 引入Record结构体，用于处理数据库记录

// 创建一个新的团队记录，并返回创建的团队信息
pub async fn create_team_by_name(name: &str, owner: &str) -> Option<(String, Team)> {
    // 使用随机ID创建新的Team实例
    let team = Team::new_rand(name, owner);
    // 构建创建Team记录的SQL语句
    let sql = Stmt::create()
        .table("team".into()) // 指定表名
        .data(CreateData::content(team)) // 设置要创建的数据
        .output(Output::After) // 指定返回创建后的数据
        .to_string(); // 将Stmt对象转换为字符串
                      // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Team>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则返回该记录
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

// 根据ID查询团队记录
pub async fn select_team_record_by_id(id: &str) -> Option<(String, Team)> {
    // 构建查询Team记录的SQL语句
    let sql = Stmt::select()
        .table(("team", id).into()) // 指定表名和ID
        .field_all() // 查询所有字段
        .to_string(); // 将Stmt对象转换为字符串
                      // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Team>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则返回该记录
    if sql_result.len() == 1 {
        let res = sql_result[0].clone();
        Some(res.to_record())
    } else {
        None
    }
}

// 根据ID更新团队记录，并返回是否更新成功
pub async fn update_team_by_id(id: &str, team: Team) -> bool {
    // 构建更新Team记录的SQL语句
    let sql = Stmt::update()
        .table(("team", id).into()) // 指定表名和ID
        .data(UpdateData::content(team)) // 设置要更新的数据
        .output(Output::After) // 指定返回更新后的数据
        .to_string(); // 将Stmt对象转换为字符串
                      // 执行SQL语句
    let mut result = DB.query(sql).await.unwrap();
    // 解析SQL执行结果
    let sql_result: Vec<Record<Team>> = result.take(0_usize).unwrap();
    // 如果查询结果恰好有一条记录，则认为更新成功
    sql_result.len() == 1
}
