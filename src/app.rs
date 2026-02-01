use crate::task::Task;

pub struct App {
    tasks: Vec<Task>,
}

impl App {
    pub fn new(tasks: Vec<Task>) -> Self {
        Self { tasks }
    }

    fn next_id(&self) -> u32 {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("no tasks were found");
            return;
        }

        for task in &self.tasks {
            println!("{} [{}]", task.title, if task.done { "X" } else { "" });
        }
    }

    pub fn add(&mut self, title: String) {
        let id = self.next_id();

        self.tasks.push(Task::new(id, &title));
    }

    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn done(&mut self, id: u32) {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => task.done(),
            None => println!("task not found"),
        }
    }

    pub fn delete(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id);
    }
}
