use super::boolean::*;
use super::complex::*;
use libc::size_t;

// #define revsort       Rf_revsort
pub use self::Rf_revsort as revsort;
// #define iPsort        Rf_iPsort
pub use self::Rf_iPsort as iPsort;
// #define rPsort        Rf_rPsort
pub use self::Rf_rPsort as rPsort;
// #define cPsort        Rf_cPsort
pub use self::Rf_cPsort as cPsort;
// #define IndexWidth    Rf_IndexWidth /* there's no IndexWidth */
// #define setIVector    Rf_setIVector
pub use self::Rf_setIVector as setIVector;
// #define setRVector    Rf_setRVector
pub use self::Rf_setRVector as setRVector;
// #define StringFalse   Rf_StringFalse
pub use self::Rf_StringFalse as StringFalse;
// #define StringTrue    Rf_StringTrue
pub use self::Rf_StringTrue as StringTrue;
// #define isBlankString Rf_isBlankString
pub use self::Rf_isBlankString as isBlankString;

#[test]
fn test_revsort() {
    unsafe {
        let mut arg1 = [1.1, 2.2, 3.3];
        let mut arg2 = [1, 2, 3];
        revsort(
            ::std::ptr::null_mut::<::libc::c_double>(),
            ::std::ptr::null_mut::<::libc::c_int>(),
            0,
        );
        Rf_revsort(arg1.as_mut_ptr(), arg2.as_mut_ptr(), 2);
        assert_eq!(arg1, [2.2, 1.1, 3.3]);
        assert_eq!(arg2, [2, 1, 3]);
        Rf_revsort(arg1.as_mut_ptr(), arg2.as_mut_ptr(), 3);
        assert_eq!(arg1, [3.3, 2.2, 1.1]);
        assert_eq!(arg2, [3, 2, 1]);
        Rf_revsort(arg1.as_mut_ptr(), arg2.as_mut_ptr(), 4);
    }
}

extern "C" {
    pub fn R_isort(arg1: *mut ::libc::c_int, arg2: ::libc::c_int) -> ();
    pub fn R_rsort(arg1: *mut ::libc::c_double, arg2: ::libc::c_int) -> ();
    pub fn R_csort(arg1: *mut Rcomplex, arg2: ::libc::c_int) -> ();
    pub fn rsort_with_index(
        arg1: *mut ::libc::c_double,
        arg2: *mut ::libc::c_int,
        arg3: ::libc::c_int,
    ) -> ();
    pub fn Rf_revsort(
        arg1: *mut ::libc::c_double,
        arg2: *mut ::libc::c_int,
        arg3: ::libc::c_int,
    ) -> ();
    pub fn Rf_iPsort(arg1: *mut ::libc::c_int, arg2: ::libc::c_int, arg3: ::libc::c_int) -> ();
    pub fn Rf_rPsort(arg1: *mut ::libc::c_double, arg2: ::libc::c_int, arg3: ::libc::c_int) -> ();
    pub fn Rf_cPsort(arg1: *mut Rcomplex, arg2: ::libc::c_int, arg3: ::libc::c_int) -> ();
    pub fn R_qsort(v: *mut ::libc::c_double, i: size_t, j: size_t) -> ();
    pub fn R_qsort_I(
        v: *mut ::libc::c_double,
        II: *mut ::libc::c_int,
        i: ::libc::c_int,
        j: ::libc::c_int,
    ) -> ();
    pub fn R_qsort_int(iv: *mut ::libc::c_int, i: size_t, j: size_t) -> ();
    pub fn R_qsort_int_I(
        iv: *mut ::libc::c_int,
        II: *mut ::libc::c_int,
        i: ::libc::c_int,
        j: ::libc::c_int,
    ) -> ();
    pub fn R_ExpandFileName(arg1: *const ::libc::c_char) -> *const ::libc::c_char;
    pub fn Rf_setIVector(arg1: *mut ::libc::c_int, arg2: ::libc::c_int, arg3: ::libc::c_int) -> ();
    pub fn Rf_setRVector(
        arg1: *mut ::libc::c_double,
        arg2: ::libc::c_int,
        arg3: ::libc::c_double,
    ) -> ();
    pub fn Rf_StringFalse(arg1: *const ::libc::c_char) -> Rboolean;
    pub fn Rf_StringTrue(arg1: *const ::libc::c_char) -> Rboolean;
    pub fn Rf_isBlankString(arg1: *const ::libc::c_char) -> Rboolean;
    pub fn R_atof(str: *const ::libc::c_char) -> ::libc::c_double;
    pub fn R_strtod(c: *const ::libc::c_char, end: *mut *mut ::libc::c_char) -> ::libc::c_double;
    pub fn R_tmpnam(
        prefix: *const ::libc::c_char,
        tempdir: *const ::libc::c_char,
    ) -> *mut ::libc::c_char;
    pub fn R_tmpnam2(
        prefix: *const ::libc::c_char,
        tempdir: *const ::libc::c_char,
        fileext: *const ::libc::c_char,
    ) -> *mut ::libc::c_char;
    pub fn R_CheckUserInterrupt() -> ();
    pub fn R_CheckStack() -> ();
    pub fn R_CheckStack2(arg1: size_t) -> ();
    pub fn findInterval(
        xt: *mut ::libc::c_double,
        n: ::libc::c_int,
        x: ::libc::c_double,
        rightmost_closed: Rboolean,
        all_inside: Rboolean,
        ilo: ::libc::c_int,
        mflag: *mut ::libc::c_int,
    ) -> ::libc::c_int;
    pub fn find_interv_vec(
        xt: *mut ::libc::c_double,
        n: *mut ::libc::c_int,
        x: *mut ::libc::c_double,
        nx: *mut ::libc::c_int,
        rightmost_closed: *mut ::libc::c_int,
        all_inside: *mut ::libc::c_int,
        indx: *mut ::libc::c_int,
    ) -> ();
    pub fn R_max_col(
        matrix: *mut ::libc::c_double,
        nr: *mut ::libc::c_int,
        nc: *mut ::libc::c_int,
        maxes: *mut ::libc::c_int,
        ties_meth: *mut ::libc::c_int,
    ) -> ();
}
