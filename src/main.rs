use console::Term;
use ctrlc;
use std::cell::RefCell;
use todo_rust::cli::{display_selection, print_message, prompt_action, read_line, Action, Message};
use todo_rust::todo::Todos;

fn main() {
    setup_ctrlc();

    let mut todos = Todos::new();
    let _ = todos.deserialize();

    let todos_rc = RefCell::new(todos);
    let mut actions = get_actions(&todos_rc);

    print_message(Message::Title);

    loop {
        if prompt_action(&mut actions, Message::WhatToDo).is_none() {
            print_message(Message::NothingChosen);
            break;
        }
    }
}

fn get_actions(todos: &RefCell<Todos>) -> Vec<Action> {
    let actions = vec![
        Action {
            label: "List all todos",
            action: Some(Box::new(|| {
                let list_todos = || -> Option<usize> {
                    let borrowed_todos = todos.borrow();
                    let titles: Vec<&str> = borrowed_todos
                        .get_all()
                        .iter()
                        .map(|todo| todo.title.as_str())
                        .collect();

                    if titles.len() == 0 {
                        print_message(Message::NoTodos);
                        return None;
                    }

                    let result = display_selection(&titles).ok().flatten();
                    let index = match result {
                        Some(index) => index,
                        None => return None,
                    };

                    let todo_at_index = borrowed_todos.get(index);
                    print_message(Message::TodoInformation(todo_at_index));

                    Some(index)
                };

                let selected_index = list_todos();
                if let Some(selected_index) = selected_index {
                    let mut todo_actions = [
                        Action {
                            label: "Mark as done",
                            action: Some(Box::new(|| {
                                let mut borrowed_todos = todos.borrow_mut();

                                let todo_id = borrowed_todos.get(selected_index).id.clone();
                                borrowed_todos.mark_as_done(&todo_id);
                            })),
                        },
                        Action {
                            label: "Delete",
                            action: Some(Box::new(|| {
                                let mut borrowed_todos = todos.borrow_mut();

                                let todo_id = borrowed_todos.get(selected_index).id.clone();
                                borrowed_todos.remove(&todo_id);
                            })),
                        },
                        Action {
                            label: "Go to menu",
                            action: None,
                        },
                    ];

                    prompt_action(&mut todo_actions, Message::WhatToDoWithTodo);
                }
            })),
        },
        Action {
            label: "Create a new todo",
            action: Some(Box::new(|| {
                let mut borrowed_todos = todos.borrow_mut();

                print_message(Message::PromptTitle);
                let title = match read_line() {
                    Some(line) => line.trim().to_owned(),
                    None => return,
                };

                print_message(Message::PromptDescription);
                let description = match read_line() {
                    Some(line) => line.trim().to_owned(),
                    None => return,
                };

                borrowed_todos.add(&title, &description);
                print_message(Message::TodoAdded);
            })),
        },
    ];

    actions
}

fn setup_ctrlc() {
    let term = Term::stdout();

    // here we have to setup a Ctrl-C handler
    // because if an user interrupts the process
    // while a dialoguer::Select is visible, the cursor will be hidden
    // so we have to manually call the show_cursor method to make it visible
    ctrlc::set_handler(move || {
        term.show_cursor().unwrap();
    })
    .unwrap();
}
