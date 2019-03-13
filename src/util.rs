use failure::Error;
use crate::lyrics;
use crate::song::SongInfo;
use std::process::Command;
use std::{thread, time};


pub fn print_error(error: &Error) {
    //clear();
    println!("Error: {}", error);
    for (i, cause) in error.iter_causes().enumerate() {
        print!("{}", "  ".repeat(i + 1));
        println!("Caused by: {}", cause);
    }
}

pub fn make_song_url(artist: &str, album: &str) -> String {
    // Example URL: http://www.darklyrics.com/lyrics/soilwork/theridemajestic.html#8
    format!(
        "http://www.darklyrics.com/lyrics/{}/{}.html", 
        artist.to_lowercase().replace(" ", ""),
        album.to_lowercase().replace(" ", "")
    )
}

fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cls")
                .spawn()
                .expect("Failed to clear the terminal");
    } else {
        Command::new("clear")
                .spawn()
                .expect("Failed to clear the terminal");
    }

    let millis = time::Duration::from_millis(100);
    thread::sleep(millis);
}

pub fn clear_and_print(text: &str) {
    clear();
    println!("{}", text);
}

pub fn print_lyrics(song: &SongInfo) {
    match lyrics::get_song_lyrics(&song) {
        Ok(lyrics) => clear_and_print(&pretty_lyrics(&song, &lyrics)),
        Err(error) => print_error(&error),
    }
}

fn pretty_lyrics(song: &SongInfo, lyrics: &str) -> String {
    format!("{} - {}\n{:02}. {}\n\n{}", song.artist, song.album, song.track, song.title, lyrics)
}
