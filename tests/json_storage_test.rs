use todo_cli::storage::{JsonStorage, Storage};
use todo_cli::models::Todo;

#[test]
fn test_load_todos() {
    let storage = JsonStorage {
        path: "todos.json".to_string(),
    };

    let todos    = storage.load();
    assert!(todos.is_ok());
    assert_eq!(todos.unwrap().len(), 0);
}

    
#[test]
fn test_save_todos() {
    let storage = JsonStorage {
        path: "todos.json".to_string(),
    };

    let todos = vec![Todo {
        id: 1,
        title: "Test".to_string(),
        done: false,
        deadline: Some("2025-04-30".to_string()),
    }];

    let result = storage.save(&todos);
    assert!(result.is_ok());
}