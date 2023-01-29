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
    println!("{} {}", "?".yellow(), "What do you want me to do?".bold());

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

pub fn render_title() {
    println!("{}", TITLE_TEXT.gradient(Color::Red));
}

pub fn print_catching_message(message: &str) {
    println!("{}", message.red());
}

pub fn print_message(message: &str) {
    println!("{}", message.yellow());
}
