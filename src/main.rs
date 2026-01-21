use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;

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
