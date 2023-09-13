use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod models {
    pub mod model_task; // Importa o módulo do arquivo models/model_task.rs
}

use models::model_task::Task;

#[get("/")]
async fn hello() -> impl Responder {
    let task = Task {
        id: 1,
        title: "Compilar".to_string(),
        description: "Compilar esse código".to_string(),
        due_date: "2023-09-20".to_string(),
        priority: "Média".to_string(),
        completed: false,
        category: "Compras".to_string(),
    };
    HttpResponse::Ok().json(task)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 3333))?
    .run()
    .await
}
