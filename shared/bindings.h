#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

static const uint64_t MAX_DELAY = ((60 * 60) * 12);

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

template<typename T = void, typename E = void>
struct Result;

struct String;

template<typename T = void>
struct Vec;

struct BeatStarSongDifficultyStats {
  String diff;
  int64_t scores;
  double stars;
  bool ranked;
  float njs;
  uint32_t bombs;
  uint32_t notes;
  uint32_t obstacles;
  String char_;
};

struct BeatStarSong {
  float bpm;
  uint32_t played_count;
  uint32_t upvotes;
  uint32_t downvotes;
  String key;
  Vec<BeatStarSongDifficultyStats> diffs;
  String uploaded;
  String hash;
  HashMap<BeatStarCharacteristics, HashMap<String, BeatStarSongDifficultyStats>> characteristics;
};

struct BeatStarDataFile {
  HashMap<String, BeatStarSong> songs;
};

extern "C" {

BeatStarCharacteristics get_diff_type(const BeatStarSongDifficultyStats *self);

Result<BeatStarDataFile, Response> beatstar_fetch_database();

} // extern "C"
