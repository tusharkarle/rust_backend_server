pub mod requests;
pub mod responses;

use responses::Task;
use sqlx::{sqlite::SqliteQueryResult, Pool, Sqlite};

pub type DBResult<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub async fn create_task(
    pool: &Pool<Sqlite>,
    name: &String,
    description: &String,
) -> DBResult<i64> {
    let mut connection = pool.acquire().await?;
    let id: i64 = sqlx::query_as!(
        Task,
        r#"
        INSERT INTO tasks (name, description) VALUES (?, ?);
        "#,
        name,
        description
    )
    .execute(&mut connection)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn get_task(pool: &Pool<Sqlite>, id: i64) -> DBResult<Task> {
    let mut connection = pool.acquire().await?;
    let task = sqlx::query_as!(
        Task,
        r#"
        SELECT id, name, description from tasks
        WHERE id = ?;
        "#,
        id
    )
    .fetch_one(&mut connection)
    .await?;
    Ok(task)
}

pub async fn get_tasks(pool: &Pool<Sqlite>) -> DBResult<Vec<Task>> {
    let mut connection = pool.acquire().await.unwrap();
    let tasks = sqlx::query_as::<_, Task>(
        r#"
        select id, name, description from tasks;
        "#,
    )
    .fetch_all(&mut connection)
    .await?;
    Ok(tasks)
}

pub async fn update_task(
    pool: &Pool<Sqlite>,
    name: &String,
    description: &String,
    id: i64,
) -> Result<SqliteQueryResult, rocket::response::Debug<sqlx::Error>> {
    let mut connection = pool.acquire().await?;
    let query = format!(
        "update tasks set name ='{}', description='{}' where id = {};",
        name, description, id
    );
    let result: SqliteQueryResult = sqlx::query(&query).execute(&mut connection).await.unwrap();
    Ok(result)
}

pub async fn delete_task(
    pool: &Pool<Sqlite>,
    id: i64,
) -> Result<SqliteQueryResult, rocket::response::Debug<sqlx::Error>> {
    let mut connection = pool.acquire().await?;
    let query = format!("delete from tasks where id = {}", id);
    let result: SqliteQueryResult = sqlx::query(&query).execute(&mut connection).await.unwrap();
    Ok(result)
}
