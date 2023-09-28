#! /bin/bash

set -ex

cd src
lfortran $HOME/repos/lfortran/lfortran/examples/expr2.f90 --backend=wasm -o guest.wasm
wasmtime compile guest.wasm -o guest.cwasm
cd ..

cargo run
