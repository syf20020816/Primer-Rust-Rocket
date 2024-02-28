// 引入标准库中的HashSet用于去重
use std::collections::HashSet;

// 引入Rocket框架的Json工具，用于处理JSON数据
use rocket::serde::json::Json;

// 引入项目内部模块，包括DTOs（数据传输对象）、VOs（值对象）、错误处理以及数据库操作函数
use crate::lib::{
    entry::{
        po::{self, Todo}, // DTOs，用于接收和发送数据
        dto::{self, Team},  // VOs，用于内部逻辑处理和展示
    },
    error::Error, // 错误处理模块
    mapping::{
        // 数据库操作映射，包括创建、更新、查询等操作
        create_team_by_name,
        create_todo_by_team,
        create_todo_by_username,
        select_team_record_by_id,
        select_user_by_username,
        update_team_by_id,
        update_user_by_username,
    },
    response::ResultJsonData, // 自定义的结果数据类型，用于统一API响应格式
};

// 创建团队的API处理函数
#[get("/<username>/<name>")]
pub async fn create_team(username: &str, name: &str) -> ResultJsonData<dto::User> {
    // 通过团队名和用户名创建团队
    let query = create_team_by_name(name, username).await;

    // 如果创建成功，尝试将团队ID添加到用户信息中并更新用户信息
    if let Some((id, _team)) = query {
        let mut user = select_user_by_username(username).await.unwrap();
        let _ = user.create_team(&id);
        let update_query = update_user_by_username(user).await;
        if let Some(user) = update_query {
            let user = dto::User::from(user).await;
            // 成功时返回更新后的用户信息
            return ResultJsonData::success(user);
        }
    }
    // 如果创建失败，返回错误信息
    let e = Error::CreateTeam;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

// 更新团队成员的API处理函数
#[put("/<name>", format = "application/json", data = "<team>")]
pub async fn update_team_members(name: &str, team: Json<Team>) -> ResultJsonData<bool> {
    // 从请求体中解析团队ID和团队信息
    let id = team.0.id.clone();
    let mut team = po::Team::from(team.0);
    // 查询用户名对应的用户信息
    let user_query = select_user_by_username(name).await;
    if let Some(mut user) = user_query {
        // 添加成员到团队并尝试更新用户所在团队信息
        let _ = team.add_member(name);
        let _ = user.create_team(&id);
        let update_user = update_user_by_username(user).await;
        // 如果用户信息更新成功，尝试更新团队信息
        if update_user.is_some() {
            let query = update_team_by_id(&id, team).await;
            if query {
                // 成功时返回true
                return ResultJsonData::success(true);
            }
        }
        // 更新团队失败时返回错误信息
        let e = Error::UpdateTeam;
        let (e_code, e_msg) = e.get();
        return ResultJsonData::define_failure(e_code, &e_msg);
    }
    // 用户不存在时返回错误信息
    let e = Error::AccountUnExist;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

// 更新团队信息的API处理函数
#[put("/", format = "application/json", data = "<team>")]
pub async fn update_team_info(team: Json<Team>) -> ResultJsonData<bool> {
    // 从请求体中解析团队ID和团队信息
    let id = team.0.id.clone();
    let team = po::Team::from(team.0);
    // 尝试更新团队信息
    let query = update_team_by_id(&id, team).await;
    if query {
        // 成功时返回true
        return ResultJsonData::success(true);
    }
    // 更新失败时返回错误信息
    let e = Error::UpdateTeam;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}

// 创建团队待办事项的API处理函数
#[post("/todo/<id>", format = "application/json", data = "<todo>")]
pub async fn create_team_todo(id: &str, todo: Json<Todo>) -> ResultJsonData<bool> {
    // 解析待办事项中的审核者和执行者，确保不重复
    let todo = todo.0;
    let reviewers = todo.reviewers.clone();
    let performers = todo.performers.clone();
    let team_id = id.to_string();
    // 使用HashSet去重，再转换为Vec
    let users = reviewers
        .into_iter()
        .chain(performers.into_iter())
        .collect::<HashSet<String>>();
    let users = users.into_iter().collect::<Vec<String>>();
    // 尝试为团队创建待办事项
    let create_query = create_todo_by_team(&team_id, todo.clone()).await;
    let mut flag = true;
    if let Some((todo_id, todo)) = create_query {
        // 查询团队记录并尝试更新团队待办事项
        let team_query = select_team_record_by_id(&team_id).await;
        if let Some((id, mut team)) = team_query {
            let _ = team.push_todo(&todo_id);
            let update_team = update_team_by_id(&id, team).await;
            if update_team {
                // 遍历所有涉及的用户，更新他们的待办事项列表
                for username in users {
                    let user_query = select_user_by_username(&username).await;
                    if let Some(mut user) = user_query {
                        let priority = todo.priority();
                        let _ = user.add_todo(todo_id.clone(), priority);
                        let update_query = update_user_by_username(user).await;
                        if update_query.is_some() {
                            continue;
                        } else {
                            flag = false;
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    if flag {
        // 所有操作成功时返回true
        return ResultJsonData::success(true);
    }

    // 创建团队TODO失败时返回错误信息
    let e = Error::CreateTeamTodo;
    let (e_code, e_msg) = e.get();
    return ResultJsonData::define_failure(e_code, &e_msg);
}
