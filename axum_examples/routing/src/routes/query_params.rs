use axum::{extract::Query, Json};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct QueryParams {
    message: String,
    id: u32,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}