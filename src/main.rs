use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    "Hello!"
}

async fn get_tasks() -> impl Responder {
    HttpResponse::Ok().json(vec![
        "Task 1", "Task 2", "Task 3"
    ])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/tasks", web::get().to(get_tasks))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
