use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

pub async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Hello, World!",
        "status": "success"
    })))
}

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

pub async fn get_users() -> Result<HttpResponse> {
    let users = vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        },
    ];
    
    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let user = User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    };
    
    Ok(HttpResponse::Ok().json(user))
}

pub async fn create_user(user_data: web::Json<CreateUser>) -> Result<HttpResponse> {
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    let new_user = User {
        id: 999, 
        name: user_data.name.clone(),
        email: user_data.email.clone(),
    };
    
    Ok(HttpResponse::Created().json(new_user))
}

pub async fn delete_user(path: web::Path<u32>) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": format!("User {} deleted successfully", user_id),
        "deleted_id": user_id
    })))
}
