use crate::beatstar::data::{BeatStarCharacteristics, BeatStarSongDifficultyStatsJson, BeatStarSongJson};
use crate::beatstar::ffi::{BeatStarDataFile, BeatStarSong, RustCStringWrapper};
use crate::beatstar::BEAT_STAR_FILE;
use std::collections::HashMap;
use std::ffi::CStr;
use std::io::{Cursor, Read};
use std::os::raw::c_char;
use std::ptr;
use std::str::FromStr;
use std::time::Duration;
use anyhow::Context;
use stopwatch::Stopwatch;
use tracing::{Level, event, span};
use ureq::{Agent, Response};

extern crate chrono;

pub const SCRAPED_SCORE_SABER_URL: &str = "https://github.com/andruzzzhka/BeatSaberScrappedData/blob/master/combinedScrappedData.zip?raw=true";

const HTTP_OK: u16 = 200;

lazy_static! {
    static ref AGENT: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
}

pub fn beatstar_zip_content(response: ureq::Response) -> Result<Vec<BeatStarSongJson>, anyhow::Error>  {
    assert!(response.has("Content-Length"));
    let len = response.header("Content-Length")
    .and_then(|s| s.parse::<usize>().ok()).unwrap();

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    response.into_reader()
        .read_to_end(&mut bytes)?;


    let cursor = Cursor::new(&bytes);
    
    let mut zip = zip::ZipArchive::new(cursor).unwrap();

    assert!(!zip.is_empty());

    let mut file = zip.by_index(0)?;

    let mut string_buffer = Vec::new();

    file.read_to_end(&mut string_buffer)?;

    let mut json: Vec<BeatStarSongJson> = serde_json::from_slice(string_buffer.as_slice())?;

    for song in &mut json {
        
        type DiffMap = HashMap<String, BeatStarSongDifficultyStatsJson>;

        let mut characteristics: HashMap<BeatStarCharacteristics, DiffMap> = HashMap::new();



        'diffLoop: for diff in &song.diffs {
            let char = BeatStarCharacteristics::from_str(diff.char.as_str());

            if char.is_err() {
                continue 'diffLoop;
            }
            

            let char_map  = characteristics.entry(char.unwrap()).or_insert_with(DiffMap::new);

            char_map.insert(diff.diff.clone(), diff.clone());
        }

        song.characteristics = characteristics;
    };

    Ok(json)
}

#[cfg(target_os = "android")]
fn initialize_log() {
    tracing_android::init(env!("CARGO_PKG_NAME"));
}

#[cfg(not(target_os = "android"))]
fn initialize_log() {
    tracing_subscriber::fmt::init();
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
        event!(Level::INFO, 
            "Received data from internet in {0}ms",
            stopwatch.elapsed().as_millis()
        );

        if response.status() == HTTP_OK {
            let body: Vec<BeatStarSongJson> = beatstar_zip_content(response).context("Failed to parse scrapped beat saver data zip.").unwrap();
            event!(Level::INFO,
                "Parsed beat file into json data in {0}ms",
                stopwatch.elapsed().as_millis()
            );


            // Get data inside zip
            let parsed_data = parse_beatstar(&body);

            BEAT_STAR_FILE.get_or_init(|| parsed_data);

            event!(
                Level::INFO,
                "Fully parsed beat file in {0}ms",
                stopwatch.elapsed().as_millis()
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
#[no_mangle]
pub extern "C" fn Beatstar_RetrieveDatabase() -> *const BeatStarDataFile {
    match beatstar_retrieve_database() {
        Ok(e) => e,
        Err(e) => panic!(
            "Unable to fetch from database {0}",
            e.into_string().unwrap()
        ),
    }
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
/// Get the song based on hash
///
///
#[no_mangle]
pub unsafe extern "C" fn Beatstar_GetSong(hash: *const c_char) -> *const BeatStarSong {
    if hash.is_null() {
        return ptr::null_mut();
    }

    let raw = CStr::from_ptr(hash);

    let hash_str = match raw.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match beatstar_get_song(hash_str) {
        Ok(e) => match e {
            None => ptr::null(),
            Some(e) => e,
        },
        Err(e) => panic!(
            "Unable to fetch from database {0}",
            e.into_string().unwrap()
        ),
    }
}

///
/// Gets a song based on it's hash
///
pub fn beatstar_get_song(hash: &str) -> Result<Option<&BeatStarSong>, Response> {
    unsafe {
        return match beatstar_update_database() {
            None => Ok((*BEAT_STAR_FILE
                .get()
                .unwrap()
                .songs)
                .get(&RustCStringWrapper::new(hash.into()))),
            Some(e) => Err(e),
        };
    }
}

///
/// Parses the entire JSON
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

    BeatStarDataFile { songs: Box::into_raw(Box::new(song_map)) }
}
