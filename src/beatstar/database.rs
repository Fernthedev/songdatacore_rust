use crate::beatstar::data::{
    BeatStarCharacteristics, BeatStarSongDifficultyStatsJson, BeatStarSongJson, UnixTime,
};
use crate::beatstar::ffi::{BeatStarDataFile, BeatStarSong, RustCStringWrapper};
use crate::beatstar::BEAT_STAR_FILE;
use anyhow::Context;
use chrono::DateTime;
use std::collections::HashMap;
use std::io::{BufReader, Cursor, Read};
use std::ops::Sub;
use std::str::FromStr;
use std::sync::Once;
use std::time::{Duration as OtherDuration, SystemTime};
use stopwatch::Stopwatch;
use tracing::{event, span, Level};
use ureq::{Agent, Response};

use super::numstuff::log10;

extern crate chrono;

pub const SCRAPED_SCORE_SABER_URL: &str = "https://github.com/andruzzzhka/BeatSaberScrappedData/blob/master/combinedScrappedData.zip?raw=true";
const HTTP_OK: u16 = 200;
static INIT_LOG: Once = Once::new();
static BEATSAVER_EPOCH: std::time::Duration = std::time::Duration::from_secs(1525132800);

lazy_static! {
    static ref AGENT: Agent = ureq::AgentBuilder::new()
        .timeout_read(OtherDuration::from_secs(5))
        .timeout_write(OtherDuration::from_secs(5))
        .build();
}

fn calculate_pp(diff: &BeatStarSongDifficultyStatsJson) -> f32 {
    if diff.stars <= 0.05 || !diff.ranked {
        return 0.0;
    }

    diff.stars * (45.0 + ((10.0 - diff.stars) / 7.0))
}

// https://github.com/bsmg/beatsaver-reloaded/blob/420be0c964f3b4ee9c876f8b7fdb25495526138d/server/src/mongo/models/Beatmap.ts#L172-L177
///
/// An algorithm for getting a song's rating.
///
fn calculate_rating(self_i: &BeatStarSongJson) -> f32 {
    let tot: f32 = (self_i.upvotes + self_i.downvotes) as f32;
    let tmp: f32 = (self_i.upvotes) as f32 / tot;

    tmp - (tmp - 0.5) * (2_f32.powf(-(tot + 1f32).log10()) as f32)
}

// https://github.com/bsmg/beatsaver-reloaded/blob/420be0c964f3b4ee9c876f8b7fdb25495526138d/server/src/mongo/models/Beatmap.ts#L179-L192
fn calculate_heatmap(
    song: &BeatStarSongJson,
    time_past_epoch: UnixTime,
    uploaded_date: UnixTime,
) -> Result<f32, anyhow::Error> {
    let seconds_diff = uploaded_date - time_past_epoch;

    let score = song.upvotes as i64 - song.downvotes as i64;

    let sign = match score {
        1.. => -1,
        0_i64 => 0,
        i64::MIN..=-1_i64 => -1,
    };

    let order = log10(score.max(1));
    let heat = sign as f64 * order as f64 + seconds_diff as f64 / 45000f64;

    Ok(heat as f32)
}

pub fn beatstar_zip_content(
    response: ureq::Response,
) -> Result<Vec<BeatStarSongJson>, anyhow::Error> {
    assert!(response.has("Content-Length"));
    let len = response
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap();

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    response.into_reader().read_to_end(&mut bytes)?;
    let cursor = Cursor::new(&bytes);

    let mut zip = zip::ZipArchive::new(cursor).unwrap();
    assert!(!zip.is_empty());
    let file = zip.by_index(0)?;
    let reader = BufReader::new(file);

    // let mut string_buffer = Vec::new();
    // file.read_to_end(&mut string_buffer)?;

    let mut json: Vec<BeatStarSongJson> = serde_json::from_reader(reader)?;
    let time_past_epoch = SystemTime::now().sub(BEATSAVER_EPOCH).elapsed()?.as_secs() as UnixTime;

    for song in &mut json {
        // sort characteristics
        type DiffMap = HashMap<String, BeatStarSongDifficultyStatsJson>;

        let mut characteristics: HashMap<BeatStarCharacteristics, DiffMap> = HashMap::new();

        'diffLoop: for diff in &mut song.diffs {
            let char = BeatStarCharacteristics::from_str(diff.char.as_str());

            if char.is_err() {
                event!(
                    Level::ERROR,
                    "Could not parse characteristic {0} for song {1}",
                    diff.char.as_str(),
                    song.hash.as_str()
                );
                continue 'diffLoop;
            }

            let char_map = characteristics
                .entry(char.unwrap())
                .or_insert_with(DiffMap::new);

            // calculate approximate PP
            diff.approximate_pp_value = calculate_pp(diff);

            let ranked_time = DateTime::parse_from_rfc3339(&song.uploaded)?.timestamp() as UnixTime;
            diff.ranked_update_time_unix_epoch = ranked_time;

            char_map.insert(diff.diff.clone(), diff.clone());
        }

        song.characteristics = characteristics;

        // calculate heatmap
        let upload_unix_time =
            DateTime::parse_from_rfc3339(&song.uploaded)?.timestamp() as UnixTime;
        song.heat = calculate_heatmap(song, time_past_epoch, upload_unix_time).unwrap_or(0f32);
        song.uploaded_unix_time = upload_unix_time;
        song.rating = calculate_rating(song);
    }

    Ok(json)
}

#[cfg(target_os = "android")]
pub(crate) fn initialize_log() {
    INIT_LOG.call_once(|| {
        tracing_android::init(env!("CARGO_PKG_NAME"));
    });
}

#[cfg(not(target_os = "android"))]
pub(crate) fn initialize_log() {
    INIT_LOG.call_once(|| {
        tracing_subscriber::fmt::init();
    });
}

///
/// Returns None unless error, in which you get the response
///
/// Fetches the latest song data and stores it indefinitely
///
pub fn beatstar_update_database() -> Option<Response> {
    if BEAT_STAR_FILE.get().is_none() {
        initialize_log();

        let span = span!(Level::TRACE, "beatstar_database_update");
        let _guard = span.enter();

        event!(Level::INFO, "Fetching from internet");
        let mut stopwatch = Stopwatch::start_new();
        let response = AGENT.get(SCRAPED_SCORE_SABER_URL).call().unwrap();
        event!(
            Level::INFO,
            "Received data from internet in {0}ms",
            stopwatch.elapsed().as_millis()
        );

        if response.status() == HTTP_OK {
            let body: Vec<BeatStarSongJson> = beatstar_zip_content(response)
                .context("Failed to parse scrapped beat saver data zip.")
                .unwrap();
            event!(
                Level::INFO,
                "Parsed beat file into json data in {0}ms",
                stopwatch.elapsed().as_millis()
            );

            // Get data inside file and map it
            let parsed_data = parse_beatstar(&body);

            let json_size = std::mem::size_of_val::<BeatStarDataFile>(&parsed_data);

            BEAT_STAR_FILE.get_or_init(|| parsed_data);

            event!(
                Level::INFO,
                "Fully parsed beat file in {0}ms (json size: {1}kb)",
                stopwatch.elapsed().as_millis(),
                json_size / 1024
            );

            stopwatch.stop();
        } else {
            return Some(response);
        }
    }

    None
}

///
/// Get the song list and clone it
///
pub fn beatstar_retrieve_database() -> Result<&'static BeatStarDataFile, Response> {
    if let Some(e) = beatstar_update_database() {
        return Err(e);
    }

    let bsf_mutex = BEAT_STAR_FILE.get().unwrap();

    Ok(bsf_mutex)
}

///
/// Gets a song based on it's hash
///
pub fn beatstar_get_song(hash: &str) -> Result<Option<&BeatStarSong>, Response> {
    unsafe {
        return match beatstar_update_database() {
            None => {
                // Get songs map
                Ok((*BEAT_STAR_FILE.get().unwrap().songs)
                    .get(&RustCStringWrapper::new(hash.into())))
            }
            Some(e) => Err(e),
        };
    }
}

///
/// Parses the entire JSON to FFI friendly types
/// This takes an average of 700 MS, do better?
///
fn parse_beatstar(songs: &[BeatStarSongJson]) -> BeatStarDataFile {
    let mut song_converted: Vec<BeatStarSong> = vec![];

    for song in songs {
        song_converted.push(BeatStarSong::convert(song))
    }

    let mut song_map: HashMap<RustCStringWrapper, BeatStarSong> = HashMap::new();

    for song in song_converted {
        song_map.insert(song.hash.clone(), song);
    }

    BeatStarDataFile {
        songs: Box::into_raw(Box::new(song_map)),
    }
}
