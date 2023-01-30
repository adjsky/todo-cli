use console::Term;
use ctrlc;
use std::cell::RefCell;
use todo_rust::cli::{display_selection, print_message, prompt_action, read_line, Action, Message};
use todo_rust::todo::Todos;

fn main() {
    setup_ctrlc();

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
                    return;
                }

                print_message(Message::HereYourTodos);

                let result = display_selection(&titles).ok().flatten();
                let index = match result {
                    Some(index) => index,
                    None => return,
                };
            }),
        },
        Action {
            label: "Create a new todo",
            action: Box::new(|| {
                let mut borrowed_todos = todos.borrow_mut();

                print_message(Message::TodoTitle);
                let title = match read_line() {
                    Some(line) => line.trim().to_owned(),
                    None => return,
                };

                print_message(Message::TodoDescription);
                let description = match read_line() {
                    Some(line) => line.trim().to_owned(),
                    None => return,
                };

                borrowed_todos.add(&title, &description);
                print_message(Message::TodoAdded);
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
