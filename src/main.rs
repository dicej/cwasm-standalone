use std::io;

extern "C" {
    fn run_wasm_native_binary(len: usize, guest: *const u8) -> i32;
}

fn main() -> Result<(), std::io::Error> {
    let guest: Vec<u8> = include_bytes!("guest.cwasm").to_vec();

    let result: i32 = unsafe {
        run_wasm_native_binary(guest.len(), guest.as_ptr())
    };

    match result {
        0 => Ok(()),
        _ => Err(io::ErrorKind::Other.into()),
    }
}
