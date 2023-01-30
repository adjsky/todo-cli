use std::cell::RefCell;
use todo_rust::cli::{display_selection, print_message, prompt_action, Action, Message};
use todo_rust::todo::Todos;

fn main() {
    let todos = RefCell::new(Todos::new());
    let mut actions = [
        Action {
            label: "List all todos",
            action: Box::new(|| {
                let borrowed_todos = todos.borrow();
                let titles: Vec<&str> = borrowed_todos
                    .get_all()
                    .iter()
                    .map(|todo| todo.title.as_str())
                    .collect();

                if titles.len() == 0 {
                    print_message(Message::NoTodos);
                } else {
                    print_message(Message::HereYourTodos);
                    let result = display_selection(&titles).ok().flatten();
                }
            }),
        },
        Action {
            label: "Create a new todo",
            action: Box::new(|| {
                let borrowed_todos = todos.borrow_mut();
            }),
        },
    ];

    print_message(Message::Title);

    loop {
        if prompt_action(&mut actions).is_none() {
            print_message(Message::NothingChosen);
            break;
        }
    }
}
