use std::cell::RefCell;
use todo_rust::cli::{print_catching_message, prompt_action, render_title, Action};
use todo_rust::todo::Todos;

fn main() {
    let todos = RefCell::new(Todos::new());
    let mut actions = [
        Action {
            label: "asdas",
            action: Box::new(|| todos.borrow_mut().add("qwe")),
        },
        Action {
            label: "asd",
            action: Box::new(|| todos.borrow_mut().add("qwe")),
        },
    ];

    render_title();

    loop {
        if prompt_action(&mut actions).is_none() {
            print_catching_message("You haven't chosen anything, i'm leaving, see you soon!");
            break;
        }
    }
}
