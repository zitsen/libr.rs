#[allow(non_camel_case_types)]
pub type va_list = [va_list_tag; 1usize];

#[repr(C)]
#[derive(Copy)]
pub struct va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}

impl ::std::clone::Clone for va_list_tag {
    fn clone(&self) -> Self {
        *self
    }
}

impl ::std::default::Default for va_list_tag {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
