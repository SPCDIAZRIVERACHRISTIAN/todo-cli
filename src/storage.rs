use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

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
        //id should not default to 0 it should stop process if data read is not correctly
        let id: u32 = parts[0].parse().unwrap_or(0); //TODO make this be a panic! or error
        let done: bool = parts.last().unwrap() == &"true";

        let title = parts[1..parts.len() - 1].join(" ");

        tasks.push(Task { id, title, done });
    }

    Ok(tasks)
}

pub fn save_tasks(path: &str, tasks: &[Task]) -> io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    for task in tasks {
        writeln!(file, "{} {} {}", task.id, task.title, task.done)?;
    }

    Ok(())
}

fn list_file(file: &str) -> std::io::Result<()> {
    let content = std::fs::read_to_string(file)?;

    for line in content.lines() {
        println!("{}", line);
    }

    Ok(())
}

fn tasks_state(file: &str) -> std::io::Result<Vec<Task>> {
    let path = Path::new(&file);
    let mut tasks: Vec<Task> = Vec::new();

    match path.exists() {
        true => {
            list(&file)?;
            tasks.extend(load_tasks(&file)?);
            Ok(tasks)
        }
        false => Ok(tasks),
    }
}
