use mpris::Metadata;
use failure::{ Error, err_msg };

#[derive(Eq, PartialEq, Debug)]
pub struct SongInfo {
    pub track: i32,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub art: String,
}

impl SongInfo {
    pub fn from_metadata(metadata: Metadata) -> Result<SongInfo, Error> {
        let track_number = &metadata.track_number()
            .ok_or(err_msg("Track number not found"))?;
        let artists = &metadata.album_artists()
            .ok_or(err_msg("Artist not found"))?;
        let album_name = &metadata.album_name()
            .ok_or(err_msg("Album name not found"))?;
        let art_url = &metadata.art_url()
            .ok_or(err_msg("Album art not found"))?;
        let title = &metadata.title()
            .ok_or(err_msg("Song title not found"))?;

        let track = track_number.to_owned();
        let title = title.to_string();
        let album = album_name.to_string();
        let artist = artists[0].to_string();
        let art = art_url.to_string();

        Ok(SongInfo { track, title, album, artist, art })
    }

    pub fn dummy() -> SongInfo {
        SongInfo {
            track: 1337,
            title: String::from("Dummy Song Title"),
            album: String::from("Dummy Album Name"),
            artist: String::from("Dummy Artist Name"),
            art: String::from("Dummy Album Art")
        }
    }
}

