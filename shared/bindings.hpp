#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace song_data_core {

enum class BeatStarCharacteristics {
  Unknown,
  Standard,
  OneSaber,
  NoArrows,
  Lightshow,
  Degree90,
  Degree360,
  Lawless,
};

template<typename K = void, typename V = void, typename Hasher = void>
struct HashMap;

template<typename T = void>
struct Vec;

struct RustCStringWrapper {
  char *string_data;

  bool operator==(const RustCStringWrapper& other) const {
    return string_data == other.string_data;
  }
  bool operator!=(const RustCStringWrapper& other) const {
    return string_data != other.string_data;
  }
};

struct BeatStarSongDifficultyStats {
  RustCStringWrapper diff;
  float approximate_pp_value;
  float stars;
  bool ranked;
  float njs;
  float njs_offset;
  uint32_t bombs;
  uint32_t notes;
  uint32_t obstacles;
  RustCStringWrapper char_;
  BeatStarCharacteristics diff_characteristics;
  const Vec<RustCStringWrapper> *requirements;

  bool operator==(const BeatStarSongDifficultyStats& other) const {
    return diff == other.diff &&
           approximate_pp_value == other.approximate_pp_value &&
           stars == other.stars &&
           ranked == other.ranked &&
           njs == other.njs &&
           njs_offset == other.njs_offset &&
           bombs == other.bombs &&
           notes == other.notes &&
           obstacles == other.obstacles &&
           char_ == other.char_ &&
           diff_characteristics == other.diff_characteristics &&
           requirements == other.requirements;
  }
  bool operator!=(const BeatStarSongDifficultyStats& other) const {
    return diff != other.diff ||
           approximate_pp_value != other.approximate_pp_value ||
           stars != other.stars ||
           ranked != other.ranked ||
           njs != other.njs ||
           njs_offset != other.njs_offset ||
           bombs != other.bombs ||
           notes != other.notes ||
           obstacles != other.obstacles ||
           char_ != other.char_ ||
           diff_characteristics != other.diff_characteristics ||
           requirements != other.requirements;
  }
};

struct BeatStarSong {
  float bpm;
  uint32_t upvotes;
  uint32_t downvotes;
  uint32_t downloads;
  uint32_t duration_secs;
  RustCStringWrapper key;
  RustCStringWrapper song_name;
  RustCStringWrapper song_sub_name;
  RustCStringWrapper song_author_name;
  RustCStringWrapper level_author_name;
  const Vec<BeatStarSongDifficultyStats> *diffs;
  RustCStringWrapper uploaded;
  RustCStringWrapper hash;
  const HashMap<BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>> *characteristics;

  bool operator==(const BeatStarSong& other) const {
    return bpm == other.bpm &&
           upvotes == other.upvotes &&
           downvotes == other.downvotes &&
           downloads == other.downloads &&
           duration_secs == other.duration_secs &&
           key == other.key &&
           song_name == other.song_name &&
           song_sub_name == other.song_sub_name &&
           song_author_name == other.song_author_name &&
           level_author_name == other.level_author_name &&
           diffs == other.diffs &&
           uploaded == other.uploaded &&
           hash == other.hash &&
           characteristics == other.characteristics;
  }
  bool operator!=(const BeatStarSong& other) const {
    return bpm != other.bpm ||
           upvotes != other.upvotes ||
           downvotes != other.downvotes ||
           downloads != other.downloads ||
           duration_secs != other.duration_secs ||
           key != other.key ||
           song_name != other.song_name ||
           song_sub_name != other.song_sub_name ||
           song_author_name != other.song_author_name ||
           level_author_name != other.level_author_name ||
           diffs != other.diffs ||
           uploaded != other.uploaded ||
           hash != other.hash ||
           characteristics != other.characteristics;
  }
};

struct BeatStarDataFile {
  const HashMap<RustCStringWrapper, BeatStarSong> *songs;

  bool operator==(const BeatStarDataFile& other) const {
    return songs == other.songs;
  }
  bool operator!=(const BeatStarDataFile& other) const {
    return songs != other.songs;
  }
};

extern "C" {

///
/// Get the song list and clone it
///
const BeatStarDataFile *Beatstar_RetrieveDatabase();

///
/// Get the song based on hash
///
///
const BeatStarSong *Beatstar_GetSong(const char *hash);

///
/// Get the value in the hashmap from the key
///
const BeatStarSong *BeatStarDataFile_map_SongsGet(const BeatStarDataFile *self_i,
                                                  const RustCStringWrapper *index);

///
/// Get the key in the hashmap from the index in it's set.
///
const RustCStringWrapper *BeatStarDataFile_map_SongsGetKey(const BeatStarDataFile *self_i,
                                                           uintptr_t index);

///
/// Get the length of the hashmap
///
uintptr_t BeatStarDataFile_map_SongsLen(const BeatStarDataFile *self_i);

///
/// An algorithm for getting a song's rating.
///
float BeatStarSong_rating(const BeatStarSong *self_i);

/// Gets the item in the vector from index
const BeatStarSongDifficultyStats *BeatStarSong_DiffGet(const BeatStarSong *self_i,
                                                        uintptr_t index);

/// Gets the length of the vector
uintptr_t BeatStarSong_DiffLen(const BeatStarSong *self_i);

///
/// Get the value in the hashmap from the key
///
const HashMap<RustCStringWrapper, BeatStarSongDifficultyStats> *useless_BeatStarSong_map_CharacteristicsGet(const BeatStarSong *self_i,
                                                                                                            const BeatStarCharacteristics *index);

///
/// Get the key in the hashmap from the index in it's set.
///
const BeatStarCharacteristics *BeatStarSong_map_CharacteristicsKeyGet(const BeatStarSong *self_i,
                                                                      uintptr_t index);

///
/// Get the length of the hashmap
///
uintptr_t BeatStarSong_map_CharacteristicsLen(const BeatStarSong *self_i);

///
/// Gets the size of the hashmap of the characteristics
///
uintptr_t BeatStarSong_map_Characteristics_DifficultyStatsLen(const BeatStarSong *self_i,
                                                              const BeatStarCharacteristics *beat_char);

///
/// Gets the difficulty stats based on the characteristic key + difficulty key
///
const BeatStarSongDifficultyStats *BeatStarSong_map_Characteristics_DifficultyStatsGet(const BeatStarSong *self_i,
                                                                                       const BeatStarCharacteristics *beat_char,
                                                                                       const char *beat_key2);

///
/// Gets the key based on the index, converted to a string.
///
const char *BeatStarSong_map_Characteristics_DifficultyStatsGetStrKey(const BeatStarSong *self_i,
                                                                      const BeatStarCharacteristics *beat_char,
                                                                      uintptr_t index);

/// Gets the item in the vector from index
const RustCStringWrapper *BeatStarSongDifficultyStats_requirementsGet(const BeatStarSongDifficultyStats *self_i,
                                                                      uintptr_t index);

/// Gets the length of the vector
uintptr_t BeatStarSongDifficultyStats_requirementsLen(const BeatStarSongDifficultyStats *self_i);

} // extern "C"

} // namespace song_data_core
