use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    http,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router, TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/protected", post(protected));

    println!("the server is running on http://127.0.0.1:8100");
    axum::Server::bind(&"127.0.0.1:8100".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
}

async fn login(Json(request): Json<LoginRequest>) -> Json<String> {
    println!("request: {:?}", request);
    let name = request.username;
    let pwd = request.password;
    let my_claims = Claims {
        name,
        pwd,
        exp: 9999999999,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();
    Json(token)
}

async fn protected(claims: Claims, Json(request): Json<ProtectedRequest>) -> Html<String> {
    println!("claims: {:?}", claims);
    println!("other: {:?}", request.test12);
    Html("authorized".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String,
    pwd: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProtectedRequest {
    test12: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthRequest {
    token: String,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send, // required by `async_trait`
{
    type Rejection = MyError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| MyError::Auth)?;
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .map_err(|_| MyError::Internal)?; //
        Ok(token_data.claims)
    }
}

#[derive(Debug)]
enum MyError {
    Auth,
    Internal,
}

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            MyError::Auth => (http::StatusCode::UNAUTHORIZED, "Unauthorized"),
            MyError::Internal => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error",
            ),
        };
        (code, msg).into_response()
    }
}
