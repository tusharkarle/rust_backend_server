#[macro_use]
extern crate rocket;
mod database;

use database::requests::TaskRequest;
use database::responses::Task;
use database::{create_task, delete_task, get_task, get_tasks, update_task, DBResult};
use dotenvy;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{Pool, Sqlite, SqlitePool};

#[post("/tasks", format = "json", data = "<task>")]
async fn create(task: Json<TaskRequest>, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Task>> {
    let id: i64 = create_task(pool, &task.name, &task.description).await?;
    let task = get_task(pool, id).await?;
    Ok(Json(task))
}

#[get("/tasks")]
async fn index(pool: &State<Pool<Sqlite>>) -> DBResult<Json<Vec<Task>>> {
    let tasks = get_tasks(pool).await?;
    Ok(Json(tasks))
}

#[get("/tasks/<id>")]
async fn detail(id: i64, pool: &State<Pool<Sqlite>>) -> DBResult<Json<Task>> {
    let task = get_task(pool, id).await?;
    Ok(Json(task))
}

#[patch("/tasks/<id>", format = "json", data = "<task>")]
async fn update_data(
    id: i64,
    task: Json<TaskRequest>,
    pool: &State<Pool<Sqlite>>,
) -> DBResult<Json<Task>, String> {
    let response = update_task(pool, &task.name, &task.description, id).await;
    let updated_record = get_task(pool, id).await;
    if response.is_err() || updated_record.is_err() {
        return DBResult::Err("Internal Server Err".to_string());
    }
    Ok(Json(updated_record.unwrap()))
}

#[delete("/tasks/<id>")]
async fn delete_tasks(id: i64, pool: &State<Pool<Sqlite>>) -> DBResult<String, String> {
    let response = delete_task(pool, id).await;
    if response.is_err() {
        return DBResult::Err("Internal Server Err".to_string());
    } else if response.unwrap().rows_affected() == 0 {
        return DBResult::Ok("No record found to delete".to_string());
    } else {
        return DBResult::Ok(format!("Record With id {} deleted", id).to_string());
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let database_url = dotenvy::var("DATABASE_URL");
    let pool = SqlitePool::connect(database_url.unwrap().as_str())
        .await
        .expect("Couldn't connect to sqlite database");
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![index, detail, create, delete_tasks, update_data],
        )
        .manage(pool)
        .launch()
        .await?;
    Ok(())
}
