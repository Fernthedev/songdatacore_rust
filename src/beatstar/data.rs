use serde::{Deserialize, Serialize};
use std::result;
use std::str::FromStr;

pub type UnixTime = libc::time_t;

#[repr(C)]
pub enum SongDiffs {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus,
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

impl Default for BeatStarCharacteristics {
    fn default() -> Self {
        BeatStarCharacteristics::Unknown
    }
}

impl std::fmt::Display for BeatStarCharacteristics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
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
