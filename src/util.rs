use crate::song::SongInfo;
use mpris::{ PlayerFinder, Metadata };
use failure::{ Error, ResultExt, err_msg };


pub fn run() -> Result<String, Error> {
    let song = read_music_player()?;
    let content = fetch_lyrics(&song)?;
    let lyrics = clear_lyrics(&content)?;
    Ok(lyrics)
}

pub fn print_error(error: &Error) {
    println!("Error: {}", error);
    for (i, cause) in error.iter_causes().enumerate() {
        print!("{}", "  ".repeat(i + 1));
        println!("Caused by: {}", cause);
    }
}

pub fn read_music_player() -> Result<SongInfo, Error> {
    let metadata = get_metadata().context("Error while reading metadata")?;
    let song = SongInfo::from_metadata(metadata)?;

    Ok(song)
}

fn get_metadata() -> Result<Metadata, Error> {
    let player_finder = PlayerFinder::new().context("Could not connect to D-Bus")?;
    let player = player_finder.find_active().context("Could not find any player")?;
    let metadata = player.get_metadata().context("Could not get metadata for player")?;

    Ok(metadata)
}

pub fn fetch_lyrics(song: &SongInfo) -> Result<String, Error> {
    let url = make_song_url(&song);
    println!("{}", &url);
    let content = reqwest::get(&url)?.text()?;
    Ok(content)
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
