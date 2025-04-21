use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub done: bool,
    pub deadline: Option<String>
}
