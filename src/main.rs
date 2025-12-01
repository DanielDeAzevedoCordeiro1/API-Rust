use std::collections::HashMap;
use std::sync::{Arc};
use axum::{routing::{get, post}, Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
struct UserRequest {
    name: String,
    age: u8,
}

#[derive(Clone, Serialize, Deserialize)]
struct UserResponse {
    payload: Option<User>,
    status: u16,
    error: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub age: u8,
}


type MemoryDB = Arc<RwLock<HashMap<u16, User>>>;
type ResponseReturn = (StatusCode, Json<UserResponse>);
type ListResponseReturn = (StatusCode, Json<Vec<UserResponse>>);

fn new_db() -> MemoryDB{
    Arc::new(RwLock::new(HashMap::new()))
}
#[tokio::main]
async fn main() {
    let memory_db: MemoryDB = new_db();

    let app = Router::new()
        .route("/user/{id}", get(get_user))
        .route("/all", get(all))
        .route("/user", post(create_user))
        .with_state(memory_db);

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:9000")
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();
}



async fn get_user(
    State(state): State<MemoryDB>,
    Path(id): Path<u16>
) -> ResponseReturn{
    let reader = state.read().await;

    match reader.get(&id) {
        Some(v) => (StatusCode::OK, Json(UserResponse { 
            payload: Some(v.clone()), 
            status: StatusCode::OK.as_u16(),
            error: None
        })),
        None => (StatusCode::NOT_FOUND, Json(UserResponse {
            payload: None,
            status: StatusCode::NOT_FOUND.as_u16(),
            error: Some("Usuario nao encontrado!".to_string())
        }))
    }
}

async fn create_user(
    State(state): State<MemoryDB>,
    Json(payload): Json<UserRequest>
) -> ResponseReturn{
    
    let mut writer = state.write().await;

    let user = User{
        id: Uuid::now_v7().to_string(),
        name: payload.name,
        age: payload.age
    };

    let index: u16 = rand::rng().random();

    writer.insert(index, user.clone());

    (StatusCode::CREATED, Json(UserResponse{
        payload: Some(user),
        status: StatusCode::CREATED.as_u16(),
        error: None
    }))
}

async fn all(
    State(state): State<MemoryDB>
) -> ListResponseReturn {
    let reader = state.read().await;

    let users = reader
        .values()
        .cloned()
        .map(|u| UserResponse { payload: Some(u), status: StatusCode::OK.as_u16(), error: None })
        .collect::<Vec<UserResponse>>();

    (StatusCode::OK, Json(users))
}
