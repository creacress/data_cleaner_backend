mod handlers;

use actix_files::Files;
use actix_web::{web, App, HttpServer};

use handlers::{
    upload::upload_csv,
    preview::preview_csv,
    clean::clean_csv,
    analysis::analyze_csv,
    transform::transform_csv,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Serveur lancé sur http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_csv))
            .route("/preview", web::get().to(preview_csv))
            .route("/clean", web::get().to(clean_csv))
            .route("/analyze", web::get().to(analyze_csv))
            .route("/transform", web::get().to(transform_csv))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
