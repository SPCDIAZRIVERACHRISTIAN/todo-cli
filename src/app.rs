pub struct App {
    pub tasks: Vec<Task>,
    pub generator: IdGenerator,
}

impl App {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let tasks = match load_tasks(path) {
            Ok(tasks) => tasks,
            Err(_) => Vec::new(),
        };

        Ok(Self {
            tasks,
            generator: IdGenerator::new(),
        })
    }

    pub fn add(&mut self, title: &str) {
        let task = Task::new(&mut self.generator, title);
        self.tasks.push(task);
    }

    pub fn mark_done(&mut self, id: u32) {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => task.mark_done(),
            None => println!("task not found"),
        }
    }

    pub fn delete(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id);
    }

    pub fn list(&self) {
        for task in &self.tasks {
            println!("{} [{}]", task.title, if task.done { "X" } else { " " });
        }
    }
}
