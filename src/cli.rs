use super::constants::TITLE_TEXT;
use colorful::{Color, Colorful};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io;

pub struct Action<'a> {
    pub label: &'static str,
    pub action: Box<dyn FnMut() + 'a>,
}

pub fn prompt_action(actions: &mut [Action]) -> Option<()> {
    print_message(Message::WhatToDo);

    let labels: Vec<&str> = actions.iter().map(|action| action.label).collect();

    let result = display_selection(&labels);
    let index = result.ok().flatten()?;

    (actions[index].action)();

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

pub enum Message {
    NothingChosen,
    NoTodos,
    HereYourTodos,
    WhatToDo,
    Title,
}

pub fn print_message(message: Message) {
    match message {
        Message::NothingChosen => println!(
            "{}",
            "You haven't chosen anything, i'm leaving, see you soon!".red()
        ),
        Message::NoTodos => println!("{}", "You have no todos!".yellow()),
        Message::HereYourTodos => println!("{}", "Here are your todos!".yellow()),
        Message::WhatToDo => println!("{} {}", "?".yellow(), "What do you want me to do?".bold()),
        Message::Title => println!("{}", TITLE_TEXT.gradient(Color::Red)),
    }
}
