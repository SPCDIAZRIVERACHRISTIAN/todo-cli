use std::io;
use std::path::Path;

use crate::task::Task;

pub fn load_tasks(path: &Path) -> io::Result<Vec<Task>> {
    let content = std::fs::read_to_string(path)?;

    let tasks = content
        .lines()
        .filter_map(|line| Task::from_line(line))
        .collect();

    Ok(tasks)
}

pub fn save_tasks(path: &Path, tasks: &[Task]) -> io::Result<()> {
    let data = tasks
        .iter()
        .map(Task::to_line)
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(path, data)
}
