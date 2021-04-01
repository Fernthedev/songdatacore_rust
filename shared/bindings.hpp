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

template<typename T>
struct VecStruct {
  const T *array;
  uintptr_t len;

  bool operator==(const VecStruct& other) const {
    return array == other.array &&
           len == other.len;
  }
  bool operator!=(const VecStruct& other) const {
    return array != other.array ||
           len != other.len;
  }
};

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
  Vec<BeatStarSongDifficultyStats> diffs;
  RustCStringWrapper uploaded;
  RustCStringWrapper hash;
  HashMap<BeatStarCharacteristics, HashMap<RustCStringWrapper, BeatStarSongDifficultyStats>> characteristics;

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
  HashMap<RustCStringWrapper, BeatStarSong> songs;

  bool operator==(const BeatStarDataFile& other) const {
    return songs == other.songs;
  }
  bool operator!=(const BeatStarDataFile& other) const {
    return songs != other.songs;
  }
};

extern "C" {

const V *get_from_hashmap(const HashMap<K, V, S> *map, const K *key);

VecStruct<T> get_from_vec(const Vec<T> *vec);

float rating(const BeatStarSong *self);

BeatStarCharacteristics get_diff_type(const BeatStarSongDifficultyStats *self);

///
/// Get the song list and clone it
///
const BeatStarDataFile *beatstar_retrieve_database_extern();

///
/// Get the song based on hash
///
///
const BeatStarSong *beatstar_get_song_extern(const char *hash);

} // extern "C"

} // namespace song_data_core
