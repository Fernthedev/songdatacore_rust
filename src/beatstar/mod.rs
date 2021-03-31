use crate::beatstar::data::BeatStarDataFile;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::ptr;
use std::hash::{Hash, BuildHasher};

mod data;
mod database;

static BEAT_STAR_FILE: OnceCell<BeatStarDataFile> = OnceCell::new();

#[no_mangle]
pub extern fn get_from_hashmap<K: Eq + Hash, V: Hash, S: BuildHasher>(map: &HashMap<K, V, S>, key: &K) -> *const V {
    return match map.get(key) {
        None => ptr::null(),
        Some(e) => {e}
    }
}

#[repr(C)]
pub struct VecStruct<T> {
    array: *const T,
    len: usize
}

#[no_mangle]
pub extern fn get_from_vec<T>(vec: &Vec<T>) -> VecStruct<T> {
    let slice = vec.as_slice();

    VecStruct { array:slice.as_ptr(), len: slice.len()}
}

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

        let diff = song.expect("Could not find song for test").diffs[0].clone();
        println!("Got the stars: {0} {1}", diff.diff.to_string(), diff.stars)

    }
}