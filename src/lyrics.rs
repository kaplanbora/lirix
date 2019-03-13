use crate::song::SongInfo;
use crate::cache;
use crate::cleaners;
use failure::{ Error, err_msg };
use regex::Regex;
use std::collections::HashMap;


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
    let cached_lyrics = cache::find_lyrics(&song);

    let lyrics = if cached_lyrics.is_ok() {
        cached_lyrics.unwrap()
    } else {
        let raw_lyrics = fetch_lyrics(&song)?;
        let album_lyrics = get_album_lyrics(raw_lyrics);
        let song_lyrics = album_lyrics.get(&format!("{}. {}", song.track, song.title))
            .ok_or(err_msg("Song not found on album lyrics"))?;
        clear_html(song_lyrics)
    };

    Ok(lyrics)
}

fn get_album_lyrics(lyrics: String) -> HashMap<String, String> {
    let mut album_lyrics: HashMap<String, String> = HashMap::new();
    let mut song_lyrics = String::new();
    let mut title = String::new();

    for line in lyrics.split("\n") {
        if line.starts_with("<h3><a name=") {
            if !song_lyrics.is_empty() {
                album_lyrics.insert(title.clone(), song_lyrics.clone());
                song_lyrics.clear();
            }
            title = clear_html(&line);
            dbg!(&title);
        } else if line.starts_with("<div class=\"note\">") {
            if !song_lyrics.is_empty() {
                album_lyrics.insert(title.clone(), song_lyrics.clone());
                song_lyrics.clear();
            }
            break;
        } else {
            song_lyrics.push_str(line);
            song_lyrics.push_str("\n");
        }
    }

    album_lyrics
}
