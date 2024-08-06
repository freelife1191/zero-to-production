use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn health_check() -> impl Responder {
    // let name = req.match_info().get("name").unwrap_or("World");
    // format!("Hello {}!", &name)
    HttpResponse::Ok()
}

// main의 비동기적인 바디를 받아서 필요한 보일러플레이트를 작성하고 tokio의 런타임 위에서 실행함
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
