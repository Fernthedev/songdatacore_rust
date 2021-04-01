use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
