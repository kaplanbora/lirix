use crate::song::SongInfo;
use failure::{ Error, err_msg };


pub fn fetch_lyrics(song: &SongInfo) -> Result<String, Error> {
    let url = make_song_url(&song);
    println!("{}", &url);
    let content = reqwest::get(&url)?.text()?;
    let lyrics = clear_lyrics(&content)?;
    Ok(lyrics)
}

fn make_song_url(song: &SongInfo) -> String {
    // Example URL: http://www.darklyrics.com/lyrics/soilwork/theridemajestic.html#8
    format!(
        "http://www.darklyrics.com/lyrics/{}/{}.html#{}", 
        &song.artist.to_lowercase().replace(" ", ""),
        &song.album.to_lowercase().replace(" ", ""),
        &song.track
    )
}

fn clear_lyrics(lyrics: &str) -> Result<String, Error> {
    let lyrics: Vec<&str> = lyrics.split("<div class=\"lyrics\">").collect();
    let lyrics: &str = lyrics.get(1).ok_or(err_msg("Lyrics not found"))?;

    let lyrics: Vec<&str> = lyrics.split("<div class=\"thanks\">").collect();
    let lyrics: &str = lyrics.get(0).ok_or(err_msg("Thanks section not found"))?;

    let lyrics: String = lyrics.replace("<br />", "");

    Ok(lyrics)
}
