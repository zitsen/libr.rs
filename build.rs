extern crate pkg_config;
extern crate bindgen;
extern crate walkdir;

use bindgen::builder;
use walkdir::WalkDir;

fn main() {
    let libr = pkg_config::probe_library("libR").expect("find libR from pkgconfig");
    eprintln!("{:?}", libr);

    let args = libr.include_paths.iter().map(|path| format!("-I{}", path.display())).collect::<Vec<_>>();

    fn generate(header: &str) {
        let bindings = builder().header(format!("__i/{}.h", header)).generate().expect("header");
        bindings.write_to_file(format!("src/{}.rs", header)).expect("write");
    }

    for entry in WalkDir::new("__i").into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|entry| entry.path().file_stem().unwrap().to_string_lossy().to_string()) {
        let bindings = builder()
            .clang_args(args.iter())
            .header(format!("__i/{}.h", entry))
            .generate()
            .expect(&format!("find header {}", entry));
        bindings.write_to_file(format!("src/{}.rs", entry)).expect("write");
    }

}
