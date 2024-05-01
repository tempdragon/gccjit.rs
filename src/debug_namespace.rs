use std::fmt;
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct DebugNamespace {
    ptr: *mut gccjit_sys::gcc_jit_debug_namespace,
}

unsafe impl Send for DebugNamespace {}
unsafe impl Sync for DebugNamespace {}

impl fmt::Debug for DebugNamespace {
    fn fmt<'a>(&self, fmt: &mut fmt::Formatter<'a>) -> Result<(), fmt::Error> {
        "DebugNamespace".fmt(fmt)
    }
}

impl DebugNamespace {
    pub fn as_ptr(&self) -> *const gccjit_sys::gcc_jit_debug_namespace {
        self.ptr
    }
    pub fn from_ptr(ptr: *mut gccjit_sys::gcc_jit_debug_namespace) -> Self {
        Self { ptr }
    }
}
