#! /bin/bash

set -ex

cd src
lfortran $HOME/repos/lfortran/lfortran/examples/expr2.f90 --backend=wasm -o guest.wasm
wasmtime compile guest.wasm -o guest.cwasm
cd ..

cd mylib
cargo build --release --offline
cd ..

strip mylib/target/release/libmylib.a

cargo run --release --offline

bin2c guest < src/guest.cwasm > guest.c
clang -c guest.c -o guest.o
clang -c driver.c -o driver.o
clang -o driver driver.o guest.o mylib/target/release/libmylib.a
ls -h driver
strip driver
ls -h driver
