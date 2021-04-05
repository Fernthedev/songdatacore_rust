use crate::beatstar::data::BeatStarSongJson;
use crate::beatstar::ffi::{BeatStarDataFile, BeatStarSong, RustCStringWrapper};
use crate::beatstar::BEAT_STAR_FILE;
use crate::beatstar::TokioRuntime;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use stopwatch::Stopwatch;
use self::surf::Response;


extern crate chrono;
extern crate tokio;
extern crate surf;

pub const SCRAPED_SCORE_SABER_URL: &str = "https://raw.githubusercontent.com/andruzzzhka/BeatSaberScrappedData/master/combinedScrappedData.json";

const HTTP_OK: u16 = 200;



///
/// Returns None unless error, in which you get the response
///
/// Fetches the latest song data and stores it indefinitely
///
pub async fn beatstar_update_database() -> Option<Response> {
    if BEAT_STAR_FILE.get().is_none() {
        println!("Fetching from internet");
        let mut stopwatch = Stopwatch::start_new();

        // Await the response...
        let mut response = surf::get(SCRAPED_SCORE_SABER_URL).await.unwrap();
        println!("Response: {}", response.status());

        println!(
            "Received data from internet in {0}ms",
            stopwatch.elapsed().as_millis()
        );

        if response.status() == HTTP_OK {
            let mut stopwatch2 = Stopwatch::start_new();

            // while let Some(chunk) = response.body_mut().data().await {
            //     stdout().write_all(&chunk.unwrapno()).await.unwrap();
            // }

            let body: Vec<BeatStarSongJson> = response.body_json().await.unwrap();

            println!(
                "Parsed beat file into json data in {0}ms ({1}ms)",
                stopwatch.elapsed().as_millis(),
                stopwatch2.elapsed().as_millis()
            );

            stopwatch2.stop();

            let parsed_data = parse_beatstar(&body);

            BEAT_STAR_FILE.get_or_init(|| parsed_data);

            println!(
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
    return match TokioRuntime.block_on(beatstar_retrieve_database()) {
        Ok(e) => e,
        Err(e) => panic!("Unable to fetch from database {0}", e.status()),
    };
}

///
/// Get the song list and clone it
///
pub async fn beatstar_retrieve_database() -> Result<&'static BeatStarDataFile, Response> {
    if let Some(e) = beatstar_update_database().await {
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

    match TokioRuntime.block_on(beatstar_get_song(hash_str)) {
        Ok(e) => match e {
            None => ptr::null(),
            Some(e) => e,
        },
        Err(e) => panic!("Unable to fetch from database {0}", e.status()),
    }
}

///
/// Gets a song based on it's hash
///
pub async fn beatstar_get_song(hash: &str) -> Result<Option<&BeatStarSong>, Response> {
    unsafe {
        return match beatstar_update_database().await {
            None => {
                Ok((*BEAT_STAR_FILE.get().unwrap().songs)
                    .get(&RustCStringWrapper::new(hash.into())))
            }
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
        song_converted.push(BeatStarSong::convert(&song))
    }

    let mut song_map: HashMap<RustCStringWrapper, BeatStarSong> = HashMap::new();

    for song in song_converted {
        song_map.insert(song.hash.clone(), song);
    }

    BeatStarDataFile {
        songs: Box::into_raw(Box::new(song_map)),
    }
}
