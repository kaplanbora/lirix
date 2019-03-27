use lirix::util;
use lirix::dbus;
use lirix::song::SongInfo;
use std::{thread, time};


fn main() {
    let one_second = time::Duration::from_secs(1);
    let mut song = SongInfo::dummy();

    loop {
        let new_song = dbus::read_music_player();

        if new_song.is_err() {
            util::clear_and_print("Waiting for a song");
        } else {
            let new_song = new_song.unwrap();
            if song != new_song {
                song = new_song;
                util::print_lyrics(&song);
            }
        }

        thread::sleep(one_second);
    }
}

