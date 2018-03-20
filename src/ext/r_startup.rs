//! R_ext/RStartup.h
//!
//! C functions to be called from alternative front-ends.
//!
//! Part of the API for such front-ends but not for packages.
//!
use super::boolean::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum SA_TYPE {
    SA_NORESTORE = 0,
    SA_RESTORE,
    SA_DEFAULT, // was === SA_RESTORE
    SA_NOSAVE,
    SA_SAVE,
    SA_SAVEASK,
    SA_SUICIDE,
}
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy)]
pub struct structRstart {
    pub R_Quiet: Rboolean,
    pub R_Slave: Rboolean,
    pub R_Interactive: Rboolean,
    pub R_Verbose: Rboolean,
    pub LoadSiteFile: Rboolean,
    pub LoadInitFile: Rboolean,
    pub DebugInitFile: Rboolean,
    pub RestoreAction: SA_TYPE,
    pub SaveAction: SA_TYPE,
    pub vsize: ::libc::size_t,
    pub nsize: ::libc::size_t,
    pub max_vsize: ::libc::size_t,
    pub max_nsize: ::libc::size_t,
    pub ppsize: ::libc::size_t,
    pub NoRenviron: ::libc::c_int,
}
impl ::std::clone::Clone for structRstart {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for structRstart {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Rstart = *mut structRstart;

extern "C" {
    pub fn R_DefParams(arg1: Rstart) -> ();
    pub fn R_SetParams(arg1: Rstart) -> ();
    pub fn R_SetWin32(arg1: Rstart) -> ();
    pub fn R_SizeFromEnv(arg1: Rstart) -> ();
    pub fn R_common_command_line(
        arg1: *mut ::libc::c_int,
        arg2: *mut *mut ::libc::c_char,
        arg3: Rstart,
    ) -> ();
    pub fn R_set_command_line_arguments(argc: ::libc::c_int, argv: *mut *mut ::libc::c_char) -> ();
    pub fn setup_Rmainloop() -> ();
}
