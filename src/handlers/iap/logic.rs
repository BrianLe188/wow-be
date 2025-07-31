use axum::{Extension, Json, extract::Path};
use chrono::{NaiveDateTime, Utc};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    config::db::{DbConn, DbPool, get_conn},
    handlers::iap::SaveReceiptPayload,
    models::{
        subscription::{NewSubscription, Subscription},
        user::User,
    },
    services::subscription::{create_subscription, get_subscription_by_user},
    utils::error_handling::AppError,
};

const APPLE_PROD_URL: &str = "https://buy.itunes.apple.com/verifyReceipt";
const APPLE_SANDBOX_URL: &str = "https://sandbox.itunes.apple.com/verifyReceipt";

const APPLE_SHARED_SECRET: &str = "";

const IAP_TEST_MODE: bool = true;

fn check_if_has_subscription(subscription: &Subscription) -> bool {
    if subscription.is_cancelled {
        return false;
    }
    let now = Utc::now().naive_utc();
    subscription.start_date <= now && subscription.end_date >= now
}

async fn create_or_update_subscription<'a>(
    conn: &mut DbConn,
    payload: &NewSubscription<'a>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = create_subscription(conn, payload).await {}

    Ok(())
}

async fn validate_ios_receipt(receipt: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = if IAP_TEST_MODE {
        APPLE_SANDBOX_URL
    } else {
        APPLE_PROD_URL
    };

    let payload = serde_json::json!({
        "receipt-data": receipt,
        "password": APPLE_SHARED_SECRET,
        "exclude-old-transactions": true
    });

    let client = reqwest::Client::new();
    let response = client.post(url).json(&payload).send().await?;

    Ok(response.json().await?)
}

async fn process_purchase(
    conn: &mut DbConn,
    app_type: &str,
    user_id: &str,
    receipt: Value,
) -> Result<(), Box<dyn std::error::Error>> {
    match app_type {
        "ios" => {
            let receipt_data = receipt.as_str().ok_or("Invalid iOS receipt")?;
            let validation_response = validate_ios_receipt(receipt_data).await?;

            let latest_receipt_info = &validation_response["latest_receipt_info"][0];
            let validation_response_str = serde_json::to_string(&validation_response.clone())?;

            let subscription = NewSubscription {
                app: app_type,
                environment: if IAP_TEST_MODE {
                    "sandbox"
                } else {
                    "production"
                },
                user_id: Uuid::parse_str(user_id)?,
                orig_tx_id: latest_receipt_info["original_transaction_id"]
                    .as_str()
                    .unwrap_or_default(),
                latest_receipt: validation_response["latest_receipt"]
                    .as_str()
                    .unwrap_or_default(),
                validation_response: validation_response_str.as_str(),
                start_date: NaiveDateTime::from_timestamp_millis(
                    latest_receipt_info["original_purchase_date_ms"]
                        .as_str()
                        .unwrap_or("0")
                        .parse::<i64>()?,
                )
                .unwrap_or_default(),
                end_date: NaiveDateTime::from_timestamp_millis(
                    latest_receipt_info["expires_date_ms"]
                        .as_str()
                        .unwrap_or("0")
                        .parse::<i64>()?,
                )
                .unwrap_or_default(),
                product_id: latest_receipt_info["product_id"]
                    .as_str()
                    .unwrap_or_default(),
                is_cancelled: false,
                fake: false,
            };

            create_or_update_subscription(conn, &subscription).await?;
        }
        "android" => {}
        _ => return Err("Unsupported app type".into()),
    }

    Ok(())
}

pub async fn save_receipt(
    Extension(pool): Extension<DbPool>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<SaveReceiptPayload>,
) -> Result<Json<Value>, AppError> {
    let user_id = current_user.id;

    let app_type = payload.app_type;
    let purchase = payload.purchase;

    let receipt = match app_type.as_str() {
        "ios" => purchase["transactionReceipt"].clone(),
        "android" => json!({
        "packageName": "",
        "productId": "",
        "purchaseToken": "",
        "subscription": true,
            }),
        _ => return Err(AppError::BadRequest("Invalid app type.".into())),
    };

    let mut conn = get_conn(&pool).await.map_err(AppError::BadRequest)?;

    process_purchase(&mut conn, &app_type, user_id.to_string().as_str(), receipt)
        .await
        .map_err(|_| AppError::BadRequest("Failed to process purchase.".into()))?;

    Ok(Json(json!({})))
}

pub async fn get_user_subscription(
    Extension(pool): Extension<DbPool>,
    Extension(current_user): Extension<User>,
    Path(app_type): Path<String>,
) -> Result<Json<Value>, AppError> {
    let user_id = current_user.id;

    let mut conn = get_conn(&pool).await.map_err(AppError::BadRequest)?;

    let user_id_str = user_id.to_string();

    let subscription = get_subscription_by_user(&mut conn, user_id_str.as_str(), app_type.as_str())
        .await
        .map_err(|_| AppError::BadRequest("Subscription not existing.".into()))?;

    let has_subscription = check_if_has_subscription(&subscription);

    if !has_subscription {
        return Err(AppError::BadRequest(
            "Subscription has been cancelled.".into(),
        ));
    }

    Ok(Json(json!({
        "subscription": subscription.clone(),
        "has_subscription": has_subscription
    })))
}
