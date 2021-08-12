use crate::beatstar::ffi::BeatStarDataFile;
use once_cell::sync::OnceCell;


mod data;
mod database;

#[macro_use]
mod macros;
mod ffi;

static BEAT_STAR_FILE: OnceCell<BeatStarDataFile> = OnceCell::new();

#[cfg(test)]
mod tests {
    use super::*;
    use database::*;
    use stopwatch::Stopwatch;

    #[test]
    fn download_db() {
        let mut stopwatch = Stopwatch::start_new();
        println!("Getting db");
        assert!(beatstar_update_database().is_none());
        println!("Got DB, took {0}ms", stopwatch.elapsed().as_millis());
        stopwatch.restart();
        assert!(beatstar_update_database().is_none());
        assert!(stopwatch.elapsed().as_millis() < 1000);
        println!("Memory Cache works");
    }

    #[test]
    fn download_song_stars() {
        download_db();
        // assert_eq!(2 + 2, 4);

        let song = beatstar_get_song("4B2DA842B687EC4CFBC948C583C21C79D4120DE0");

        unsafe {
            let diff = (*song
                .expect("Could not fetch song database")
                .expect("Could not find song in database")
                .diffs)[0]
                .clone();
            println!("Got the notes!: {0} {1}", diff.diff.to_string(), diff.notes);
            println!("Got the stars!: {0} {1}", diff.diff.to_string(), diff.stars);
        }
    }
}
