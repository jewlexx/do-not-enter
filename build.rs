use std::{env, fs, process};

fn main() {
    if cfg!(not(any(feature = "bsp_rpi4", feature = "bsp_rpi3"))) {
        eprintln!("This crate is only meant to be used on Raspberry Pi. You must enable either rpi4 or rpi3!");
        process::exit(1);
    }

    let ld_script_path = match env::var("LD_SCRIPT_PATH") {
        Ok(var) => var,
        _ => process::exit(0),
    };

    let files = fs::read_dir(ld_script_path).unwrap();
    files
        .filter_map(Result::ok)
        .filter(|d| {
            if let Some(e) = d.path().extension() {
                e == "ld"
            } else {
                false
            }
        })
        .for_each(|f| println!("cargo:rerun-if-changed={}", f.path().display()));
}
