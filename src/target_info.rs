use std::{ffi::CStr, fmt};

pub struct TargetInfo {
    ptr: *mut gccjit_sys::gcc_jit_target_info,
}

unsafe impl Send for TargetInfo {}
unsafe impl Sync for TargetInfo {}

impl fmt::Debug for TargetInfo {
    fn fmt<'a>(&self, fmt: &mut fmt::Formatter<'a>) -> Result<(), fmt::Error> {
        "TargetInfo".fmt(fmt)
    }
}

impl TargetInfo {
    pub fn cpu_supports(&self, feature: &[u8]) -> bool {
        debug_assert!(feature.ends_with(&[b'\0']), "Expecting a NUL-terminated C string");
        unsafe {
            gccjit_sys::gcc_jit_target_info_cpu_supports(self.ptr, feature.as_ptr() as *const _) != 0
        }
    }

    pub fn arch(&self) -> Option<&'static CStr> {
        unsafe {
            let arch = gccjit_sys::gcc_jit_target_info_arch(self.ptr);
            if arch.is_null() {
                return None;
            }
            Some(CStr::from_ptr(arch))
        }
    }

    pub fn supports_128bit_int(&self) -> bool {
        unsafe {
            gccjit_sys::gcc_jit_target_info_supports_128bit_int(self.ptr) != 0
        }
    }
}

impl Drop for TargetInfo {
    fn drop(&mut self) {
        unsafe {
            gccjit_sys::gcc_jit_target_info_release(self.ptr);
        }
    }
}

pub unsafe fn from_ptr(ptr: *mut gccjit_sys::gcc_jit_target_info) -> TargetInfo {
    TargetInfo {
        ptr,
    }
}
