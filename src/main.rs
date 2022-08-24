use actix_web::{web, App, HttpServer, Responder, Result};


async fn mine_block() -> Result<impl Responder> {
    Ok(web::Json({}))
}


async fn display_chain() -> Result<impl Responder> {
    Ok(web::Json({}))
}


async fn valid() -> Result<impl Responder> {
    Ok(web::Json({}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting node on port 8080 ...");
    HttpServer::new(|| {
        App::new()
            .route("/get_chain", web::get().to(display_chain))
            .route("/mine_block", web::get().to(mine_block))
            .route("/valid", web::get().to(valid))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}