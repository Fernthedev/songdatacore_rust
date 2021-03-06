use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::result;
use std::str::FromStr;

pub type UnixTime = libc::time_t;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", rename = "with typo")]
pub struct BeatStarSongJson {
    #[serde(rename = "Bpm")]
    pub bpm: f32,

    #[serde(rename = "Upvotes")]
    pub upvotes: u32,

    #[serde(rename = "Downvotes")]
    pub downvotes: u32,

    #[serde(rename = "Duration")]
    pub duration_secs: u32,

    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "SongName")]
    pub song_name: String,

    #[serde(rename = "SongSubName")]
    pub song_sub_name: String,

    #[serde(rename = "SongAuthorName")]
    pub song_author_name: String,

    #[serde(rename = "LevelAuthorName")]
    pub level_author_name: String,

    #[serde(rename = "Diffs")]
    pub diffs: Vec<BeatStarSongDifficultyStatsJson>,

    #[serde(rename = "Uploaded")]
    pub uploaded: String,

    #[serde(skip_deserializing)]
    pub uploaded_unix_time: UnixTime,

    #[serde(rename = "Hash")]
    pub hash: String,

    #[serde(skip_deserializing)]
    pub characteristics:
        HashMap<BeatStarCharacteristics, HashMap<String, BeatStarSongDifficultyStatsJson>>,

    #[serde(skip_deserializing)]
    pub heat: f32,

    #[serde(skip_deserializing)]
    pub rating: f32,
}

#[repr(C)]
pub enum SongDiffs {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BeatStarSongDifficultyStatsJson {
    pub diff: String,
    #[serde(default)]
    pub stars: f32,
    #[serde(default)]
    pub ranked: bool,
    pub njs: f32,

    #[serde(rename = "NjsOffset")]
    pub njs_offset: f32,

    pub requirements: Vec<String>,

    pub bombs: u32,
    pub notes: u32,
    pub obstacles: u32,
    #[serde(rename = "Char")]
    pub char: String,

    #[serde(skip_deserializing)]
    pub approximate_pp_value: f32,

    #[serde(rename = "RankedUpdateTime")]
    pub ranked_update_time: String,

    #[serde(skip_deserializing)]
    pub ranked_update_time_unix_epoch: UnixTime,
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

impl std::fmt::Display for BeatStarCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
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
            "90degree" => Ok(BeatStarCharacteristics::Degree90),
            "degree360" => Ok(BeatStarCharacteristics::Degree360),
            "360degree" => Ok(BeatStarCharacteristics::Degree360),
            "lawless" => Ok(BeatStarCharacteristics::Lawless),
            _ => Err(()),
        }
    }
}
