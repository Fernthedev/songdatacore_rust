#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

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

struct BeatStarSongDifficultyStats {
  CString diff;
  int64_t scores;
  double stars;
  bool ranked;
  float njs;
  uint32_t bombs;
  uint32_t notes;
  uint32_t obstacles;
  CString char_;
};

struct BeatStarSong {
  float bpm;
  uint32_t played_count;
  uint32_t upvotes;
  uint32_t downvotes;
  CString key;
  Vec<BeatStarSongDifficultyStats> diffs;
  CString uploaded;
  CString hash;
  HashMap<BeatStarCharacteristics, HashMap<CString, BeatStarSongDifficultyStats>> characteristics;
};

struct BeatStarDataFile {
  HashMap<CString, BeatStarSong> songs;
};

extern "C" {

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
