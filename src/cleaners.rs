use crate::song::SongInfo;
use crate::util;
use regex::Regex;


pub fn default(song: &SongInfo) -> String {
    util::make_song_url(&song.artist, &song.album)
}

pub fn remove_paranthesis(song: &SongInfo) -> String {
    let par_cleaner = Regex::new(r"\([^)]*\)").unwrap();
    let artist = par_cleaner.replace_all(&song.artist, "");
    let album = par_cleaner.replace_all(&song.album, "");
    util::make_song_url(&artist, &album)
}
