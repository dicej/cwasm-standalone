use {
    anyhow::{anyhow, Result},
    clap::Parser,
    wasmtime::{Config, Engine, Linker, Module, Store},
    wasmtime_wasi::{Dir, I32Exit, WasiCtx, WasiCtxBuilder},
};

fn parse_mapdir(s: &str) -> Result<(String, String)> {
    if let Some((guest_dir, host_dir)) = s.split_once("::") {
        Ok((guest_dir.into(), host_dir.into()))
    } else {
        Err(anyhow!(
            "expected string of form GUEST_DIR::HOST_DIR; got {s}"
        ))
    }
}

fn parse_env(s: &str) -> Result<(String, String)> {
    if let Some((name, value)) = s.split_once('=') {
        Ok((name.into(), value.into()))
    } else {
        Err(anyhow!("expected string of form NAME=VALUE; got {s}"))
    }
}

#[derive(Parser)]
pub struct Options {
    #[clap(long, value_name = "GUEST_DIR::HOST_DIR", value_parser = parse_mapdir)]
    mapdir: Vec<(String, String)>,

    #[clap(long, value_name = "NAME=VALUE", value_parser = parse_env)]
    env: Vec<(String, String)>,
}

struct Ctx {
    wasi: WasiCtx,
}

fn main() -> Result<()> {
    let options = Options::parse();

    let engine = &Engine::new(&Config::new())?;

    let mut linker = Linker::<Ctx>::new(engine);
    wasmtime_wasi::add_to_linker(&mut linker, |ctx| &mut ctx.wasi)?;

    let mut wasi = WasiCtxBuilder::new();
    wasi.inherit_stdio();

    for (guest_dir, host_dir) in options.mapdir {
        wasi.preopened_dir(
            Dir::from_std_file(std::fs::File::open(host_dir)?),
            guest_dir,
        )?;
    }

    for (name, value) in options.env {
        wasi.env(&name, &value)?;
    }

    let wasi = wasi.build();
    let mut store = Store::new(engine, Ctx { wasi });

    let instance = linker.instantiate(&mut store, &unsafe {
        Module::deserialize(
            engine,
            include_bytes!(concat!(env!("OUT_DIR"), "/wasm32-wasi/release/guest.cwasm")),
        )
    }?)?;

    let start = instance
        .get_func(&mut store, "_start")
        .ok_or_else(|| anyhow!("unable to find `_start` function"))?;

    start
        .call(&mut store, &[], &mut [])
        .or_else(ignore_successful_proc_exit_trap)?;

    Ok(())
}

fn ignore_successful_proc_exit_trap(guest_err: anyhow::Error) -> Result<()> {
    match guest_err.root_cause().downcast_ref::<I32Exit>() {
        Some(trap) => match trap.0 {
            0 => Ok(()),
            _ => Err(guest_err),
        },
        None => Err(guest_err),
    }
}
