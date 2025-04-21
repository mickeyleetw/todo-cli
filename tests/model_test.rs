use todo_cli::models::Todo;

#[test]
fn test_create_todo() {
    let todo = Todo {
        id: 1,
        title: "Test".into(),
        done: false,
        deadline: Some("2025-04-30".into()),
    };

    assert_eq!(todo.id, 1);
    assert_eq!(todo.done, false);
    assert_eq!(todo.deadline.as_deref(), Some("2025-04-30"));
}


#[test]
fn test_create_todo_without_deadline() {
    let todo = Todo {
        id: 1,
        title: "Test".into(),
        done: false,
        deadline: None,
    };

    assert_eq!(todo.id, 1);
    assert_eq!(todo.done, false);
    assert_eq!(todo.deadline, None);
}   


#[test]
fn test_create_todo_with_empty_title() {
    let todo = Todo {
        id: 1,
        title: "".into(),
        done: false,
        deadline: Some("2025-04-30".into()),
    };

    assert_eq!(todo.id, 1);
    assert_eq!(todo.done, false);
    assert_eq!(todo.title, String::from(""));
}



