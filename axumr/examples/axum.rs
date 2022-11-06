use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{self, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router, TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    let store = TodoStrore::default();
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/create", post(todo_create).layer(Extension(store.clone())))
        .route(
            "/todo",
            post(get_todo_infos).layer(Extension(store.clone())),
        );

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    name: String,
    done: bool,
}

#[derive(Default, Debug, Clone)]
struct TodoStrore {
    todos: Arc<Mutex<Vec<Todo>>>,
}

async fn todo_create(claims: Claims, state: Extension<TodoStrore>) -> Result<StatusCode, MyError> {
    let store = state.0;
    let todo = Todo {
        id: get_id(),
        name: claims.name,
        done: false,
    };
    store.todos.lock().unwrap().push(todo);
    Ok(StatusCode::CREATED)
}

fn get_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

async fn get_todo_infos(
    _claims: Claims,
    Json(_request): Json<ProtectedRequest>,
    state: Extension<TodoStrore>,
) -> Result<Json<Vec<Todo>>, MyError> {
    Ok(Json(state.todos.lock().unwrap().clone()))
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
