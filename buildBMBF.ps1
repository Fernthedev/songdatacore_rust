# Builds a .zip file for loading with BMBF
& $PSScriptRoot/build.ps1

if ($?) {
    Compress-Archive -Path "./libs/arm64-v8a/libsong_data_core_rust.so", "./bmbfmod.json" -DestinationPath "./SongDataCore_v0.1.0.zip" -Update
}
