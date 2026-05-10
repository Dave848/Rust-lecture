use crate::task::Task;
use crate::printer::Printer;
use crate::file_handler::FileHandler;

pub struct InputHandler {
    file_handler: FileHandler,
}

impl InputHandler {
    pub fn new(file_handler: FileHandler) -> Self {
        InputHandler {
            file_handler,
        }
    }

    pub fn handle_user_input(&self, input: &str) {
        match input {
            "1" => self.view_tasks(),
            "2" => self.add_task(),
            "3" => self.remove_task(),
            _ => Printer::invalid_choice_message(),
        }
    }

    pub fn get_user_input(&self) -> String {
        let mut user_input: String = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut user_input) {
            eprintln!("Failed to read input: {}", e);
        }        
        user_input.trim().to_string()
    }

    pub fn view_tasks(&self){
        let tasks: Vec<Task> = match self.file_handler.read_tasks_from_file() {
            Ok(tasks) => tasks,
            Err(e) => {
                eprintln!("Error reading tasks: {}", e);
                return;
            }
        };

        if tasks.is_empty() {
            Printer::no_tasks();
        } else {
            Printer::tasks(&tasks);
        }
    }

    pub fn add_task(&self) {
        println!("Type in new task:");
        let task_name: String = self.get_user_input();
        let task: Task = Task { name: task_name };
        if let Err(e) = self.file_handler.add_task_to_file(&task) {
            println!("Error adding task: {}", e);
        }
    }

    pub fn remove_task(&self) {
        let tasks: Vec<Task> = match self.file_handler.read_tasks_from_file() {
            Ok(tasks) => tasks,
            Err(e) => {
                eprintln!("Error reading tasks: {}", e);
                return;
            }
        };

        if tasks.is_empty() {
            Printer::no_tasks();
            return;
        }

        Printer::tasks(&tasks);
        println!("Type in number of task to remove:");
        let input: usize = match self.get_user_input().parse() {
            Ok(num) => num,
            Err(_) => {
                Printer::invalid_choice_message();
                return;
            }
        };
        if input == 0 || input > tasks.len() {
            Printer::invalid_choice_message();
            return;
        }

        let mut tasks: Vec<Task> = tasks;
        tasks.remove(input - 1);
        if let Err(e) = self.file_handler.save_tasks_to_file(&tasks) {
            println!("Error saving tasks: {}", e);
        }
    }
}