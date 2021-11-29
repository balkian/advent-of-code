use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new("src").join("solutions.rs");
    let sol_dir = Path::new("src").join("solutions");
    let mut f = File::create(&dest_path).unwrap();

    println!("cargo:rerun-if-changed=src/solutions");

    f.write_all(b"use crate::aoc_main;\n\n").unwrap();
    dbg!(&sol_dir);
    let paths = fs::read_dir(sol_dir).unwrap();
    f.write_all(b"aoc_main!(").unwrap();
    for path in paths {
        let fpath = path.unwrap().path();
        let mod_name = fpath.file_stem().unwrap().to_str().unwrap();
        f.write_all(format!("{};", mod_name).as_bytes()).unwrap();
    }
    f.write_all(b");").unwrap();
}
