use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use todo_rust::utils::read_line;
use todo_rust::Todos;

fn main() -> std::io::Result<()> {
    let term = Term::stdout();
    Todos::new();

    loop {
        let line = read_line().unwrap();

        match line.trim_end() {
            "colored" => {
                term.write_line(format!("{}", style("LOH").cyan()).as_str())?;
            }
            "select" => {
                let items = vec!["Item 1", "item 2"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .items(&items)
                    .default(0)
                    .interact_on_opt(&Term::stderr())?;

                match selection {
                    Some(index) => println!("User selected item : {}", items[index]),
                    None => println!("User did not select anything"),
                }
            }
            _ => {
                println!("break");
                break;
            }
        }
    }

    Ok(())
}
