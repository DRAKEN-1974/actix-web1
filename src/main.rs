use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Clone)]
struct User {
    id: usize,
    name: String,
    email: String,
}

struct AppState {
    users: Mutex<Vec<User>>,
}

// GET /users - get all users
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(&*users)
}

// POST /users - create a new user
async fn create_user(data: web::Data<AppState>, new_user: web::Json<User>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.push(new_user.into_inner());
    HttpResponse::Created().body("User created")
}

// GET /users/{id} - get user by id
async fn get_user_by_id(data: web::Data<AppState>, user_id: web::Path<usize>) -> impl Responder {
    let users = data.users.lock().unwrap();
    let id = user_id.into_inner();
    match users.iter().find(|user| user.id == id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

// PUT /users/{id} - update user by id
async fn update_user(
    data: web::Data<AppState>,
    user_id: web::Path<usize>,
    updated_user: web::Json<User>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let id = user_id.into_inner();

    match users.iter_mut().find(|user| user.id == id) {
        Some(user) => {
            user.name = updated_user.name.clone();
            user.email = updated_user.email.clone();
            HttpResponse::Ok().body("User updated")
        }
        None => HttpResponse::NotFound().body("User not found"),
    }
}

// DELETE /users/{id} - delete user by id
async fn delete_user(data: web::Data<AppState>, user_id: web::Path<usize>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let id = user_id.into_inner();

    let len_before = users.len();
    users.retain(|user| user.id != id);

    if users.len() == len_before {
        HttpResponse::NotFound().body("User not found")
    } else {
        HttpResponse::Ok().body("User deleted")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
