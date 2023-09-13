use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub priority: String,
    pub completed: bool,
    pub category: String,
}
