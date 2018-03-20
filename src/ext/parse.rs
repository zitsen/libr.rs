use super::super::internals::SEXP;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ParseStatus {
    PARSE_NULL = 0,
    PARSE_OK,
    PARSE_INCOMPLETE,
    PARSE_ERROR,
    PARSE_EOF,
}

extern "C" {
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::libc::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;
}
