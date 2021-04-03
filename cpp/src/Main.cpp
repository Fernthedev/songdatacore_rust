#include "Main.hpp"
#include "../shared/bindings.hpp"
#include "Stopwatch.hpp"
#include <iostream>

using namespace std;

int main() {
    auto stopwatch = stopwatch::Stopwatch();
    stopwatch.start();

    cout << "Fetching from db\n";

    song_data_core::beatstar_retrieve_database_extern();

    cout << "Time taken to fetch from database: " << stopwatch.elapsed<stopwatch::milliseconds>() << "ms\n";

    auto song = song_data_core::beatstar_get_song_extern("4B2DA842B687EC4CFBC948C583C21C79D4120DE0");

    auto diff = song_data_core::extern_vec_diffs_get(song, 0);

    cout << "Got the stars: " << diff->diff.string_data << " " << diff->stars;

    return 0;
}