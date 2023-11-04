use crate::models::task::{NewTask, Task};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;
use std::env;

mod models;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

async fn get_tasks() -> impl Responder {
    HttpResponse::Ok().json(vec!["Task 1", "Task 2", "Task 3"])
}

async fn create_task(pool: web::Data<sqlx::PgPool>, task: web::Json<NewTask>) -> impl Responder {
    let rec = sqlx::query!(
        "INSERT INTO tasks (title, description) VALUES ($1, $2) RETURNING id",
        task.title,
        task.description
    )
    .fetch_one(pool.get_ref())
    .await
    .expect("Failed to execute query.");

    HttpResponse::Created().json(Task {
        id: rec.id as u32,
        title: task.title.clone(),
        description: task.description.clone(),
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
            .route("/", web::get().to(greet))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
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
