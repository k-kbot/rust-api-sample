use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::models::task::Task;

mod models;

async fn greet() -> impl Responder {
    "Hello!"
}

async fn get_tasks() -> impl Responder {
    HttpResponse::Ok().json(vec![
        "Task 1", "Task 2", "Task 3"
    ])
}

async fn create_task(task: web::Json<Task>) -> impl Responder {
    println!("Creating new task: {:?}", task);
    HttpResponse::Created().json(task.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
