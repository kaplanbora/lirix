use lirix::util;


fn main() {
    match util::read_music_player() {
        Ok(song) => print!("{:?}", song),
        Err(error) => util::print_error(&error),
    };
}
