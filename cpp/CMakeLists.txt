cmake_minimum_required(VERSION 3.17)

project(SongDataCoreRust_CppTest)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
    message("Building release")
endif()

set(CMAKE_CXX_FLAGS_RELEASE "-O3")

include_directories("../target/release")
include_directories("../shared/")
include_directories("./src")

add_executable(SongDataCoreRust_CppTest src/main.cpp)

get_filename_component(RUST_DLL_LIBRARY_FILE "../target/release/song_data_core_rust.dll" ABSOLUTE) # Convert relative path to absolute so it can link
get_filename_component(RUST_LIBRARY_FILE "../target/release/song_data_core_rust.dll.lib" ABSOLUTE) # Convert relative path to absolute so it can link


message("Linking to ${RUST_LIBRARY_FILE} assuming you have built the rust project")

target_link_libraries(SongDataCoreRust_CppTest
PRIVATE ${RUST_LIBRARY_FILE}
) # Requires building using  "cargo build --release"

if (EXISTS ${RUST_LIBRARY_FILE})
    file(COPY ${RUST_DLL_LIBRARY_FILE}
            DESTINATION ${CMAKE_BINARY_DIR})
endif()