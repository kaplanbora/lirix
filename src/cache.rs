use crate::song::SongInfo;
use failure::{Error, err_msg};
use std::fs;
use std::path::PathBuf;
use dirs;


fn get_album_dir(song: &SongInfo) -> Result<PathBuf, Error> {
    let mut lyrics_path = dirs::home_dir().ok_or(err_msg("Unable to get home directory"))?;
    lyrics_path.push("lyrics");
    lyrics_path.push(&song.artist.replace("/", "-"));
    lyrics_path.push(&song.album.replace("/", "-"));
    Ok(lyrics_path)
}

fn format_song_file(song: &SongInfo) -> String {
    format!("{:02}. {}.txt", &song.track, &song.title.replace("/", "-"))
}

pub fn find_lyrics(song: &SongInfo) -> Result<String, Error> {
    let mut lyrics_path = get_album_dir(&song)?;
    lyrics_path.push(format_song_file(&song));
    fs::read_to_string(lyrics_path.as_path())
        .map_err(|_| err_msg("Lyrics not found on cache"))
}

pub fn save_lyrics(song: &SongInfo, lyrics: &str) -> Result<(), Error> {
    let mut lyrics_path = get_album_dir(&song)?;
    fs::create_dir_all(lyrics_path.as_path())
        .map_err(|_| err_msg("Unable to create album directory"))?;
    lyrics_path.push(format_song_file(&song));
    fs::write(lyrics_path.as_path(), &lyrics)
        .map_err(|_| err_msg("Unable to create lyrics file"))
}


