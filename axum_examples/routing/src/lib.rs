mod routes;

use routes::creating_routes;

pub async fn run() {
    let app = creating_routes();
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}