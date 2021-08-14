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
use std::ptr;
use std::str::FromStr;

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

    fn from_copy(c_str: *mut c_char) -> Self {
        unsafe {
            let oldString = CString::from_raw(c_str);
            let newString = oldString.clone();
            oldString.into_raw();
            return RustCStringWrapper {
                string_data: newString.into_raw(),
            };
        }
    }
}

impl From<*mut c_char> for RustCStringWrapper {
    fn from(c_str: *mut c_char) -> Self {
        unsafe {
            return RustCStringWrapper {
                string_data: CString::from_raw(c_str).into_raw(),
            };
        }
    }
}

#[repr(C)]
pub struct BeatStarDataFile {
    pub songs: *const HashMap<RustCStringWrapper, BeatStarSong>,
}

unsafe impl Send for BeatStarDataFile {}
unsafe impl Sync for BeatStarDataFile {}

map_extern!(
    BeatStarDataFile,
    songs,
    RustCStringWrapper,
    BeatStarSong,
    BeatStarDataFile_map_SongsGet,
    BeatStarDataFile_map_SongsLen,
    BeatStarDataFile_map_SongsGetKey
);

#[repr(C)]
pub struct BeatStarSong {
    pub bpm: f32,
    pub upvotes: u32,
    pub downvotes: u32,
    pub downloads: u32,
    pub duration_secs: u32,
    pub key: RustCStringWrapper,
    pub song_name: RustCStringWrapper,
    pub song_sub_name: RustCStringWrapper,
    pub song_author_name: RustCStringWrapper,
    pub level_author_name: RustCStringWrapper,
    pub diffs: *const Vec<BeatStarSongDifficultyStats>,
    pub uploaded: RustCStringWrapper,
    pub hash: RustCStringWrapper,
    pub characteristics: *const HashMap<
        BeatStarCharacteristics,
        HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>,
    >,
}

impl BeatStarSong {
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
                    BeatStarSongDifficultyStats::convert(diff_json),
                );
            }

            characteristics_convert.insert(*char_star, char_map_convert);
        }

        BeatStarSong {
            bpm: og.bpm,
            upvotes: og.upvotes,
            downvotes: og.downvotes,
            downloads: og.downloads,
            song_name: RustCStringWrapper::new(og.song_name.clone().into()),
            song_author_name: RustCStringWrapper::new(og.song_author_name.clone().into()),
            song_sub_name: RustCStringWrapper::new(og.song_sub_name.clone().into()),
            key: RustCStringWrapper::new(og.key.clone().into()),
            diffs: Box::into_raw(Box::new(diff_convert)),
            uploaded: RustCStringWrapper::new(og.uploaded.clone().into()),
            hash: RustCStringWrapper::new(og.hash.clone().into()),
            characteristics: Box::into_raw(Box::new(characteristics_convert)),
            duration_secs: og.duration_secs,
            level_author_name: RustCStringWrapper::new(og.level_author_name.clone().into()),
        }
    }
}

///
/// An algorithm for getting a song's rating.
///
#[no_mangle]
pub extern "C" fn BeatStarSong_rating(self_i: &BeatStarSong) -> f32 {
    let tot: f32 = (self_i.upvotes + self_i.downvotes) as f32;
    let tmp: f32 = (self_i.upvotes) as f32 / tot;

    tmp - (tmp - 0.5) * (2_i32.pow(-(tot + 1f32).log10() as u32) as f32)
}

vec_extern!(
    BeatStarSong,
    diffs,
    BeatStarSongDifficultyStats,
    BeatStarSong_DiffGet,
    BeatStarSong_DiffLen
);
map_extern!(BeatStarSong, characteristics, BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>,
    BeatStarSong_map_CharacteristicsGet,
    BeatStarSong_map_CharacteristicsLen,
    BeatStarSong_map_CharacteristicsKeyGet);

///
/// Gets the size of the hashmap of the characteristics
///
#[no_mangle]
pub extern "C" fn BeatStarSong_CharacteristicDifficultyLen(
    self_i: &BeatStarSong,
    beat_char: &BeatStarCharacteristics,
) -> usize {
    unsafe {
        return match (*self_i.characteristics).get(beat_char) {
            None => 0,
            Some(e) => e.len(),
        };
    }
}

///
/// Gets the difficulty stats based on the characteristic key + difficulty key
///
#[no_mangle]
pub extern "C" fn BeatStarSong_CharacteristicStatsGet(
    self_i: &BeatStarSong,
    beat_char: &BeatStarCharacteristics,
    beat_key2: *mut c_char,
) -> *const BeatStarSongDifficultyStats {
    unsafe {
        // let len = CStr::strlen(beat_key2) + 1; // Including the NUL byte

        // let copy_str = vec![0, len].as_mut_ptr();
        // beat_key2.copy_to(copy_str, len);

        return match (*self_i.characteristics).get(beat_char) {
            Some(map) => match map.get(&RustCStringWrapper::from_copy(beat_key2)) {
                None => ptr::null(),
                Some(e) => e,
            },
            None => ptr::null(),
        };
    }
}

///
/// Gets the key based on the index, converted to a string.
///
#[no_mangle]
pub extern "C" fn BeatStarSong_CharacteristicsGetStrKey(
    self_i: &BeatStarSong,
    beat_char: &BeatStarCharacteristics,
    index: usize,
) -> *const c_char {
    unsafe {
        return match (*self_i.characteristics).get(beat_char) {
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
}

#[derive(Clone)]
#[repr(C)]
pub struct BeatStarSongDifficultyStats {
    pub diff: RustCStringWrapper,
    pub approximate_pp_value: f32,
    pub stars: f32,
    pub ranked: bool,
    pub njs: f32,
    pub njs_offset: f32,
    pub bombs: u32,
    pub notes: u32,
    pub obstacles: u32,
    pub char: RustCStringWrapper,
    pub requirements: *const Vec<RustCStringWrapper>,
}

vec_extern!(
    BeatStarSongDifficultyStats,
    requirements,
    RustCStringWrapper,
    BeatStarSongDifficultyStats_requirementsGet,
    BeatStarSongDifficultyStats_requirementsLen
);

/// Gets the BeatStarCharacteristics enum value from the BeatStarSongDifficultyStats
#[no_mangle]
pub extern "C" fn BeatStarSongDifficultyStats_DiffCharacteristicsGet(
    self_i: &BeatStarSongDifficultyStats,
) -> BeatStarCharacteristics {
    return match BeatStarCharacteristics::from_str(self_i.char.to_string().as_str()) {
        Ok(e) => e,
        Err(_) => BeatStarCharacteristics::Unknown,
    };
}

impl BeatStarSongDifficultyStats {
    pub fn convert(og: &BeatStarSongDifficultyStatsJson) -> BeatStarSongDifficultyStats {
        let mut requirements: Vec<RustCStringWrapper> = vec![];

        for requirement in &og.requirements {
            requirements.push(RustCStringWrapper::new(requirement.clone().into()))
        }

        BeatStarSongDifficultyStats {
            diff: RustCStringWrapper::new(og.diff.clone().into()),
            stars: og.stars,
            ranked: og.ranked,
            njs: og.njs,
            bombs: og.bombs,
            notes: og.notes,
            obstacles: og.obstacles,
            char: RustCStringWrapper::new(og.char.clone().into()),
            njs_offset: og.njs_offset,
            requirements: Box::into_raw(Box::new(requirements)),
            approximate_pp_value: og.approximate_pp_value,
        }
    }
}
