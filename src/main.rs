use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::models::task::Task;


mod models;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
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
