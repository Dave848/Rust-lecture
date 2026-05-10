mod app;
mod task;
mod printer;
mod input_handler;
mod file_handler;
use app::App;

fn main() {
    let mut app: App = App::new();
    app.run();
}
