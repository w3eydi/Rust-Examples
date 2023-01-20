use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files::Files;

use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Kedidex",
        "cats": [
            {
                "name": "şapşal",
                "image_path": "/static/image/komik_sapsal.jpeg"
            },
            {
                "name": "vaşak",
                "image_path": "/static/image/vasak.jpeg"
            }
        ]
    });

    let content = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(content)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static"))
            .route("/", web::get().to(index))     
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
