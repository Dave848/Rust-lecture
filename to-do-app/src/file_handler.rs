use std::fs;
use serde_json;

use crate::task::Task;

pub struct FileHandler {
   pub file_name: String,
}

impl FileHandler {
    pub fn new() -> Self {
        FileHandler {
            file_name: String::from("tasks.json"),
        }
    }

    pub fn read_tasks_from_file(&self) -> std::io::Result<Vec<Task>> {
        let data: String = fs::read_to_string(&self.file_name)?;
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap_or_else(|_| vec![]);
        Ok(tasks)
    }

    pub fn save_tasks_to_file(&self, tasks: &Vec<Task>) -> std::io::Result<()> {
        let data: String = serde_json::to_string_pretty(tasks).unwrap();
        fs::write(&self.file_name, data)?;
        Ok(())
    }

    pub fn add_task_to_file(&self, task: &Task) -> std::io::Result<()> {
        let mut tasks: Vec<Task> = self.read_tasks_from_file().unwrap_or_else(|_| vec![]);
        tasks.push(task.clone());
        self.save_tasks_to_file(&tasks)
    }
}