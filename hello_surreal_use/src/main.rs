use surreal_use::{core::sql::Field, core::Stmt};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let select = Stmt::select()
        .table("user".into())
        .fields(Field::from_vec(vec!["name", "address", "email"]))
        .to_string();
    //使用query发起语句
    dbg!(select);
    Ok(())
}
