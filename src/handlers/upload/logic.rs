use axum::{Extension, Json, extract::Multipart};
use chrono::Utc;
use serde_json::{Value, json};

use crate::{
    config::storage::{delete_file, upload_file},
    models::user::User,
    utils::error_handling::AppError,
};

pub async fn upload(Extension(current_user): Extension<User>, mut multipart: Multipart) -> Result<Json<Value>, AppError> {
    let mut results = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|err| AppError::BadRequest(err.to_string()))? {
        let field_name = field.name().unwrap_or("");

        if field_name == "files" {
            let file_name = field.file_name().unwrap_or("upload.bin").to_string();
            let unique_file_name = format!("{}-{}", Utc::now().timestamp_millis(), file_name);
            let content_type = field.content_type().unwrap_or("application/octet-stream");

            let file_type = if content_type.starts_with("image/") {
                "image"
            } else if content_type.starts_with("video/") {
                "video"
            } else {
                "other"
            };

            let data = field.bytes().await.map_err(|err| AppError::BadRequest(err.to_string()))?;

            let destination_path = format!("{}/{}", &current_user.id.to_string(), unique_file_name);

            match upload_file(&destination_path, data.to_vec()).await {
                Ok(path) => results.push(json!({
                    "type": file_type,
                    "file_name": file_name,
                    "status": "uploaded",
                    "path":path
                })),
                Err(err) => results.push(json!({
                    "type": file_type,
                    "file_name": file_name,
                    "status": "error",
                    "error": err
                })),
            }
        }
    }

    Ok(Json(json!({
        "results": results
    })))
}

pub async fn delete(Json(payload): Json<Value>) -> Result<Json<Value>, AppError> {
    let path = payload.get("path").ok_or(AppError::BadRequest("Missing path.".into()))?.as_str().unwrap();

    delete_file(path).await.map_err(|_| AppError::BadRequest("Failed to delete file.".into()))?;

    Ok(Json(json!({})))
}
