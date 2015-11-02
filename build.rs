extern crate pkg_config;

fn main() {
    match pkg_config::find_library("libR") {
        Ok(..) => return,
        Err(e) => panic!("Couldn't find libR from \
                           pkgconfig ({:?}), failed", e),
    }
}
