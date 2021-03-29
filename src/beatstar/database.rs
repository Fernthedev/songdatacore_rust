use crate::beatstar::data::{BeatStarDataFile, BeatStarSong};
use crate::beatstar::BEAT_STAR_FILE;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use stopwatch::Stopwatch;
use ureq::{Agent, Response};

extern crate chrono;

pub const SCRAPED_SCORE_SABER_URL: &str = "https://raw.githubusercontent.com/andruzzzhka/BeatSaberScrappedData/master/combinedScrappedData.json";
pub const MAX_DELAY: u64 = 60 * 60 * 12; // 60 seconds * 60 seconds = hour * 12 = 12 hours

static mut LAST_FETCH: SystemTime = SystemTime::UNIX_EPOCH;

const HTTP_OK: u16 = 200;

lazy_static! {
    static ref AGENT: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
}

#[no_mangle]
pub unsafe extern fn beatstar_fetch_database() -> Result<BeatStarDataFile, Response> {
    if LAST_FETCH.elapsed().unwrap().as_secs() > MAX_DELAY || BEAT_STAR_FILE.is_none() {
        let response = AGENT.get(SCRAPED_SCORE_SABER_URL).call().unwrap();

        if response.status() == HTTP_OK {
            let mut stopwatch = Stopwatch::start_new();

            let body = response.into_string().unwrap();

            BEAT_STAR_FILE = Some(parse_beatstar(body.as_str()));

            stopwatch.stop();
            LAST_FETCH = SystemTime::now();
        } else {
            return Err(response);
        }
    }
    let bsf = BEAT_STAR_FILE.borrow().as_ref().unwrap();

    return Ok(bsf.clone());
}

fn parse_beatstar(input: &str) -> BeatStarDataFile {
    let songs: Vec<BeatStarSong> = serde_json::from_str(input).unwrap();
    let mut song_map: HashMap<String, BeatStarSong> = HashMap::new();

    for mut song in songs {
        song.characteristics = HashMap::new();

        for diff in &mut song.diffs {
            let diff_type = diff.get_diff_type();

            if !song.characteristics.contains_key(&diff_type) {
                song.characteristics.insert(diff_type, HashMap::new());
            }

            song.characteristics
                .get_mut(&diff_type)
                .unwrap()
                .insert(diff.diff.clone(), diff.clone());
        }

        song_map.insert(song.hash.clone(), song);
    }

    return BeatStarDataFile { songs: song_map };
}
