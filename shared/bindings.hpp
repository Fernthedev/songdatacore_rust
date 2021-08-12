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
  float stars;
  bool ranked;
  float njs;
  float njs_offset;
  uint32_t bombs;
  uint32_t notes;
  uint32_t obstacles;
  RustCStringWrapper char_;
  const Vec<RustCStringWrapper> *requirements;

  bool operator==(const BeatStarSongDifficultyStats& other) const {
    return diff == other.diff &&
           stars == other.stars &&
           ranked == other.ranked &&
           njs == other.njs &&
           njs_offset == other.njs_offset &&
           bombs == other.bombs &&
           notes == other.notes &&
           obstacles == other.obstacles &&
           char_ == other.char_ &&
           requirements == other.requirements;
  }
  bool operator!=(const BeatStarSongDifficultyStats& other) const {
    return diff != other.diff ||
           stars != other.stars ||
           ranked != other.ranked ||
           njs != other.njs ||
           njs_offset != other.njs_offset ||
           bombs != other.bombs ||
           notes != other.notes ||
           obstacles != other.obstacles ||
           char_ != other.char_ ||
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

RustCStringWrapper RustCStringWrapper_c_new(char *c_str);

const BeatStarSong *BeatStarDataFile_map_SongsGet(const BeatStarDataFile *self_i,
                                                  const RustCStringWrapper *index);

const RustCStringWrapper *BeatStarDataFile_map_SongsGetKey(const BeatStarDataFile *self_i,
                                                           uintptr_t index);

uintptr_t BeatStarDataFile_map_SongsLen(const BeatStarDataFile *self_i);

float BeatStarSong_rating(const BeatStarSong *self_i);

const BeatStarSongDifficultyStats *BeatStarSong_DiffGet(const BeatStarSong *self_i,
                                                        uintptr_t index);

uintptr_t BeatStarSong_DiffLen(const BeatStarSong *self_i);

const HashMap<RustCStringWrapper, BeatStarSongDifficultyStats> *BeatStarSong_map_CharacteristicsGet(const BeatStarSong *self_i,
                                                                                                    const BeatStarCharacteristics *index);

const BeatStarCharacteristics *BeatStarSong_map_CharacteristicsKeyGet(const BeatStarSong *self_i,
                                                                      uintptr_t index);

uintptr_t BeatStarSong_map_CharacteristicsLen(const BeatStarSong *self_i);

///
/// Gets the size of the hashmap of the characteristics
///
uintptr_t BeatStarSong_CharacteristicDifficultyLen(const BeatStarSong *self_i,
                                                   const BeatStarCharacteristics *beat_char);

///
/// Gets the difficulty stats based on the characteristic key + difficulty key
///
const BeatStarSongDifficultyStats *BeatStarSong_CharacteristicStatsGet(const BeatStarSong *self_i,
                                                                       const BeatStarCharacteristics *beat_char,
                                                                       char *beat_key2);

///
/// Gets the key based on the index, converted to a string.
///
const char *BeatStarSong_CharacteristicsGetStrKey(const BeatStarSong *self_i,
                                                  const BeatStarCharacteristics *beat_char,
                                                  uintptr_t index);

const RustCStringWrapper *BeatStarSongDifficultyStats_requirementsGet(const BeatStarSongDifficultyStats *self_i,
                                                                      uintptr_t index);

uintptr_t BeatStarSongDifficultyStats_requirementsLen(const BeatStarSongDifficultyStats *self_i);

BeatStarCharacteristics BeatStarSongDifficultyStats_DiffCharacteristicsGet(const BeatStarSongDifficultyStats *self_i);

} // extern "C"

} // namespace song_data_core
