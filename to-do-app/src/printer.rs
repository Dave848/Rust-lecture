use crate::task::Task;

pub struct Printer {}

impl Printer {
    pub fn hello_message() {
        println!("Hello user!");
    }

    pub fn goodbye_message() {
        println!("Goodbye!");
    }

    pub fn menu() {
        Printer::separator();
        println!("Menu:");
        println!("1. View tasks");
        println!("2. Add new task");
        println!("3. Remove task");
        println!("0. Exit");
        Printer::separator();
    }

    pub fn invalid_choice_message() {
        println!("Invalid choice!");
    }

    pub fn separator() {
        println!("-----------------------------");
    }

    pub fn no_tasks() {
        println!("No tasks found.");
    }

    pub fn tasks(tasks: &Vec<Task>) {
        println!("Tasks:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task.name);
        }
    }
}