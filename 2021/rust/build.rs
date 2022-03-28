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
    let mut mods: Vec<_> = fs::read_dir(sol_dir)
        .unwrap()
        .map(|r| r.unwrap()
             .path())
        .filter_map(|p| {
            if p.extension()?.to_str()?.ends_with("rs") {
                Some(p.file_stem()?.to_str()?.to_owned())
            } else {
                None
            }
        })
        .filter(|x| !x.starts_with('.'))
        .collect();
    mods.sort();
    f.write_all(b"aoc_main!(").unwrap();
    for mod_name in mods.into_iter() {
        f.write_all(format!("{};", mod_name).as_bytes()).unwrap();
    }
    f.write_all(b");").unwrap();
}
