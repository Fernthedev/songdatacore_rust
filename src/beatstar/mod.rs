use crate::beatstar::ffi::BeatStarDataFile;
use once_cell::sync::OnceCell;

use tokio::runtime::Builder;
use tokio::runtime::Runtime;

mod data;
mod database;

#[macro_use]
mod macros;
mod ffi;

lazy_static! {
    static ref TokioRuntime: Runtime = Builder::new_multi_thread()
        .worker_threads(6)
        .enable_all()
        .build()
        .unwrap();
}

static BEAT_STAR_FILE: OnceCell<BeatStarDataFile> = OnceCell::new();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::beatstar::TokioRuntime;
    use database::*;
    use stopwatch::Stopwatch;

    #[test]
    fn download_db() {
        let mut stopwatch = Stopwatch::start_new();
        println!("Getting db");
        assert!(TokioRuntime.block_on(beatstar_update_database()).is_none());
        println!("Got DB, took {0}ms", stopwatch.elapsed().as_millis());
        stopwatch.restart();
        assert!(TokioRuntime.block_on(beatstar_update_database()).is_none());
        assert!(stopwatch.elapsed().as_millis() < 1000);
        println!("Memory Cache works");
    }

    #[test]
    fn download_song_stars() {
        download_db();
        // assert_eq!(2 + 2, 4);

        let song = TokioRuntime.block_on(beatstar_get_song(
            "4B2DA842B687EC4CFBC948C583C21C79D4120DE0",
        ));

        unsafe {
            let diff = (*song
                .expect("Could not fetch song database")
                .expect("Could not find song in database")
                .diffs)[0]
                .clone();
            println!("Got the stars: {0} {1}", diff.diff.to_string(), diff.stars)
        }
    }
}
