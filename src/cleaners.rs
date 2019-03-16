use crate::song::SongInfo;
use crate::util;
use regex::Regex;
use failure::{ Error, err_msg };
use std::collections::HashMap;

/*
 * There are two kinds of cleaners
 * 1- Darklyrics url cleaner
 * 2- Downloaded album lyrics cleaner
 */

fn fetch(url: &str) -> Result<String, Error> {
    let message = format!("No lyrics found on url: \n{}", url);
    let lyrics = reqwest::get(url)
        .map_err(|_| err_msg(message.clone()))?
        .text()
        .map_err(|_| err_msg(message.clone()))?;

    if lyrics.contains("<div class=\"lyrics\">") {
        Ok(lyrics)
    } else {
        Err(err_msg(message))
    }
}

fn default_url(song: &SongInfo) -> String {
    util::make_song_url(&song.artist, &song.album)
}

fn url_without_pars(song: &SongInfo) -> String {
    let par_cleaner = Regex::new(r"\([^)]*\)").unwrap();
    let artist = par_cleaner.replace_all(&song.artist, "");
    let album = par_cleaner.replace_all(&song.album, "");
    util::make_song_url(&artist, &album)
}

pub fn fetch_album_lyrics(song: &SongInfo) -> Result<String, Error> {
    fetch(&default_url(&song))
        .or(fetch(&url_without_pars(&song)))
}

pub fn find_song_in_album(album_lyrics: &HashMap<String, String>, song: &SongInfo) -> Result<String, Error> {
    with_track_and_title(&album_lyrics, song.track, &song.title)
        .or(with_title(&album_lyrics, &song.title))
        .or(with_track(&album_lyrics, song.track))
}

fn with_track_and_title(album_lyrics: &HashMap<String, String>, track: i32, title: &str) -> Result<String, Error> {
    album_lyrics.get(&format!("{}. {}", track, title))
        .map(|lyrics| lyrics.to_string())
        .ok_or(err_msg("Song not found on album lyrics"))
}

fn with_title(album_lyrics: &HashMap<String, String>, title: &str) -> Result<String, Error> {
    album_lyrics.iter()
        .find(|(song, _)| song.contains(title))
        .map(|(_, lyrics)| lyrics.to_string())
        .ok_or(err_msg("Song not found on album lyrics"))
}

fn with_track(album_lyrics: &HashMap<String, String>, track: i32) -> Result<String, Error> {
    album_lyrics.iter()
        .find(|(song, _)| song.starts_with(&track.to_string()))
        .map(|(_, lyrics)| lyrics.to_string())
        .ok_or(err_msg("Song not found on album lyrics"))
}
