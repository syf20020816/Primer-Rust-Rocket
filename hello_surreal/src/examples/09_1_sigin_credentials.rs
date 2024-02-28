use serde::Serialize;
use surrealdb::opt::auth::{Database, Namespace, Root, Scope};

fn main() {
    //设置一个对应Scope的结构体ScopeCredential
    //对应Scope中的params字段
    //由于Scope实现了serde的Serialize和Debug trait，所以自己编写的ScopeCredential也需要去实现
    #[derive(Serialize, Debug)]
    struct ScopeCredential<'a> {
        email: &'a str,
        pass: &'a str,
        name: &'a str,
    }
    let root = Root {
        username: "root",
        password: "root",
    };
    let namespace = Namespace {
        namespace: "test",
        username: "root",
        password: "root",
    };
    let database = Database {
        namespace: "test",
        database: "test",
        username: "root",
        password: "root",
    };
    let scope = Scope {
        namespace: "test",
        database: "test",
        scope: "test_sc",
        params: ScopeCredential {
            email: "Matt@gmail.com",
            pass: "Matt001",
            name: "Matt",
        },
    };
}
