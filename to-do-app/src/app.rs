use crate::printer::Printer;
use crate::file_handler::FileHandler;
use crate::input_handler::InputHandler;

pub struct App {
    input_handler: InputHandler,
}

impl App {
    pub fn new() -> Self {
        let file_handler: FileHandler = FileHandler::new();
        let input_handler: InputHandler = InputHandler::new(file_handler);
        Self {
            input_handler,
        }
    }

    pub fn run(&mut self) {
        Printer::hello_message();
        self.run_loop();
        Printer::goodbye_message();
    }

    fn run_loop(&mut self) {
        loop {
            Printer::menu();
            let user_input: String = self.input_handler.get_user_input();
            if user_input == "0" || user_input.to_lowercase() == "q" {
                break;
            }
            self.input_handler.handle_user_input(&user_input);
        }
    }
}