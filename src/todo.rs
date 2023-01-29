use nanoid::nanoid;

pub struct Todo {
    pub id: String,
    pub done: bool,
    pub description: String,
}

pub struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    pub fn new() -> Todos {
        Todos { todos: vec![] }
    }

    pub fn add(&mut self, description: &str) {
        self.todos.push(Todo {
            id: nanoid!(),
            done: false,
            description: String::from(description),
        })
    }

    pub fn mark_as_done(&mut self, id: &str) {
        self.todos.iter().position(|todo| todo.id == id);
    }

    pub fn remove(&mut self, id: &str) {
        self.todos.retain(|todo| todo.id == id)
    }
}
