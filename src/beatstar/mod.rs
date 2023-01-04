use crate::beatstar::ffi::BeatStarDataFile;
use once_cell::sync::OnceCell;
use std::env;

mod data;
mod database;

#[macro_use]
mod macros;
mod ffi;
mod numstuff;

static BEAT_STAR_FILE: OnceCell<BeatStarDataFile> = OnceCell::new();
// static BEAT_STAR_MUTEX: Mutex<i8> = Mutex::new(0);

#[cfg(test)]
mod tests {
    use super::*;
    use database::*;
    use stopwatch::Stopwatch;

    #[test]
    fn download_db() -> anyhow::Result<()> {
        let mut stopwatch = Stopwatch::start_new();
        println!("Getting db");
        beatstar_update_database_network()?;
        println!("Got DB, took {0}ms", stopwatch.elapsed().as_millis());
        stopwatch.restart();
        beatstar_update_database_network()?;
        assert!(stopwatch.elapsed().as_millis() < 100);
        println!("Memory Cache works");
        Ok(())
    }


    #[test]
    fn download_song_stars() {
        download_db().unwrap();
        // assert_eq!(2 + 2, 4);

        let song = beatstar_get_song("4B2DA842B687EC4CFBC948C583C21C79D4120DE0");

        let diff = (song
            .expect("Could not fetch song database")
            .expect("Could not find song in database")
            .diffs)[0]
            .clone();
        println!("Got the notes!: {0} {1}", diff.diff.to_string(), diff.notes);
        println!("Got the stars!: {0} {1}", diff.diff.to_string(), diff.stars);
    }

    #[test]
    fn download_load_songs_file() -> anyhow::Result<()> {
        // Download file
        download_db().unwrap();

        let mut stopwatch = Stopwatch::start_new();

        println!("Downloading");
        let mut path = env::current_dir().unwrap();
        path.push("combinedScrappedDataTest.zip");
        beatstar_download_database_to_file(path.to_str().unwrap()).unwrap();

        println!("Getting db from file");
        beatstar_update_database_file(path.to_str().unwrap())?;

        println!("Got DB, took {0}ms", stopwatch.elapsed().as_millis());

        stopwatch.restart();

        beatstar_update_database_network()?;
        beatstar_update_database_file(path.to_str().unwrap())?;
        assert!(stopwatch.elapsed().as_millis() < 100);

        println!("Memory Cache works");
        Ok(())
    }

    #[test]
    fn get_song_characteristics() {
        download_db().unwrap();
        // assert_eq!(2 + 2, 4);

        let song = beatstar_get_song("B9BED84A127130BF80AFF18DB677EDD215CE0AB5")
            .unwrap()
            .unwrap();

        for _i in 0..3 {
            let diff_map_size = song.characteristics.len();
            println!("Characteristics size: {diff_map_size}");
            assert_eq!(diff_map_size, 2);
            for (chara, diff_map) in &song.characteristics {
                println!("Got the char!: {0} {1}", chara, diff_map.len());

                for (_diff_name, diff) in diff_map {
                    println!(
                        "Got the diff!: {0} with pp {1}",
                        diff.diff.to_string(),
                        diff.approximate_pp_value
                    );
                }
            }
        }
    }
}
