use crate::models::Todo;
use std::fs;
use std::path::Path;

// define storage to process data
// use trait as interface
pub trait Storage<T> {
    fn load(&self) -> Result<Vec<T>, String>;
    fn save(&self, items: &[T]) -> Result<(), String>;
}

pub struct JsonStorage {
    pub path: String,
}

impl Storage<Todo> for JsonStorage {
    fn load(&self) -> Result<Vec<Todo>, String> {
        if !Path::new(&self.path).exists() {
            return Ok(vec![]);
        }

        //let content = fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
        let content = match fs::read_to_string(&self.path) {
            Ok(content) => content,
            Err(e) => return Err(e.to_string())
        };

        // let todos = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        let todos = match serde_json::from_str(&content) {
            Ok(todos) => todos,
            Err(e) => return Err(e.to_string())
        };
        Ok(todos)
    }

    fn save(&self, items: &[Todo]) -> Result<(), String> {
        // let content = serde_json::to_string_pretty(items).map_err(|e| e.to_string())?;
        let content=match serde_json::to_string_pretty(items){
            Ok(content)=>content,
            Err(e)=>return Err(e.to_string())
        };
        
        // fs::write(&self.path, content).map_err(|e| e.to_string())
        match fs::write(&self.path, content){
            Ok(_)=>Ok(()),
            Err(e   )=>Err(e.to_string())
        }
    }
}   

