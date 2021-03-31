use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::result;
use std::str::FromStr;
use std::ffi::CString;
use std::borrow::Borrow;

#[repr(C)]
pub struct BeatStarDataFile {
    pub songs: HashMap<CString, BeatStarSong>,
}


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
    pub key: CString,
    pub diffs: Vec<BeatStarSongDifficultyStats>,
    pub uploaded: CString,
    pub hash: CString,
    pub characteristics: HashMap<BeatStarCharacteristics, HashMap<CString, BeatStarSongDifficultyStats>>,
}

impl BeatStarSong {
    #[no_mangle]
    pub extern fn rating(&self) -> f32 {
        let tot: f32 = (self.upvotes + self.downvotes) as f32;
        let tmp: f32 = (self.upvotes) as f32 / tot;

        tmp - (tmp - 0.5) * (2_i32.pow(-(tot + 1f32).log10() as u32) as f32)
    }


    pub fn convert(og: &BeatStarSongJson) -> BeatStarSong {
        let mut diff_convert: Vec<BeatStarSongDifficultyStats> = vec![];

        for diff in &og.diffs {
            diff_convert.push(BeatStarSongDifficultyStats::convert(diff))
        }

        let mut characteristics_convert: HashMap<BeatStarCharacteristics, HashMap<CString, BeatStarSongDifficultyStats>> = HashMap::new();

        for (char_star, char_map) in og.characteristics.borrow() {
            let mut char_map_convert: HashMap<CString, BeatStarSongDifficultyStats> = HashMap::new();

            for (str, diff_json) in char_map {


                char_map_convert.insert(CString::new(str.clone()).unwrap(), BeatStarSongDifficultyStats::convert(&diff_json));
            }

            characteristics_convert.insert(*char_star, char_map_convert);

        }

        BeatStarSong {
            bpm: og.bpm,
            played_count: og.played_count,
            upvotes: og.upvotes,
            downvotes: og.downvotes,
            key: CString::new(og.key.clone()).unwrap(),
            diffs: diff_convert,
            uploaded: CString::new(og.uploaded.clone()).unwrap(),
            hash: CString::new(og.hash.clone()).unwrap(),
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
    pub diff: CString,
    pub scores: i64,
    pub stars: f64,
    pub ranked: bool,
    pub njs: f32,
    pub bombs: u32,
    pub notes: u32,
    pub obstacles: u32,
    pub char: CString,
}

impl BeatStarSongDifficultyStats {
    #[no_mangle]
    pub extern fn get_diff_type(&self) -> BeatStarCharacteristics {
        return match BeatStarCharacteristics::from_str(self.char.clone().into_string().unwrap().as_str()) {
            Ok(e) => e,
            Err(_) => BeatStarCharacteristics::Unknown,
        };
    }

    pub fn convert(og: &BeatStarSongDifficultyStatsJson) -> BeatStarSongDifficultyStats {
        BeatStarSongDifficultyStats {
            diff: CString::new(og.diff.clone()).unwrap(),
            scores: og.scores,
            stars: og.stars,
            ranked: og.ranked,
            njs: og.njs,
            bombs: og.bombs,
            notes: og.notes,
            obstacles: og.obstacles,
            char: CString::new(og.char.clone()).unwrap()
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
