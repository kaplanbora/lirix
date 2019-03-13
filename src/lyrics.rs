use crate::song::SongInfo;
use crate::cleaners;
use failure::{ Error, err_msg };
use regex::Regex;


fn fetch_lyrics(song: &SongInfo) -> Result<String, Error> {
    let url = cleaners::default(&song);
    println!("{}", &url);
    let content = reqwest::get(&url)?.text()?;
    if content.is_empty() {
        Err(err_msg(format!("Darklyrics returned empty body for url: \n{}", &url)))
    } else {
        clear_lyrics(&content)
    }
}


fn clear_lyrics(lyrics: &str) -> Result<String, Error> {
    let lyrics: Vec<&str> = lyrics.split("<div class=\"lyrics\">").collect();
    let lyrics: &str = lyrics.get(1).ok_or(err_msg("Lyrics not found"))?;

    let lyrics: Vec<&str> = lyrics.split("<div class=\"thanks\">").collect();
    let lyrics: &str = lyrics.get(0).ok_or(err_msg("Thanks section not found"))?;

    let lyrics: String = lyrics.replace("<br />", "");

    Ok(lyrics)
}

fn clear_html(lyrics: &str) -> String {
    let html_cleaner = Regex::new(r"<[^>]*>").unwrap();
    html_cleaner.replace_all(&lyrics, "").to_string()
}

pub fn get_song_lyrics(song: &SongInfo) -> Result<String, Error> {
    let lyrics = fetch_lyrics(&song)?;
    let songs = split_songs(lyrics);
    let song_lyrics = songs.get(song.track as usize).ok_or(err_msg("Error while splitting the songs"))?;
    let clean_lyrics = clear_html(song_lyrics);
    Ok(clean_lyrics)
}

fn split_songs(lyrics: String) -> Vec<String> {
    let mut songs: Vec<String> = vec!();
    let mut song = String::new();

    for line in lyrics.split("\n") {

        if line.starts_with("<h3><a name=") || line.starts_with("<div class=\"note\">") {

            if !song.is_empty() {
                songs.push(song.clone());
                song.clear();
            }

            song.push_str(line);
            song.push_str("\n");
            song.push_str("\n");
        } else {
            song.push_str(line);
            song.push_str("\n");
        }
    }

    songs
}
