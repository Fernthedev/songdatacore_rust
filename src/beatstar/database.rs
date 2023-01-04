use crate::beatstar::data::{BeatStarCharacteristics, UnixTime};
use crate::beatstar::ffi::{
    BeatStarDataFile, BeatStarSong, BeatStarSongDifficultyStats, RustCStringWrapper,
};
use crate::beatstar::BEAT_STAR_FILE;
use anyhow::{anyhow, bail, Context};
use chrono::DateTime;
use std::collections::HashMap;
use std::ffi::CStr;
use std::io::{BufReader, Cursor, Read};

use std::ops::Sub;
use std::str::FromStr;
use std::sync::Once;
use std::time::{Duration as OtherDuration, SystemTime};
use stopwatch::Stopwatch;
use tracing::{event, span, Level};
use ureq::{Agent, Response};

use super::numstuff::log10;
// use super::BEAT_STAR_MUTEX;

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

fn calculate_pp(diff: &BeatStarSongDifficultyStats) -> f32 {
    if diff.stars <= 0.05 || !diff.ranked {
        return 0.0;
    }

    diff.stars * (45.0 + ((10.0 - diff.stars) / 7.0))
}

// https://github.com/bsmg/beatsaver-reloaded/blob/420be0c964f3b4ee9c876f8b7fdb25495526138d/server/src/mongo/models/Beatmap.ts#L172-L177
///
/// An algorithm for getting a song's rating.
///
fn calculate_rating(self_i: &BeatStarSong) -> f32 {
    let tot: f32 = (self_i.upvotes + self_i.downvotes) as f32;
    let tmp: f32 = (self_i.upvotes) as f32 / tot;

    tmp - (tmp - 0.5) * 2_f32.powf(-(tot + 1f32).log10())
}

// https://github.com/bsmg/beatsaver-reloaded/blob/420be0c964f3b4ee9c876f8b7fdb25495526138d/server/src/mongo/models/Beatmap.ts#L179-L192
fn calculate_heatmap(
    song: &BeatStarSong,
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

#[inline(always)]
pub fn beatstar_zip_content_network(response: Response) -> anyhow::Result<Vec<BeatStarSong>> {
    assert!(response.has("Content-Length"));
    let len = response
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or_else(|| anyhow!("Unable to get header response for content length"))?;

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    response.into_reader().read_to_end(&mut bytes)?;

    beatstar_zip_content(bytes)
}

pub fn beatstar_zip_content(bytes: Vec<u8>) -> anyhow::Result<Vec<BeatStarSong>> {
    let cursor = Cursor::new(&bytes);

    let mut zip = zip::ZipArchive::new(cursor).context("Unable to read zip archive")?;
    assert!(!zip.is_empty());
    let file = zip.by_index(0)?;
    let mut reader = BufReader::new(file);

    // let mut string_buffer = Vec::new();
    // file.read_to_end(&mut string_buffer)?;

    let mut str = String::with_capacity(8 * 1024 * 1024);
    reader.read_to_string(&mut str)?;

    let mut json: Vec<BeatStarSong> = serde_json::from_str(&str)?;
    let time_past_epoch = SystemTime::now().sub(BEATSAVER_EPOCH).elapsed()?.as_secs() as UnixTime;

    for song in &mut json {
        // sort characteristics
        type DiffMap = HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>;

        let mut characteristics: HashMap<BeatStarCharacteristics, DiffMap> = HashMap::new();

        'diffLoop: for diff in &mut song.diffs {
            let char = BeatStarCharacteristics::from_str(diff.char.to_string().as_str());

            if char.is_err() {
                event!(
                    Level::ERROR,
                    "Could not parse characteristic {0} for song {1}",
                    diff.char.to_string().as_str(),
                    song.hash.to_string().as_str()
                );
                continue 'diffLoop;
            }

            diff.diff_characteristics = char.unwrap();

            let char_entry = characteristics
                .entry(char.unwrap())
                .or_insert_with(DiffMap::new);

            // calculate approximate PP
            diff.approximate_pp_value = calculate_pp(diff);

            let ranked_time = DateTime::parse_from_rfc3339(song.uploaded.to_string().as_str())?
                .timestamp() as UnixTime;
            diff.ranked_update_time_unix_epoch = ranked_time;

            char_entry.insert(diff.diff.clone(), diff.clone());
        }

        song.characteristics = characteristics;

        // calculate heatmap
        let upload_unix_time = DateTime::parse_from_rfc3339(song.uploaded.to_string().as_str())?
            .timestamp() as UnixTime;
        song.heat = calculate_heatmap(song, time_past_epoch, upload_unix_time).unwrap_or(0f32);
        song.uploaded_unix_time = upload_unix_time;
        song.rating = calculate_rating(song);
    }

    Ok(json)
}

#[cfg(target_os = "android")]
pub(crate) fn initialize_log() {
    INIT_LOG.call_once(|| {
        paranoid_android::init(env!("CARGO_PKG_NAME"));
    });
}

#[cfg(not(target_os = "android"))]
pub(crate) fn initialize_log() {
    INIT_LOG.call_once(|| {
        tracing_subscriber::fmt::init();
    });
}

pub fn beatstar_download_database_to_file(file_path: &str) -> anyhow::Result<()>  {
    initialize_log();

    let span = span!(Level::TRACE, "beatstar_database_update");
    let _guard = span.enter();

    event!(Level::INFO, "Fetching from internet");
    let mut stopwatch = Stopwatch::start_new();
    let response = AGENT.get(SCRAPED_SCORE_SABER_URL).call()?;
    event!(
        Level::INFO,
        "Received data from internet in {0}ms",
        stopwatch.elapsed().as_millis()
    );

    // TODO: Get latest change and return it to not reload it again
    if response.status() != HTTP_OK {
        bail!("Did not receive HTTP_OK status. {:?}", response);
    } 

    assert!(response.has("Content-Length"));
    let len = response
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or_else(|| anyhow!("Unable to get header response for content length"))?;

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    response.into_reader().read_to_end(&mut bytes)?;

    std::fs::write(file_path, &bytes)?;

    BEAT_STAR_FILE.get_or_try_init(|| -> anyhow::Result<_> {
        let body = beatstar_zip_content(bytes)?;

        Ok(parse_beatstar(body))
    })?;


    stopwatch.stop();
    Ok(())

}

///
/// Returns None unless error, in which you get the response
///
/// Fetches the latest song data and stores it indefinitely
///
pub fn beatstar_update_database_network() -> anyhow::Result<()> {
    // If already initialized
    // if BEAT_STAR_FILE.get().is_some() {
    //     return Ok(());
    // }

    // let lock = match BEAT_STAR_MUTEX.try_lock() {
    //     Ok(guard) => guard,
    //     Err(_) => {
    //         drop(BEAT_STAR_MUTEX.lock().unwrap());
    //         return Ok(());
    //     }
    // };

    // This is fine, since while multiple threads can call it, only one function gets executed
    // the rest block
    BEAT_STAR_FILE.get_or_try_init(|| -> anyhow::Result<_> {

        initialize_log();

        let span = span!(Level::TRACE, "beatstar_database_update");
        let _guard = span.enter();

        event!(Level::INFO, "Fetching from internet");
        let mut stopwatch = Stopwatch::start_new();
        let response = AGENT.get(SCRAPED_SCORE_SABER_URL).call()?;
        event!(
            Level::INFO,
            "Received data from internet in {0}ms",
            stopwatch.elapsed().as_millis()
        );

        if response.status() != HTTP_OK {
            bail!("Did not receive HTTP_OK status. {:?}", response);
        }

        let body: Vec<BeatStarSong> = beatstar_zip_content_network(response)
            .context("Failed to parse scrapped beat saver data zip.")?;
        event!(
            Level::INFO,
            "Parsed beat file into json data in {0}ms",
            stopwatch.elapsed().as_millis()
        );

        // Get data inside file and map it
        let parsed_data = parse_beatstar(body);

        let json_size = parsed_data.songs.iter()
                .map(|(str, diff)| unsafe { CStr::from_ptr(str.string_data).to_bytes().len() } + std::mem::size_of_val(diff))
                .reduce(|acc, i| acc + i).unwrap_or(0);

        event!(
            Level::INFO,
            "Fully parsed beat file in {0}ms (json size: {1}kb)",
            stopwatch.elapsed().as_millis(),
            json_size / 1024
        );

        stopwatch.stop();
        Ok(parsed_data)
    })?;

    // drop(lock);

    Ok(())
}

///
/// Get the song list and clone it
///
pub fn beatstar_retrieve_database() -> anyhow::Result<&'static BeatStarDataFile> {
    beatstar_update_database_network()?;

    let bsf_mutex = BEAT_STAR_FILE
        .get()
        .ok_or_else(|| anyhow!("Unable to read beat star file"))?;

    Ok(bsf_mutex)
}


pub fn beatstar_update_database_file(file_path: &str) -> anyhow::Result<()> {
    // If already initialized
    // if BEAT_STAR_FILE.get().is_some() {
    //     return Ok(());
    // }

    // let lock = match BEAT_STAR_MUTEX.try_lock() {
    //     Ok(guard) => guard,
    //     Err(_) => {
    //         drop(BEAT_STAR_MUTEX.lock().unwrap());
    //         return Ok(());
    //     }
    // };

    // This is fine, since while multiple threads can call it, only one function gets executed
    // the rest block
    BEAT_STAR_FILE.get_or_try_init(|| -> anyhow::Result<_> {

        initialize_log();

        let span = span!(Level::TRACE, "beatstar_database_update");
        let _guard = span.enter();

        event!(Level::INFO, "Fetching from file");
        let mut stopwatch = Stopwatch::start_new();
        
        // Read zip from path
        let bytes: Vec<u8> = std::fs::read(file_path)?;

        let body: Vec<BeatStarSong> = beatstar_zip_content(bytes)
            .context("Failed to parse scrapped beat saver data zip.")?;
        event!(
            Level::INFO,
            "Parsed beat file into json data in {0}ms",
            stopwatch.elapsed().as_millis()
        );

        // Get data inside file and map it
        let parsed_data = parse_beatstar(body);

        let json_size = parsed_data.songs.iter()
                .map(|(str, diff)| unsafe { CStr::from_ptr(str.string_data).to_bytes().len() } + std::mem::size_of_val(diff))
                .reduce(|acc, i| acc + i).unwrap_or(0);

        event!(
            Level::INFO,
            "Fully parsed beat file in {0}ms (json size: {1}kb)",
            stopwatch.elapsed().as_millis(),
            json_size / 1024
        );

        stopwatch.stop();
        Ok(parsed_data)
    })?;

    // drop(lock);

    Ok(())
}

///
/// Get the song list and clone it
///
pub fn beatstar_retrieve_database_from_file(file_path: &str) -> anyhow::Result<&'static BeatStarDataFile> {
    beatstar_update_database_file(file_path)?;

    let bsf_mutex = BEAT_STAR_FILE
        .get()
        .ok_or_else(|| anyhow!("Unable to read beat star file"))?;

    Ok(bsf_mutex)
}

///
/// Gets a song based on it's hash
///
pub fn beatstar_get_song(hash: &str) -> anyhow::Result<Option<&BeatStarSong>> {
    beatstar_update_database_network()?;
    Ok((BEAT_STAR_FILE
        .get()
        .ok_or_else(|| anyhow!("Unable to read beat star file"))?
        .songs)
        .get(&RustCStringWrapper::new(hash)))
}

///
/// Parses the entire JSON to FFI friendly types
/// This takes an average of 700 MS, do better?
///
fn parse_beatstar(songs: Vec<BeatStarSong>) -> BeatStarDataFile {
    let song_map: HashMap<RustCStringWrapper, BeatStarSong> = songs
        .into_iter()
        .map(|s: BeatStarSong| (s.hash.clone(), s))
        .collect();

    BeatStarDataFile { songs: song_map }
}
