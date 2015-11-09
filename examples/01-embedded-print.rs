extern crate libr;

use std::env;
use std::ffi::CString;

use libr::ffi::embedded;
use libr::ffi::internals::*;

fn main() {
    if let Err(_) = env::var("R_HOME") {
        panic!("Rembedded test need R_HOME be setted");
    }

    let mut s = Box::new(vec![CString::new("R").unwrap().into_raw(),
                              CString::new("--quiet").unwrap().into_raw(),
                              CString::new("--no-save").unwrap().into_raw()]);
    unsafe {
        embedded::Rf_initEmbeddedR(s.len() as i32, s.as_mut_ptr());
        Rprintf(CString::new("Hello world").unwrap().into_raw());
        embedded::Rf_endEmbeddedR(0);
    }
}
