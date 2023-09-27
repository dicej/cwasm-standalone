use {
    anyhow::Result,
    std::{env, fs, path::PathBuf, process::Command},
    wasmtime::{Config, Engine},
};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    assert!(Command::new("cargo")
        .current_dir("guest")
        .env("CARGO_TARGET_DIR", &out_dir)
        .args(["build", "--release", "--target", "wasm32-wasi"])
        .status()?
        .success());

    let engine = &Engine::new(&Config::new())?;

    let cwasm =
        engine.precompile_module(&fs::read(out_dir.join("wasm32-wasi/release/guest.wasm"))?)?;

    fs::write(out_dir.join("wasm32-wasi/release/guest.cwasm"), cwasm)?;

    println!("cargo:rerun-if-changed=guest");

    Ok(())
}
