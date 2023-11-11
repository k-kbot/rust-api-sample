use crate::models::task::{NewTask, Status, Task};
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;
use std::env;

mod models;

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/tasks")]
async fn get_tasks(pool: Data<Pool<Postgres>>) -> impl Responder {
    let tasks = find_tasks(&pool).await;
    match tasks {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn find_tasks(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Task>, sqlx::Error> {
    let mut tasks = Vec::new();

    let rows = sqlx::query("SELECT id, title, description, status FROM tasks")
        .fetch_all(pool)
        .await?;

    for row in rows {
        let task = Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
        };
        tasks.push(task);
    }

    Ok(tasks)
}

#[post("/tasks")]
async fn create_task(pool: web::Data<sqlx::PgPool>, task: web::Json<NewTask>) -> impl Responder {
    let rec = sqlx::query!(
        "INSERT INTO tasks (title, description, status) VALUES ($1, $2, $3) RETURNING id",
        task.title,
        task.description,
        Status::Todo.value(),
    )
    .fetch_one(pool.get_ref())
    .await
    .expect("Failed to execute query.");

    HttpResponse::Created().json(Task {
        id: rec.id,
        title: task.title.clone(),
        description: task.description.clone(),
        status: Status::Todo,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db_pool: Pool<Postgres> = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(greet)
            .service(get_tasks)
            .service(create_task)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// tests
#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{test, web::Bytes};

    #[actix_rt::test]
    async fn test_greet() {
        let mut app = test::init_service(App::new().route("/", web::get().to(greet))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, Bytes::from_static(b"Hello!"));
    }
}
