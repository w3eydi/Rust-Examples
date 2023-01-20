use actix_web::{web, App, HttpServer, Result};
use actix_files::{NamedFile, Files};

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
