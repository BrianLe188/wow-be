#[cfg(test)]
mod test {
    use axum::{
        Router,
        body::Body,
        http::{
            Method, Request, StatusCode,
            header::{AUTHORIZATION, CONTENT_TYPE},
        },
    };
    use dotenvy::dotenv;
    use http_body_util::BodyExt;
    use tokio::sync::OnceCell;
    use tower::ServiceExt;

    use crate::config::app::init_test_app;

    static ACCESS_TOKEN: OnceCell<String> = OnceCell::const_new();

    async fn sign_in(app: Router) -> String {
        let payload = serde_json::json!({
            "email": "anh@gmail.com",
            "password": "123123123123"
        });

        let payload_str = serde_json::to_string(&payload).unwrap();

        let request = Request::builder()
            .method(Method::POST)
            .uri("/auth/sign-in")
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(payload_str))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        println!("{:?}", body);

        body["access_token"].as_str().unwrap().to_string()
    }

    async fn get_access_token(app: Router) -> String {
        ACCESS_TOKEN
            .get_or_init(|| async { sign_in(app).await })
            .await
            .to_string()
    }

    #[tokio::test]
    async fn test_success_path_response() {
        dotenv().ok();

        let app = init_test_app().await;

        let access_token = get_access_token(app.clone()).await;

        let payload = serde_json::json!({
            "origin": [
                16.07909,
                108.1784457
            ],
            "waypoints": [
                [
                    [
                        108.1724962,
                        16.0143747
                    ],
                    [
                        108.1935533,
                        16.0488041
                    ]
                ],
                [
                    [
                        108.2123137,
                        16.0371405
                    ]
                ]
            ]
        });

        let payload_str = serde_json::to_string(&payload).unwrap();

        let request = Request::builder()
            .method(Method::POST)
            .uri("/waypoints")
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(payload_str))
            .unwrap();

        let response = match app.oneshot(request).await {
            Ok(rs) => rs,
            Err(err) => {
                eprintln!("response: {}", err);
                return;
            }
        };

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let expected_result = serde_json::json!({
              "path": [
                [
                  108.1724962,
                  16.0143747
                ],
                [
                  108.1935533,
                  16.0488041
                ],
                [
                  108.2123137,
                  16.0371405
                ]
              ]
        });

        assert_eq!(body, expected_result);
    }

    #[tokio::test]
    async fn test_unauthorized_request() {
        dotenv().ok();

        let app = init_test_app().await;

        let payload = serde_json::json!({
            "origin": [16.07909, 108.1784457],
            "waypoints": [[[108.1724962, 16.0143747]]]
        });

        let request = Request::builder()
            .method(Method::POST)
            .uri("/waypoints")
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_missing_origin_field() {
        dotenv().ok();

        let app = init_test_app().await;

        let access_token = get_access_token(app.clone()).await;

        let payload = serde_json::json!({
            "waypoints": [[]]
        });

        let request = Request::builder()
            .method(Method::POST)
            .uri("/waypoints")
            .header(AUTHORIZATION, format!("Bearer {}", access_token))
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
