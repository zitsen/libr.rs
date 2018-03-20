//! R_ext/Callbacks.h
//!
//! Not part of the API, subject to change at any time. -- From R
//!
//! These structures are for C (and R function) top-level task handlers.
//! Such routines are called at the end of every (successful) top-level task
//! in the regular REPL.

pub use super::super::internals::SEXP;
pub use super::boolean::*;

#[allow(non_camel_case_types)]
pub type R_ToplevelCallback = ::std::option::Option<
    unsafe extern "C" fn(
        expr: SEXP,
        value: SEXP,
        succeeded: Rboolean,
        visible: Rboolean,
        arg1: *mut ::libc::c_void,
    ) -> Rboolean,
>;
#[allow(non_camel_case_types)]
pub type R_ToplevelCallbackEl = Struct__ToplevelCallback;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__ToplevelCallback {
    pub cb: R_ToplevelCallback,
    pub data: *mut ::libc::c_void,
    pub finalizer: ::std::option::Option<unsafe extern "C" fn(data: *mut ::libc::c_void) -> ()>,
    pub name: *mut ::libc::c_char,
    pub next: *mut R_ToplevelCallbackEl,
}
impl ::std::clone::Clone for Struct__ToplevelCallback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__ToplevelCallback {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(non_camel_case_types)]
pub type R_ObjectTable = Struct__R_ObjectTable;
#[allow(non_camel_case_types)]
pub type Rdb_exists = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const ::libc::c_char,
        canCache: *mut Rboolean,
        arg1: *mut R_ObjectTable,
    ) -> Rboolean,
>;
#[allow(non_camel_case_types)]
pub type Rdb_get = ::std::option::Option<
    unsafe extern "C" fn(
        name: *const ::libc::c_char,
        canCache: *mut Rboolean,
        arg1: *mut R_ObjectTable,
    ) -> SEXP,
>;
#[allow(non_camel_case_types)]
pub type Rdb_remove = ::std::option::Option<
    unsafe extern "C" fn(name: *const ::libc::c_char, arg1: *mut R_ObjectTable) -> ::libc::c_int,
>;
#[allow(non_camel_case_types)]
pub type Rdb_assign = ::std::option::Option<
    unsafe extern "C" fn(name: *const ::libc::c_char, value: SEXP, arg1: *mut R_ObjectTable)
        -> SEXP,
>;
#[allow(non_camel_case_types)]
pub type Rdb_objects =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut R_ObjectTable) -> SEXP>;
#[allow(non_camel_case_types)]
pub type Rdb_canCache = ::std::option::Option<
    unsafe extern "C" fn(name: *const ::libc::c_char, arg1: *mut R_ObjectTable) -> Rboolean,
>;
#[allow(non_camel_case_types)]
pub type Rdb_onDetach = ::std::option::Option<unsafe extern "C" fn(arg1: *mut R_ObjectTable) -> ()>;
#[allow(non_camel_case_types)]
pub type Rdb_onAttach = ::std::option::Option<unsafe extern "C" fn(arg1: *mut R_ObjectTable) -> ()>;
#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct Struct__R_ObjectTable {
    pub _type: ::libc::c_int,
    pub cachedNames: *mut *mut ::libc::c_char,
    pub active: Rboolean,
    pub exists: Rdb_exists,
    pub get: Rdb_get,
    pub remove: Rdb_remove,
    pub assign: Rdb_assign,
    pub objects: Rdb_objects,
    pub canCache: Rdb_canCache,
    pub onDetach: Rdb_onDetach,
    pub onAttach: Rdb_onAttach,
    pub privateData: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct__R_ObjectTable {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct__R_ObjectTable {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

extern "C" {
    pub fn Rf_removeTaskCallbackByIndex(id: ::libc::c_int) -> Rboolean;
    pub fn Rf_removeTaskCallbackByName(name: *const ::libc::c_char) -> Rboolean;
    pub fn R_removeTaskCallback(which: SEXP) -> SEXP;
    pub fn Rf_addTaskCallback(
        cb: R_ToplevelCallback,
        data: *mut ::libc::c_void,
        finalizer: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::libc::c_void) -> ()>,
        name: *const ::libc::c_char,
        pos: *mut ::libc::c_int,
    ) -> *mut R_ToplevelCallbackEl;
}
