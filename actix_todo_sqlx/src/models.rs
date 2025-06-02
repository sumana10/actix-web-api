use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

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
