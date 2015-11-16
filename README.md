Native C types and bindings of libR.

## Usage

First, add the following to your Cargo.toml:

```lang=toml
[dependencies]
libr = 0.1
```

Next, add this to your crate root:

```
extern crate libr;
```

## What is libr?

The primary purpose of this crate is to provide an easily
interface for embedding R functions. This include type
definitions (e.g. `SEXP`), constants(e.g. `PI`) as well as
function headers(e.g. `Rf_initEmbeddedR`)

The library now support 64bit Linux only, for the case
I have no idea to how to test it in other system. I'm
a newer in [Rust](http://rust-lang.org), so it's so welcome
for help and suggestions.

## Examples

### For a similar native R-like app, use this:

```
extern crate libr;
use std::env;
use std::ffi::CString;
use libr::embedded::Rf_initialize_R;
use libr::interface::*;

fn main() {
    if let Err(_) = env::var("R_HOME") {
        panic!("Rembedded test need R_HOME be setted");
    }
    let args = vec!["R", "--no-save"];
    let mut args = args.into_iter()
       .map(|arg|
           CString::new(arg.as_bytes()).unwrap().into_raw())
       .collect::<Vec<_>>();
    unsafe {
       R_running_as_main_program = 1;
       Rf_initialize_R(args.len() as i32, args.as_mut_ptr());
       Rf_mainloop();
       return;
    }
}
```

### R math functions

See details in [math](math/index.html).

```
extern crate libr;
use libr::math::R_pow;

fn main() {
    assert_eq!(unsafe { R_pow(2., 3.) }, 8.);
}
```

### Embedded R

See details documents in [embedded module](embedded/index.html).

```
extern crate libr;
use std::env;
use std::ffi::CString;

use libr::internals::*;
use libr::embedded as Rembedded;
use libr::ext::parse::{ParseStatus, R_ParseVector};

fn main() {
    if let Err(_) = env::var("R_HOME") {
        panic!("Rembedded test need R_HOME be setted");
    }
    let mut s = Box::new(vec![CString::new("R").unwrap().into_raw(),
                         CString::new("--quiet").unwrap().into_raw(),
                         CString::new("--no-save").unwrap().into_raw()]);
    unsafe {
        Rembedded::Rf_initEmbeddedR(s.len() as i32, s.as_mut_ptr());
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
        Rembedded::Rf_endEmbeddedR(0);
    }
}
```
