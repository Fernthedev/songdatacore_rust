use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::result;
use std::str::FromStr;

#[repr(C)]
#[derive(Clone)]
pub struct BeatStarDataFile {
    pub songs: HashMap<String, BeatStarSong>,
}

#[repr(C)]
#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Clone)]
pub struct BeatStarSong {
    pub bpm: f32,
    pub played_count: u32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub key: String,
    pub diffs: Vec<BeatStarSongDifficultyStats>,
    pub uploaded: String,
    pub hash: String,

    pub characteristics:
        HashMap<BeatStarCharacteristics, HashMap<String, BeatStarSongDifficultyStats>>,
}

impl BeatStarSong {
    pub fn rating(&self) -> f32 {
        let tot: f32 = (self.upvotes + self.downvotes) as f32;
        let tmp: f32 = (self.upvotes) as f32 / tot;

        return tmp - (tmp - 0.5) * (2_i32.pow(-(tot + 1f32).log10() as u32) as f32);
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

#[repr(C)]
#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Clone)]
pub struct BeatStarSongDifficultyStats {
    pub diff: String,
    pub scores: i64,
    pub stars: f64,
    pub ranked: bool,
    pub njs: f32,
    pub bombs: u32,
    pub notes: u32,
    pub obstacles: u32,
    pub char: String,
}

impl BeatStarSongDifficultyStats {
    #[no_mangle]
    pub extern fn get_diff_type(&self) -> BeatStarCharacteristics {
        return match BeatStarCharacteristics::from_str(self.char.as_str()) {
            Ok(e) => e,
            Err(_) => BeatStarCharacteristics::Unknown,
        };
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
            "Unknown" => Ok(BeatStarCharacteristics::Unknown),
            "Standard" => Ok(BeatStarCharacteristics::Standard),
            "OneSaber" => Ok(BeatStarCharacteristics::OneSaber),
            "NoArrows" => Ok(BeatStarCharacteristics::NoArrows),
            "Lightshow" => Ok(BeatStarCharacteristics::Lightshow),
            "Degree90" => Ok(BeatStarCharacteristics::Degree90),
            "Degree360" => Ok(BeatStarCharacteristics::Degree360),
            "Lawless" => Ok(BeatStarCharacteristics::Lawless),
            _ => Err(()),
        }
    }
}
