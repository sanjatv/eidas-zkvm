use host::{ create_zkp_age_over_18, verify_age_over_18 };
use risc0_zkvm::Receipt;
use serde::{ Deserialize, Serialize };
use axum::{ Router, extract::Json, http::{ self, Method }, routing::post };
use tower_http::cors::{ Any, CorsLayer };

#[derive(Deserialize, Serialize)]
pub struct ApiVerifyRequest {
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct ApiVerifyResponse {
    pub is_valid: bool,
    // pub receipt: Receipt,
}

async fn http_verify_age_over_18(Json(req): Json<ApiVerifyRequest>) -> Json<ApiVerifyResponse> {
    let receipt: Receipt = create_zkp_age_over_18(&req.jwt).unwrap();
    let (is_valid, _) = verify_age_over_18(receipt).unwrap();
    Json(ApiVerifyResponse { is_valid })
}

pub async fn start_server() {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<http::HeaderValue>().unwrap())
        .allow_methods([Method::POST])
        .allow_headers(Any);
    // Create a new route 'verify' (post) to the router
    // f.eks. kaller POST http://localhost ../verify kommer til app
    // post(verify_http): denne ruten svarer på post requests. når en post request kommer på /verify, kall verify_http
    //let app = Router::new().route("/verify", post(http_verify_age_over_18));
    let app = Router::new().route("/verify", post(http_verify_age_over_18)).layer(cors);

    // lag TCP server som lytter på en nettverksadresse
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// NEXT: les https://docs.shuttle.dev/templates/tutorials/rest-http-service-with-axum
