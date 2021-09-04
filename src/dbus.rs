use crate::song::SongInfo;
use failure::{Error, ResultExt};
use mpris::{Metadata, PlayerFinder};

pub fn read_music_player() -> Result<SongInfo, Error> {
    let metadata = get_metadata().context("Error while reading metadata")?;
    let song = SongInfo::from_metadata(metadata)?;
    Ok(song)
}

fn get_metadata() -> Result<Metadata, Error> {
    let apps = vec!["spotify", "mopidy", "mpd", "chromium"];
    let player_finder = PlayerFinder::new().context("Could not connect to D-Bus")?;
    let active_players = player_finder
        .find_all()
        .context("Could not find any player")?;

    let active_player = apps
        .iter()
        .find(|&app| {
            active_players
                .iter()
                .find(|p| &&*p.identity().to_lowercase() == app)
                .is_some()
        })
        .unwrap();

    let player = active_players
        .iter()
        .find(|p| &&*p.identity().to_lowercase() == active_player)
        .unwrap();

    let metadata = player
        .get_metadata()
        .context("Could not get metadata for player")?;

    Ok(metadata)
}
