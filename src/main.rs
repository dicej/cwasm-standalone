use {
    anyhow::{anyhow, Result},
    std::io,
    clap::Parser,
    wasmtime::{Config, Engine, Linker, Module, Store},
    wasmtime_wasi::{Dir, I32Exit, WasiCtx, WasiCtxBuilder},
    mylib::run_internal,
};

extern "C" {
    fn run1234(len: usize, guest: *const u8) -> i32;
}

fn main() -> Result<(), io::Error> {
    let guest: Vec<u8> = include_bytes!("guest.cwasm").to_vec();

    let result = run_internal(&guest);

    Ok(())
    /*
    let result: i32 = unsafe {
        run1234(guest.len(), guest.as_ptr())
    };

    match result {
        0 => Ok(()),
        _ => Err(io::ErrorKind::Other.into()),
    }
    */
}
