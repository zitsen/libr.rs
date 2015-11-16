extern crate libr;

use std::env;
use std::ffi::CString;

use libr::internals::*;
use libr::embedded::{self, Rf_endEmbeddedR, Rf_initEmbeddedR};
use libr::ext::parse::{ParseStatus, R_ParseVector};

fn main() {
    if let Err(_) = env::var("R_HOME") {
        panic!("Rembedded test need R_HOME be setted");
    }

    let mut s = Box::new(vec![CString::new("R").unwrap().into_raw(),
                              CString::new("--quiet").unwrap().into_raw(),
                              CString::new("--no-save").unwrap().into_raw()]);
    unsafe {
        embedded::Rf_initEmbeddedR(s.len() as i32, s.as_mut_ptr());
        // Plot
        let mut status = ParseStatus::PARSE_OK;
        let mut had_error = 0;
        let tmp = Rf_protect(Rf_mkString(CString::new("{pdf(\"03-plot.pdf\"); plot(1:10, \
                                                           pch=\"+\"); print(1:10)}")
                                                 .unwrap()
                                                 .into_raw()));
        let e = Rf_protect(R_ParseVector(tmp, 1, &mut status, R_NilValue));
        Rf_PrintValue(e);
        R_tryEval(VECTOR_ELT(e, 0), R_GlobalEnv, &mut had_error);
        Rf_unprotect(2);
        embedded::Rf_endEmbeddedR(0);
    }
}
