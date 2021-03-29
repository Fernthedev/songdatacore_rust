use crate::beatstar::data::BeatStarDataFile;

mod data;
mod database;

static mut BEAT_STAR_FILE: Option<BeatStarDataFile> = None;
