extern crate libc;
extern crate libr;
use libc::c_int;
use std::ptr;
use std::env;
use std::ffi::CString;

use libr::internals::*;
use libr::embedded::{self, Rf_endEmbeddedR, Rf_initEmbeddedR};

unsafe fn source(path: *const ::libc::c_char) {
    let e: SEXP = Rf_lang2(Rf_install(CString::new("source").unwrap().into_raw()),
                               Rf_mkString(path));
    Rf_protect(e);
    R_tryEval(e, R_GlobalEnv, ptr::null_mut::<libc::c_int>());
    Rf_unprotect(1);
}

fn main() {
    if let Err(_) = env::var("R_HOME") {
        panic!("Rembedded test need R_HOME be setted");
    }

    let mut s = Box::new(vec![CString::new("R").unwrap().into_raw(),
                              CString::new("--quiet").unwrap().into_raw(),
                              CString::new("--no-save").unwrap().into_raw()]);
    unsafe {
        embedded::Rf_initEmbeddedR(s.len() as i32, s.as_mut_ptr());
        source(CString::new("foo.R").unwrap().into_raw());
        embedded::Rf_endEmbeddedR(0);
    }
}
