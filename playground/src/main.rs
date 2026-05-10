fn main() {
    let result = std::panic::catch_unwind(|| {
        should_throw();
    });

    match result {
        Ok(_) => println!("Function executed successfully."),
        Err(err) => println!("Caught a panic: {:?}", err),
    }
}

fn should_throw() -> String {
    panic!("This function always panics!");
}
