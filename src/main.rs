use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

pub fn mark_done(tasks: &mut [Task], id: u32) {
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => task.done = true,
        None => println!("task not found"),
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: u32) -> io::Result<()> {
    let len_before = tasks.len();
    tasks.retain(|task| task.id != id);
    len_before != tasks.len();
    Ok(())
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

fn add(tasks: &mut Vec<Task>, title: &str, generator: &mut IdGenerator) -> std::io::Result<()> {
    let added = Task::new(generator, title);

    tasks.push(added);

    Ok(())
}

fn list(file: &str) -> std::io::Result<()> {
    let content = std::fs::read_to_string(file)?;

    for line in content.lines() {
        println!("{}", line);
    }

    Ok(())
}

fn list_state(tasks: &[Task]) -> std::io::Result<()> {
    for task in tasks {
        match task.done {
            true => println!("{} [X]", task.title),
            false => println!("{} []", task.title),
        }
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

fn main() -> std::io::Result<()> {
    let file = "test2.txt";
    let mut generator = IdGenerator::new();
    let mut tasks: Vec<Task> = tasks_state(&file)?;

    add(&mut tasks, "Take a bath", &mut generator)?;
    add(&mut tasks, "eat", &mut generator)?;
    add(&mut tasks, "sleep", &mut generator)?;
    println!("added tasks bro!");

    list_state(&tasks)?;

    println!("marking done 1 & 3");
    mark_done(&mut tasks, 3);
    mark_done(&mut tasks, 1);

    println!("deleting number 2...");
    delete_task(&mut tasks, 2)?;

    println!("file made and wrote shit to it!");
    save_tasks(&file, &mut tasks);

    println!("This is the list working just fine");
    list(&file)?;

    Ok(())
}
