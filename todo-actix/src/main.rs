use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;


#[derive(Serialize, Deserialize, Clone)]

struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]

struct CreateTodo {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

type TodoStore = Mutex<HashMap<u32, Todo>>;
type IdCounter = Mutex<u32>;


async fn get_todos(store: web::Data<TodoStore>) -> Result<HttpResponse> {
    let todos = store.lock().unwrap();
    let todo_list: Vec<Todo> = todos.values().cloned().collect();
    return Ok(HttpResponse::Ok().json(todo_list));
}

async fn create_todo(
    todo: web::Json<CreateTodo>,
    store: web::Data<TodoStore>,
    counter: web::Data<IdCounter>,
) -> Result<HttpResponse> {

    
    let mut todos = store.lock().unwrap();
    let mut id = counter.lock().unwrap();

    //it’s a smart pointer to the actual u32 inside the mutex.
    *id += 1;
    
    let new_todo = Todo {
        id: *id,
        title: todo.title.clone(),
        completed: false,
    };

    todos.insert(*id, new_todo.clone());
    Ok(HttpResponse::Created().json(new_todo))
}

async fn get_todo(path: web::Path<u32>, store: web::Data<TodoStore>) -> Result<HttpResponse> {
    let todos = store.lock().unwrap();
    let todo_id = path.into_inner();

    match todos.get(&todo_id) {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "Todo not found"})))
    }
}


async fn update_todo(
    path: web::Path<u32>,
    update: web::Json<UpdateTodo>,
    store: web::Data<TodoStore>,
) -> Result <HttpResponse> {
    let mut todos = store.lock().unwrap();
    let todo_id = path.into_inner();

    match todos.get_mut(&todo_id) {
        Some(todo) =>{
            if let Some(title) = &update.title {
                todo.title = title.clone();
            }
            if let Some(completed) = update.completed{
                todo.completed = completed;
            }
            Ok(HttpResponse::Ok().json(todo.clone()))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "Todo not found"})))
    }
}

async fn delete_todo(path: web::Path<u32>, store: web::Data<TodoStore>) -> Result<HttpResponse> {
    let mut todos = store.lock().unwrap();
    let todo_id = path.into_inner();

    match todos.remove(&todo_id) {
        Some(_) => Ok(HttpResponse::Ok().json(serde_json::json!({"message": "Todo deleted"}))),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({"error": "Todo not found"})))
    }
}
//procedural macro that sets up the async runtime
#[actix_web::main]

async fn main() -> std::io::Result<()> {

    let todo_store = web::Data::new(TodoStore::new(HashMap::new()));
    let id_counter = web::Data::new(IdCounter::new(0));


    HttpServer::new(move || {
        App::new()
        .app_data(todo_store.clone())
        .app_data(id_counter.clone())
        .route("/todos", web::get().to(get_todos))
        .route("/todos", web::post().to(create_todo))
        .route("/todos/{id}", web::get().to(get_todo))
        .route("/todos/{id}", web::put().to(update_todo))
        .route("/todos/{id}", web::delete().to(delete_todo))
    })
    //? Instead of crashing, it gracefully returns the error to the caller 
    .bind("127.0.0.1:8080")?
    .run()
    .await
}