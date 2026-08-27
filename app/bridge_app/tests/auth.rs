use axum::body::Body;
use axum::http::{Request, StatusCode};
use bridge_app::dto;
use serde_json::json;
use tower::ServiceExt; // для .oneshot()

#[tokio::test]
async fn get_nonce() {
    dotenv::from_filename(".env.test").ok();
    let app = bridge_app::build_app_default().await;
    //let localvalidator = bridge_app::local::run_evm_local_validator();
    bridge_app::init_tracing();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/generate_verify")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&json!(dto::auth::RandomNonceReq {
                        address: "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97".to_string(),
                        rand_nonce: 123123,
                        rand_bytes_arr: [0; 32],
                    }))
                    .expect("0"),
                ))
                .expect("1"),
        )
        .await
        .expect("pidor jopa");

    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("failed to read body");
    let body_str = String::from_utf8_lossy(&bytes);
    println!("status: {status}");
    println!("body: {body_str}");
    assert_eq!(status, StatusCode::OK, "response body was: {body_str}");
}
