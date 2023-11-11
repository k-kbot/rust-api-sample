use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "integer")]
#[repr(i32)]
pub enum Status {
    Todo = 0,
    Done = 1,
}

impl Status {
    pub fn value(&self) -> i32 {
        match *self {
            Status::Todo => 0,
            Status::Done => 1,
        }
    }
}
