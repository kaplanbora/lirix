use lirix::util;
use lirix::lyrics;
use lirix::dbus;
use failure::{ Error };


fn main() {
    match run() {
        Ok(lyrics) => println!("{}", lyrics),
        Err(error) => util::print_error(&error),
    };
}

fn run() -> Result<String, Error> {
    let song = dbus::read_music_player()?;
    let lyrics = lyrics::fetch_lyrics(&song)?;
    Ok(lyrics)
}

