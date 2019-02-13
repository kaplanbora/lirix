use lirix::util;


fn main() {
    match util::run() {
        Ok(lyrics) => println!("{}", lyrics),
        Err(error) => util::print_error(&error),
    };
}

