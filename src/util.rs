use crate::song::SongInfo;
use mpris::{ PlayerFinder, Metadata };
use failure::{ Error, ResultExt };


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

