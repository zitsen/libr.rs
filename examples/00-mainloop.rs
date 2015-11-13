extern crate libr;
use std::env;
use std::ffi::CString;
use libr::embedded::Rf_initialize_R;
use libr::interface::*;

fn main() {
    if let Err(_) = env::var("R_HOME") {
        panic!("Rembedded test need R_HOME be setted");
    }
    let mut args = env::args()
                       .map(|arg| CString::new(arg.as_bytes()).unwrap().into_raw())
                       .collect::<Vec<_>>();
    unsafe {
        R_running_as_main_program = 1;
        Rf_initialize_R(args.len() as i32, args.as_mut_ptr());
        Rf_mainloop();
        return;
    }
}
