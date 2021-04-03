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
const BeatStarDataFile *beatstar_retrieve_database_extern();

///
/// Get the song based on hash
///
///
const BeatStarSong *beatstar_get_song_extern(const char *hash);

RustCStringWrapper rust_cstring_wrapper_c_new(char *c_str);

const BeatStarSong *extern_map_songs_get(const BeatStarDataFile *self,
                                         const RustCStringWrapper *index);

const RustCStringWrapper *extern_map_songs_get_key(const BeatStarDataFile *self, uintptr_t index);

uintptr_t extern_map_songs_len(const BeatStarDataFile *self);

float rating(const BeatStarSong *self);

const BeatStarSongDifficultyStats *extern_vec_diffs_get(const BeatStarSong *self, uintptr_t index);

uintptr_t extern_vec_diffs_len(const BeatStarSong *self);

const HashMap<RustCStringWrapper, BeatStarSongDifficultyStats> *extern_map_characteristics_get(const BeatStarSong *self,
                                                                                               const BeatStarCharacteristics *index);

const BeatStarCharacteristics *extern_map_characteristics_key_get(const BeatStarSong *self,
                                                                  uintptr_t index);

uintptr_t extern_map_characteristics_len(const BeatStarSong *self);

uintptr_t get_characteristics_len(const BeatStarSong *self,
                                  const BeatStarCharacteristics *beat_char);

const char *get_characteristics_str(const BeatStarSong *self,
                                    const BeatStarCharacteristics *beat_char,
                                    uintptr_t index);

const BeatStarSongDifficultyStats *get_characteristic_stats(const BeatStarSong *self,
                                                            const BeatStarCharacteristics *beat_char,
                                                            char *beat_key2);

BeatStarCharacteristics get_diff_type(const BeatStarSongDifficultyStats *self);

} // extern "C"

} // namespace song_data_core
