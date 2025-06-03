use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Serialize, Deserialize, FromRow)] // Add FromRow
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "lowercase")] // For JSON (HTTP requests/responses)
#[sqlx(type_name = "text", rename_all = "lowercase")] // For database conversion
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}
//implementing default trait of priority enum
impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<Priority>,
}

#[derive(Debug, Serialize)]
pub struct TodoStats {
    pub total: i64,
    pub completed: i64,
    pub pending: i64,
    pub by_priority: PriorityStats,
}

#[derive(Debug, Serialize)]
pub struct PriorityStats {
    pub low: i64,
    pub medium: i64,
    pub high: i64,
    pub urgent: i64,
}
