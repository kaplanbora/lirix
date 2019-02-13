use failure::Error;


pub fn print_error(error: &Error) {
    println!("Error: {}", error);
    for (i, cause) in error.iter_causes().enumerate() {
        print!("{}", "  ".repeat(i + 1));
        println!("Caused by: {}", cause);
    }
}

