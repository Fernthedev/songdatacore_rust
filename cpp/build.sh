echo This assumes that you have built the rust project with "cargo build"



clang++ -L../target/release/ -I../shared -o testb.out ./src/Main.cpp -lsong_data_core_rust


export LD_LIBRARY_PATH=../target/release
echo Finished compile, now running
./testb.out