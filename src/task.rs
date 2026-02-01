pub struct Task {
    id: u32,
    title: String,
    done: bool,
}

pub struct IdGenerator {
    next_id: u32,
}

pub impl IdGenerator {
    fn new() -> Self {
        IdGenerator { next_id: 1 }
    }

    fn next(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub impl Task {
    fn new(generator: &mut IdGenerator, title: &str) -> Task {
        Task {
            id: generator.next(),
            title: title.to_string(),
            done: false,
        }
    }

    fn to_line(&self) -> String {
        format!("{} {} {}", self.id, self.title, self.done)
    }
}
