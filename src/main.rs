use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

struct Task {
    id: u32,
    title: String,
    done: bool,
}

struct IdGenerator {
    next_id: u32,
}

impl IdGenerator {
    fn new() -> Self {
        IdGenerator { next_id: 1 }
    }

    fn next(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

impl Task {
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

pub fn mark_done(tasks: &mut [Task], id: u32) -> bool {
    for task in tasks {
        if task.id == id {
            task.done = true;
            return true;
        }
    }
    false
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) -> bool {
    let len_before = tasks.len();
    tasks.retain(|task| task.id != id);
    len_before != tasks.len()
}

pub fn load_tasks(path: &str) -> io::Result<Vec<Task>> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()), // file doesn't exist yet
    };

    let reader = BufReader::new(file);
    let mut tasks = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            continue;
        }

        let id: u32 = parts[0].parse().unwrap_or(0);
        let done: bool = parts.last().unwrap() == &"true";

        let title = parts[1..parts.len() - 1].join(" ");

        tasks.push(Task { id, title, done });
    }

    Ok(tasks)
}

pub fn save_tasks(path: &str, tasks: &[Task]) -> io::Result<()> {
    let mut file = File::create(path)?;

    for task in tasks {
        writeln!(file, "{} {} {}", task.id, task.title, task.done)?;
    }

    Ok(())
}

fn add(file_path: &str, content: Task) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    writeln!(&mut file, "{}", content.to_line())?;

    Ok(())
}

fn list(file: &str) -> std::io::Result<()> {
    let content = std::fs::read_to_string(file)?;

    for line in content.lines() {
        println!("{}", line);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut generator = IdGenerator::new();

    add("test.txt", Task::new(&mut generator, "Take a bath"))?;
    add("test.txt", Task::new(&mut generator, "eat"))?;
    add("test.txt", Task::new(&mut generator, "sleep"))?;
    println!("file made and wrote shit to it!");
    println!("This is the list working just fine");
    list("test.txt")?;
    Ok(())
}
