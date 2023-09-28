use {
    anyhow::{Result},
    std::io,
};

#[link(name = "mylib", kind = "static")]
extern "C" {
    fn run1234(len: usize, guest: *const u8) -> i32;
}

fn main() -> Result<(), io::Error> {
    let guest: Vec<u8> = include_bytes!("guest.cwasm").to_vec();

    /*
    let _result = run_internal(&guest);

    Ok(())
    */
    let result: i32 = unsafe {
        run1234(guest.len(), guest.as_ptr())
    };

    match result {
        0 => Ok(()),
        _ => Err(io::ErrorKind::Other.into()),
    }
}
