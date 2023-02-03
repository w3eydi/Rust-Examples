use axum::Json;
use serde::Serialize;

#[derive(Serialize, serde::Deserialize, Debug)]
pub struct MirrorJson {
    message: String
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String
}

pub async fn mirror_body_json(Json(merhaba): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    Json(MirrorJsonResponse { message: merhaba.message, message_from_server: "hello from Axum".to_owned() })
}