use super::constants::TITLE_TEXT;
use crate::todo::Todo;
use colorful::{Color, Colorful};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self, Write};

pub struct Action<'a> {
    pub label: &'static str,
    pub action: Option<Box<dyn FnMut() + 'a>>,
}

pub fn prompt_action(actions: &mut [Action], message: Message) -> Option<()> {
    print_message(message);

    let labels: Vec<&str> = actions.iter().map(|action| action.label).collect();

    let result = display_selection(&labels);
    let index = result.ok().flatten()?;

    let action = &mut actions[index].action;
    if let Some(action) = action {
        action();
    }

    Some(())
}

pub fn display_selection(items: &[&str]) -> io::Result<Option<usize>> {
    Select::with_theme(&ColorfulTheme::default())
        .items(items)
        .default(0)
        .interact_on_opt(&Term::stderr())
}

pub fn read_line() -> Option<String> {
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => Some(buffer),
        Err(_) => None,
    }
}

pub enum Message<'a> {
    NothingChosen,
    NoTodos,
    WhatToDo,
    WhatToDoWithTodo,
    Title,
    PromptTitle,
    PromptDescription,
    TodoAdded,
    TodoInformation(&'a Todo),
}

pub fn print_message(message: Message) {
    match message {
        Message::NothingChosen => println!(
            "{}",
            "You haven't chosen anything, i'm leaving, see you soon!".red()
        ),
        Message::NoTodos => println!("{} {}", "!".yellow(), "You have no todos :(".bold()),
        Message::WhatToDo => println!(
            "{} {} {}",
            "?".yellow(),
            "What do you want me to do?".bold(),
            "(you can press <esc> or <q> to exit)"
        ),
        Message::WhatToDoWithTodo => println!(
            "{} {}",
            "?".yellow(),
            "Do you want to do something with this todo?".bold(),
        ),
        Message::Title => println!("{}", TITLE_TEXT.gradient(Color::Red)),
        Message::PromptTitle => {
            print!(
                "{} {}",
                "?".yellow(),
                "What title should your todo have? ".bold(),
            );
            io::stdout().flush().unwrap();
        }
        Message::PromptDescription => {
            print!(
                "{} {}",
                "?".yellow(),
                "What description should your todo have? ".bold()
            );
            io::stdout().flush().unwrap();
        }
        Message::TodoAdded => println!("{} {}", "!".yellow(), "Todo added!".bold()),
        Message::TodoInformation(todo) => {
            println!(
                "{} {}\n{}: {}\n{}: {}\n{}: {}",
                "!".yellow(),
                "Todo information!".bold(),
                "Title".yellow().bold(),
                todo.title,
                "Description".yellow().bold(),
                todo.description,
                "Done".yellow().bold(),
                todo.done
            )
        }
    }
}
