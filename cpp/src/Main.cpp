#include "Main.hpp"
#include "../shared/bindings.hpp"
#include "Stopwatch.hpp"
#include <iostream>

using namespace std;

int main() {
    auto stopwatch = stopwatch::Stopwatch();
    stopwatch.start();

    cout << "Fetching from db\n";

    song_data_core::Beatstar_RetrieveDatabase();

    cout << "Time taken to fetch from database: " << stopwatch.elapsed<stopwatch::milliseconds>() << "ms\n";

    auto song = song_data_core::Beatstar_GetSong("4B2DA842B687EC4CFBC948C583C21C79D4120DE0");

    auto diff = song_data_core::BeatStarSong_DiffGet(song, 0);

    cout << "Got the stars: " << diff->diff.string_data << " " << diff->stars << std::endl;

    auto song2 = song_data_core::Beatstar_GetSong("B9BED84A127130BF80AFF18DB677EDD215CE0AB5");

    if (song2) {
        for (int i = 0; i < 3; i++) {
            auto charMapLen = song_data_core::BeatStarSong_map_CharacteristicsLen(song2);

            cout << "Got the chars for song: " << song2->song_name.string_data << " with size: " << charMapLen << std::endl;

            for (int j = 0; j < charMapLen; j++)
            {
                const song_data_core::BeatStarCharacteristics* charMapKey = song_data_core::BeatStarSong_map_CharacteristicsKeyGet(song2, j);

                auto diffMapLen = song_data_core::BeatStarSong_map_Characteristics_DifficultyStatsLen(song2, charMapKey);

                cout << "   Got the diffs for char: " << charMapKey << " with size: " << diffMapLen << std::endl;

                for (int k = 0; k < diffMapLen; k++) {
                    auto diffKey = song_data_core::BeatStarSong_map_Characteristics_DifficultyStatsGetStrKey(song2, charMapKey, k);
                    auto diffData = song_data_core::BeatStarSong_map_Characteristics_DifficultyStatsGet(song2, charMapKey, diffKey);

                    cout << "       Got the diff: " << diffData->diff.string_data << " with pp: " << diffData->approximate_pp_value << std::endl;
                }
            }
        }
    }

    return 0;
}