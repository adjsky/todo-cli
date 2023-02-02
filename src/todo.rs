use crate::constants::SAVE_PATH;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub done: bool,
    pub title: String,
    pub description: String,
}

pub struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Todos {
        Todos { todos: vec![] }
    }

    pub fn add(&mut self, title: &str, description: &str) {
        self.todos.push(Todo {
            id: nanoid!(),
            done: false,
            title: String::from(title),
            description: String::from(description),
        });

        let _ = self.serialize();
    }

    pub fn mark_as_done(&mut self, id: &str) {
        let position = self.todos.iter().position(|todo| todo.id == id);

        if let Some(index) = position {
            self.todos[index].done = true
        }

        let _ = self.serialize();
    }

    pub fn remove(&mut self, id: &str) {
        self.todos.retain(|todo| todo.id != id);

        let _ = self.serialize();
    }

    pub fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn get(&self, index: usize) -> &Todo {
        &self.todos[index]
    }

    pub fn deserialize(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(SAVE_PATH)?;
        let reader = BufReader::new(file);

        let deserialized: Vec<Todo> = serde_json::from_reader(reader)?;
        self.todos = deserialized;

        Ok(())
    }

    pub fn serialize(&self) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(SAVE_PATH)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer(writer, &self.todos)?;

        Ok(())
    }
}
