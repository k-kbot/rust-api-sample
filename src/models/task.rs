use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Todo = 0,
    Done = 1,
}

impl TaskStatus {
    pub fn value(&self) -> i32 {
        match *self {
            TaskStatus::Todo => 0,
            TaskStatus::Done => 1,
        }
    }
}
