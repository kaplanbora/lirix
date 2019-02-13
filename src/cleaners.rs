use crate::song::SongInfo;
use crate::lyrics;
use regex::Regex;


pub fn default(song: &SongInfo) -> String {
    lyrics::make_song_url(&song.artist, &song.album, song.track)
}

pub fn remove_paranthesis(song: &SongInfo) -> String {
    let par_cleaner = Regex::new(r"\([^)]*\)").unwrap();
    let artist = par_cleaner.replace_all(&song.artist, "");
    let album = par_cleaner.replace_all(&song.album, "");
    lyrics::make_song_url(&artist, &album, song.track)
}
