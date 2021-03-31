use crate::beatstar::data::BeatStarDataFile;
use once_cell::sync::OnceCell;

mod data;
mod database;

static BEAT_STAR_FILE: OnceCell<BeatStarDataFile> = OnceCell::new();
