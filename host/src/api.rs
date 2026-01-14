use host::{ verify_jwt_signature };
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

async fn verify_http(Json(req): Json<ApiVerifyRequest>) -> Json<ApiVerifyResponse> {
    let (is_valid, _) = verify_jwt_signature(&req.jwt).unwrap();
    Json(ApiVerifyResponse { is_valid })
}

pub async fn start_server() {
    // Create a new route 'verify' (post) to the router
    // f.eks. kaller POST http://localhost ../verify kommer til app
    // post(verify_http): denne ruten svarer på post requests. når en post request kommer på /verify, kall verify_http
    let app = Router::new().route("/verify", post(verify_http));

    // lag TCP server som lytter på en nettverksadresse
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

// NEXT: les https://docs.shuttle.dev/templates/tutorials/rest-http-service-with-axum
