use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new("src").join("main.rs");
    let mut f = File::create(&dest_path).unwrap();
    f.write_all(b"mod solutions;\n\n").unwrap();
    f.write_all(b"fn main() {\n    solutions::main();\n}\n")
        .unwrap();

    let dest_path = Path::new("src").join("solutions.rs");
    let mut f = File::create(&dest_path).unwrap();
    let sol_dir = Path::new("src").join("solutions");
    let sol_dir_path = sol_dir.as_path().to_str().unwrap();
    println!("cargo:rerun-if-changed={sol_dir_path}");

    f.write_all(b"use aoc_utils::aoc_main;\n\n").unwrap();
    dbg!(&sol_dir);
    let mut mods: Vec<_> = fs::read_dir(sol_dir)
        .unwrap()
        .map(|r| r.unwrap().path())
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
    f.write_all(b");\n").unwrap();
}
