use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::result;
use std::str::FromStr;
use std::ffi::{CString, CStr};
use std::borrow::Borrow;
use std::os::raw::c_char;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

// #[derive(Derivative)]
#[derive(Eq)]
#[repr(C)]
pub struct RustCStringWrapper {
    // #[derivative(PartialOrd="ignore")]
    // #[derivative(PartialEq="ignore")]
    // #[derivative(Hash="ignore")]
    pub string_data: *mut c_char
}

impl Clone for RustCStringWrapper {
    fn clone(&self) -> Self {
        unsafe {
            return RustCStringWrapper::new(self.to_string().into());
        }
    }
}

impl PartialEq for RustCStringWrapper {
    fn eq(&self, other: &Self) -> bool {
        return self.to_string() == other.to_string()
    }
}

impl PartialOrd for RustCStringWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.to_string().partial_cmp(&other.to_string())
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
            return String::new()
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
        unsafe { CString::from_raw(self.string_data); }
    }
}

impl RustCStringWrapper {
    pub fn new(str_data: Vec<u8>) -> RustCStringWrapper {
        let c_string  = CString::new(str_data).expect("RustCStringWrapper::new failed");
        let ptr = c_string.into_raw();
        RustCStringWrapper {
            string_data: ptr
        }
    }
}

#[repr(C)]
pub struct BeatStarDataFile {
    pub songs: HashMap<RustCStringWrapper, BeatStarSong>,
}

//
// impl BeatStarDataFile {
//     pub extern fn get_song(self, song: *const c_char) -> *const BeatStarSong {
//         return self.songs.get(&RustCStringWrapper::new(song)).unwrap_or_else(ptr::null());
//     }
// }


#[serde(rename_all = "PascalCase", rename = "with typo")]
#[derive(Serialize, Deserialize, Clone)]
pub struct BeatStarSongJson {
    #[serde(rename = "Bpm")]
    pub bpm: f32,

    #[serde(rename = "PlayedCount")]
    pub played_count: u32,

    #[serde(rename = "Upvotes")]
    pub upvotes: u32,

    #[serde(rename = "Downvotes")]
    pub downvotes: u32,

    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "Diffs")]
    pub diffs: Vec<BeatStarSongDifficultyStatsJson>,

    #[serde(rename = "Uploaded")]
    pub uploaded: String,

    #[serde(rename = "Hash")]

    pub hash: String,

    #[serde(skip_deserializing)]
    pub characteristics:
        HashMap<BeatStarCharacteristics, HashMap<String, BeatStarSongDifficultyStatsJson>>,
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
    pub characteristics: HashMap<BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>>,
}

impl BeatStarSong {
    #[no_mangle]
    pub extern fn rating(&self) -> f32 {
        let tot: f32 = (self.upvotes + self.downvotes) as f32;
        let tmp: f32 = (self.upvotes) as f32 / tot;

        tmp - (tmp - 0.5) * (2_i32.pow(-(tot + 1f32).log10() as u32) as f32)
    }

    // pub extern fn get_characteristic(self, beat_char: *BeatStarCharacteristics, beat_key2: *const c_char) -> &BeatStarSongDifficultyStats {
    //     return match self.characteristics.get(&beat_char) {
    //         Some(map) => map.get(&RustCStringWrapper::new(beat_key2)).unwrap_or_else(ptr::null()),
    //         None => ptr::null()
    //     }
    // }

    pub fn convert(og: &BeatStarSongJson) -> BeatStarSong {
        let mut diff_convert: Vec<BeatStarSongDifficultyStats> = vec![];

        for diff in &og.diffs {
            diff_convert.push(BeatStarSongDifficultyStats::convert(diff))
        }

        let mut characteristics_convert: HashMap<BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>> = HashMap::new();

        for (char_star, char_map) in og.characteristics.borrow() {
            let mut char_map_convert: HashMap<RustCStringWrapper, BeatStarSongDifficultyStats> = HashMap::new();

            for (str, diff_json) in char_map {


                char_map_convert.insert(RustCStringWrapper::new(str.clone().into()), BeatStarSongDifficultyStats::convert(&diff_json));
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
            characteristics: characteristics_convert
        }
    }
}

#[repr(C)]
pub enum SongDiffs {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}

#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Clone)]
pub struct BeatStarSongDifficultyStatsJson {
    pub diff: String,
    pub scores: i64,
    pub stars: f64,
    pub ranked: bool,
    pub njs: f32,
    pub bombs: u32,
    pub notes: u32,
    pub obstacles: u32,
    #[serde(rename = "Char")]
    pub char: String,
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
    pub extern fn get_diff_type(&self) -> BeatStarCharacteristics {
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
            char: RustCStringWrapper::new(og.char.clone().into())
        }
    }
}

#[repr(C)]
#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum BeatStarCharacteristics {
    Unknown, // TODO: Unkown? intentional mispelling? We're fixing it here until it breaks
    Standard,
    OneSaber,
    NoArrows,
    Lightshow,
    Degree90,
    Degree360,
    Lawless,
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
