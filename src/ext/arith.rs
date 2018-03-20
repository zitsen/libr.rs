//! R_ext/Arith.h
//!
//! Given `isnan` and `finite` check functions.
//!
//! See [R Source]
//! (https://github.com/wch/r-source/blob/trunk/src/include/R_ext/Arith.h)
//!
//! # Examples
//! ```
//! extern crate libr;
//! use libr::ext::arith::*;
//!
//! fn main() {
//!     unsafe {
//!         // NAN
//!         assert!(isnan(0. / 0.) == 1);
//!         assert!(ISNAN(0. / 0.) == 1);
//!         assert!(ISNAN(std::f64::NAN) == 1);
//!         assert!(isnan(1.) == 0);
//!         assert!(ISNAN(1.) == 0);
//!
//!         // Finite or Infinite.
//!         assert!(finite(0.) == 1);
//!         assert_eq!(finite(0.), R_FINITE(0.));
//!         assert_eq!(finite(0.), R_finite(0.));
//!         let infinity = std::f64::INFINITY;
//!         assert!(finite(infinity) == 0);
//!         assert_eq!(finite(infinity), R_FINITE(infinity));
//!         assert_eq!(finite(infinity), R_finite(infinity));
//!
//!         // TODO: `NA` need test.
//!     }
//! }
//! ```

extern "C" {
    pub static mut R_NaN: ::libc::c_double;
    pub static mut R_PosInf: ::libc::c_double;
    pub static mut R_NegInf: ::libc::c_double;
    pub static mut R_NaReal: ::libc::c_double;
    pub static mut R_NaInt: ::libc::c_int;
}

extern "C" {
    pub fn isnan(__value: ::libc::c_double) -> ::libc::c_int;
    pub fn finite(__value: ::libc::c_double) -> ::libc::c_int;
    pub fn R_IsNA(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn R_IsNaN(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn R_finite(arg1: ::libc::c_double) -> ::libc::c_int;
}

pub use self::R_IsNA as ISNA;
pub use self::R_IsNaN as ISNAN;
pub use self::R_finite as R_FINITE;
