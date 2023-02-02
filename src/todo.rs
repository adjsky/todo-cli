use nanoid::nanoid;

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
        })
    }

    pub fn mark_as_done(&mut self, id: &str) {
        let position = self.todos.iter().position(|todo| todo.id == id);

        if let Some(index) = position {
            self.todos[index].done = true
        }
    }

    pub fn remove(&mut self, id: &str) {
        self.todos.retain(|todo| todo.id != id)
    }

    pub fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn get(&self, index: usize) -> &Todo {
        &self.todos[index]
    }
}
