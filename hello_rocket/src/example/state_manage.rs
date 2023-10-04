#[macro_use]
extern crate rocket;

use rocket::{launch, get, routes, catchers, State};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::{Json, Value};
use std::mem;

// 添加状态需要保证数据的原子性
// 在程序中可以广泛应用到std::sync::atomic下的各种类型
// use std::sync::atomic::*;
use std::sync::{Arc, Mutex};


//构建一个被数据库生产的模拟数据
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct User {
    id: u8,
    name: String,
    // status表示用户状态(上线或离线)
    status: bool,
}

impl User {
    pub fn new(id: u8, name: &str) -> Self {
        User {
            id,
            name: String::from(name),
            status: false,
        }
    }
}

//构建一个模拟数据库的结构体
//它的目的是生产一个数据
//再将数据添加到状态管理中
struct DB;

impl DB {
    // 生成一系列用户数据
    pub fn product() -> Vec<User> {
        let mut users: Vec<User> = Vec::new();
        for (id, name) in [
            (1, "Matt"),
            (2, "John"),
            (3, "Kaven")
        ] {
            users.push(User::new(id, name));
        }
        users
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
struct UserList(Vec<User>);

impl UserList {
    pub fn new() -> Self {
        UserList(Vec::new())
    }
    //将新数据与旧数据交换内存空间起到替换
    pub fn copy_from(&mut self, value: Vec<User>) -> &mut Self {
        let _ = mem::replace(self, UserList(value));
        self
    }
}


#[get("/1")]
fn get_user_list1(arc_user_list: &State<Arc<Mutex<UserList>>>) -> Json<UserList> {
    //检查UserList是否长度为0
    //若为0则需要请求数据库获取UserList
    //若不为0则快速返回即可
    let inner = arc_user_list.inner();
    //锁住，进行独占访问
    let mut lock_inner = inner.lock().unwrap();
    let mut user_list = UserList::new();
    if lock_inner.0.len().eq(&0_usize) {
        let _ = lock_inner.copy_from(
            DB::product()
        );
    }
    let inner_vec = lock_inner.0.clone();
    let _ = user_list.copy_from(inner_vec);
    Json(user_list)
}

#[get("/2")]
fn get_user_list2(user_list: &State<Arc<Mutex<UserList>>>) -> Json<UserList> {
    let mut inner = user_list
        .inner()
        .lock()
        .unwrap();
    let mut user_list = UserList::new();
    let _ = user_list.copy_from(inner.0.clone());
    Json(user_list)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_user_list1,get_user_list2])
        // 预存储数据
        .manage(
            Arc::new(
                Mutex::new(UserList::new())
            )
        )
}
