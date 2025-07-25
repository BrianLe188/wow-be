use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use std::env;

pub async fn upload_file(destination_path: &str, buffer: Vec<u8>) -> Result<String, String> {
    let storage_url = env::var("STORAGE_URL").map_err(|err| err.to_string())?;
    let storage_bucket_name = env::var("STORAGE_BUCKET_NAME").map_err(|err| err.to_string())?;
    let storage_anon_key = env::var("STORAGE_ANON_KEY").map_err(|err| err.to_string())?;

    let upload_url = format!(
        "{}/storage/v1/object/{}/{}",
        storage_url, storage_bucket_name, destination_path
    );

    let client = Client::new();

    let response = client
        .post(&upload_url)
        .header(AUTHORIZATION, format!("Bearer {}", storage_anon_key))
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(buffer)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.status().is_success() {
        Ok(destination_path.to_owned())
    } else {
        let error_text = response.text().await.map_err(|err| err.to_string())?;
        Err(format!("Upload failed: {}", error_text))
    }
}

pub async fn delete_file(path: &str) -> Result<(), String> {
    let storage_url = env::var("STORAGE_URL").map_err(|err| err.to_string())?;
    let storage_bucket_name = env::var("STORAGE_BUCKET_NAME").map_err(|err| err.to_string())?;
    let storage_anon_key = env::var("STORAGE_ANON_KEY").map_err(|err| err.to_string())?;

    let delete_url = format!(
        "{}/storage/v1/object/{}/{}",
        storage_url, storage_bucket_name, path
    );

    let client = Client::new();

    let response = client
        .delete(&delete_url)
        .header(AUTHORIZATION, format!("Bearer {}", storage_anon_key))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_text = response.text().await.map_err(|err| err.to_string())?;
        Err(format!("Delete failed: {}", error_text))
    }
}
