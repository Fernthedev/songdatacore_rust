use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::result;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", rename = "with typo")]
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
pub enum SongDiffs {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
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

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "PascalCase")]
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
