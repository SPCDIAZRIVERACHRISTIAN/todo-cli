pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, title: &str) -> Task {
        Task {
            id,
            title: title.to_string(),
            done: false,
        }
    }

    pub fn to_line(&self) -> String {
        format!("{}|{}|{}", self.id, self.title, self.done)
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let mut parts = line.split('|');

        let id = parts.next()?.parse().ok()?;
        let title = parts.next()?.to_string();
        let done = parts.next()?.parse().ok()?;

        Some(Task { id, title, done })
    }

    pub fn done(&mut self) {
        self.done = !self.done;
    }
}
