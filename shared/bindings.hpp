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
  int64_t scores;
  double stars;
  bool ranked;
  float njs;
  uint32_t bombs;
  uint32_t notes;
  uint32_t obstacles;
  RustCStringWrapper char_;

  bool operator==(const BeatStarSongDifficultyStats& other) const {
    return diff == other.diff &&
           scores == other.scores &&
           stars == other.stars &&
           ranked == other.ranked &&
           njs == other.njs &&
           bombs == other.bombs &&
           notes == other.notes &&
           obstacles == other.obstacles &&
           char_ == other.char_;
  }
  bool operator!=(const BeatStarSongDifficultyStats& other) const {
    return diff != other.diff ||
           scores != other.scores ||
           stars != other.stars ||
           ranked != other.ranked ||
           njs != other.njs ||
           bombs != other.bombs ||
           notes != other.notes ||
           obstacles != other.obstacles ||
           char_ != other.char_;
  }
};

struct BeatStarSong {
  float bpm;
  uint32_t played_count;
  uint32_t upvotes;
  uint32_t downvotes;
  RustCStringWrapper key;
  const Vec<BeatStarSongDifficultyStats> *diffs;
  RustCStringWrapper uploaded;
  RustCStringWrapper hash;
  const HashMap<BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>> *characteristics;

  bool operator==(const BeatStarSong& other) const {
    return bpm == other.bpm &&
           played_count == other.played_count &&
           upvotes == other.upvotes &&
           downvotes == other.downvotes &&
           key == other.key &&
           diffs == other.diffs &&
           uploaded == other.uploaded &&
           hash == other.hash &&
           characteristics == other.characteristics;
  }
  bool operator!=(const BeatStarSong& other) const {
    return bpm != other.bpm ||
           played_count != other.played_count ||
           upvotes != other.upvotes ||
           downvotes != other.downvotes ||
           key != other.key ||
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

BeatStarCharacteristics BeatStarSongDifficultyStats_DiffCharacteristicsGet(const BeatStarSongDifficultyStats *self_i);

} // extern "C"

} // namespace song_data_core
