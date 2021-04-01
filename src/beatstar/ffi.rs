use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::os::raw::c_char;

//noinspection RsExternalLinter
#[macro_use]
use crate::vec_extern;

//noinspection RsExternalLinter
#[macro_use]
use crate::map_extern;
use crate::beatstar::data::{
    BeatStarCharacteristics, BeatStarSongDifficultyStatsJson, BeatStarSongJson,
};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::str::FromStr;
use std::{ptr, result};

#[derive(Eq)]
#[repr(C)]
pub struct RustCStringWrapper {
    pub string_data: *mut c_char,
}

impl Clone for RustCStringWrapper {
    fn clone(&self) -> Self {
        RustCStringWrapper::new(self.to_string().into())
    }
}

impl PartialEq for RustCStringWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl PartialOrd for RustCStringWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl Hash for RustCStringWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

unsafe impl Send for RustCStringWrapper {}
unsafe impl Sync for RustCStringWrapper {}

impl ToString for RustCStringWrapper {
    fn to_string(&self) -> String {
        if self.string_data.is_null() {
            return String::new();
        }

        let raw = unsafe { CStr::from_ptr(self.string_data) };

        let hash_str = match raw.to_str() {
            Ok(s) => s,
            Err(_) => return String::new(),
        };

        hash_str.to_string()
    }
}

impl Drop for RustCStringWrapper {
    fn drop(&mut self) {
        if self.string_data.is_null() {
            return;
        }
        unsafe {
            CString::from_raw(self.string_data);
        }
    }
}

impl RustCStringWrapper {
    pub fn new(str_data: Vec<u8>) -> RustCStringWrapper {
        let c_string = CString::new(str_data).expect("RustCStringWrapper::new failed");
        let ptr = c_string.into_raw();
        RustCStringWrapper { string_data: ptr }
    }

    #[no_mangle]
    pub extern "C" fn c_new(c_str: *mut c_char) -> RustCStringWrapper {
        unsafe {
            RustCStringWrapper {
                string_data: CString::from_raw(c_str).into_raw(),
            }
        }
    }
}

#[repr(C)]
pub struct BeatStarDataFile {
    pub songs: HashMap<RustCStringWrapper, BeatStarSong>,
}

impl BeatStarDataFile {
    map_extern!(
        songs,
        RustCStringWrapper,
        BeatStarSong,
        extern_map_songs_get,
        extern_map_songs_len,
        extern_map_songs_get_key
    );
}

#[repr(C)]
pub struct BeatStarSong {
    pub bpm: f32,
    pub played_count: u32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub key: RustCStringWrapper,
    pub diffs: Vec<BeatStarSongDifficultyStats>,
    pub uploaded: RustCStringWrapper,
    pub hash: RustCStringWrapper,
    pub characteristics:
        HashMap<BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>>,
}

impl BeatStarSong {
    #[no_mangle]
    pub extern "C" fn rating(&self) -> f32 {
        let tot: f32 = (self.upvotes + self.downvotes) as f32;
        let tmp: f32 = (self.upvotes) as f32 / tot;

        tmp - (tmp - 0.5) * (2_i32.pow(-(tot + 1f32).log10() as u32) as f32)
    }

    vec_extern!(
        diffs,
        BeatStarSongDifficultyStats,
        extern_vec_diffs_get,
        extern_vec_diffs_len
    );
    map_extern!(characteristics, BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>, extern_map_songs_get, extern_map_songs_len, extern_map_characteristics_key_get);

    #[no_mangle]
    pub extern "C" fn get_characteristics_len(&self, beat_char: &BeatStarCharacteristics) -> usize {
        return match self.characteristics.get(beat_char) {
            None => 0,
            Some(e) => e.len(),
        };
    }

    #[no_mangle]
    pub extern "C" fn get_characteristics_str(
        &self,
        beat_char: &BeatStarCharacteristics,
        index: usize,
    ) -> *const c_char {
        return match self.characteristics.get(beat_char) {
            None => ptr::null(),
            Some(e) => {
                let keys: Vec<&RustCStringWrapper> = e.keys().collect();

                match keys.get(index) {
                    None => ptr::null(),
                    Some(s) => {
                        return s.string_data;
                    }
                }
            }
        };
    }

    #[no_mangle]
    pub extern "C" fn get_characteristic_stats(
        &self,
        beat_char: &BeatStarCharacteristics,
        beat_key2: *mut c_char,
    ) -> *const BeatStarSongDifficultyStats {
        return match self.characteristics.get(beat_char) {
            Some(map) => match map.get(&RustCStringWrapper::c_new(beat_key2)) {
                None => ptr::null(),
                Some(e) => e,
            },
            None => ptr::null(),
        };
    }

    pub fn convert(og: &BeatStarSongJson) -> BeatStarSong {
        let mut diff_convert: Vec<BeatStarSongDifficultyStats> = vec![];

        for diff in &og.diffs {
            diff_convert.push(BeatStarSongDifficultyStats::convert(diff))
        }

        let mut characteristics_convert: HashMap<
            BeatStarCharacteristics,
            HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>,
        > = HashMap::new();

        for (char_star, char_map) in og.characteristics.borrow() {
            let mut char_map_convert: HashMap<RustCStringWrapper, BeatStarSongDifficultyStats> =
                HashMap::new();

            for (str, diff_json) in char_map {
                char_map_convert.insert(
                    RustCStringWrapper::new(str.clone().into()),
                    BeatStarSongDifficultyStats::convert(&diff_json),
                );
            }

            characteristics_convert.insert(*char_star, char_map_convert);
        }

        BeatStarSong {
            bpm: og.bpm,
            played_count: og.played_count,
            upvotes: og.upvotes,
            downvotes: og.downvotes,
            key: RustCStringWrapper::new(og.key.clone().into()),
            diffs: diff_convert,
            uploaded: RustCStringWrapper::new(og.uploaded.clone().into()),
            hash: RustCStringWrapper::new(og.hash.clone().into()),
            characteristics: characteristics_convert,
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct BeatStarSongDifficultyStats {
    pub diff: RustCStringWrapper,
    pub scores: i64,
    pub stars: f64,
    pub ranked: bool,
    pub njs: f32,
    pub bombs: u32,
    pub notes: u32,
    pub obstacles: u32,
    pub char: RustCStringWrapper,
}

impl BeatStarSongDifficultyStats {
    #[no_mangle]
    pub extern "C" fn get_diff_type(&self) -> BeatStarCharacteristics {
        return match BeatStarCharacteristics::from_str(self.char.to_string().as_str()) {
            Ok(e) => e,
            Err(_) => BeatStarCharacteristics::Unknown,
        };
    }

    pub fn convert(og: &BeatStarSongDifficultyStatsJson) -> BeatStarSongDifficultyStats {
        BeatStarSongDifficultyStats {
            diff: RustCStringWrapper::new(og.diff.clone().into()),
            scores: og.scores,
            stars: og.stars,
            ranked: og.ranked,
            njs: og.njs,
            bombs: og.bombs,
            notes: og.notes,
            obstacles: og.obstacles,
            char: RustCStringWrapper::new(og.char.clone().into()),
        }
    }
}

impl FromStr for BeatStarCharacteristics {
    type Err = ();

    fn from_str(input: &str) -> result::Result<BeatStarCharacteristics, Self::Err> {
        match input.to_lowercase().as_str() {
            "unknown" => Ok(BeatStarCharacteristics::Unknown),
            "standard" => Ok(BeatStarCharacteristics::Standard),
            "onesaber" => Ok(BeatStarCharacteristics::OneSaber),
            "noarrows" => Ok(BeatStarCharacteristics::NoArrows),
            "lightshow" => Ok(BeatStarCharacteristics::Lightshow),
            "degree90" => Ok(BeatStarCharacteristics::Degree90),
            "degree360" => Ok(BeatStarCharacteristics::Degree360),
            "lawless" => Ok(BeatStarCharacteristics::Lawless),
            _ => Err(()),
        }
    }
}
