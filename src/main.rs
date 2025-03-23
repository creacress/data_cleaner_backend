mod handlers;

use actix_files::Files;
use actix_web::{App, HttpServer, web};
use handlers::upload_csv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Serveur lancé sur http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_csv))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
