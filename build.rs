use {
    anyhow::Result,
    std::{env, fs, path::PathBuf, process::Command},
    wasmtime::{Config, Engine},
    wit_component::ComponentEncoder,
};

fn main() -> Result<()> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    assert!(Command::new("cargo")
        .current_dir("guest")
        .env("CARGO_TARGET_DIR", &out_dir)
        .args(["build", "--release", "--target", "wasm32-wasi"])
        .status()?
        .success());

    let adapter = &reqwest::blocking::get(
        "https://github.com/bytecodealliance/wasmtime/releases/download/\
         v13.0.0/wasi_snapshot_preview1.command.wasm",
    )?
    .error_for_status()?
    .bytes()?;

    let component = &ComponentEncoder::default()
        .module(&fs::read(out_dir.join("wasm32-wasi/release/guest.wasm"))?)?
        .validate(true)
        .adapter("wasi_snapshot_preview1", adapter)?
        .encode()?;

    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = &Engine::new(&config)?;

    let cwasm = engine.precompile_component(component)?;

    fs::write(out_dir.join("wasm32-wasi/release/guest.cwasm"), cwasm)?;

    println!("cargo:rerun-if-changed=guest");

    Ok(())
}
