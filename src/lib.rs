#[link(name = "set_stack_pointer")]
extern "C" {
    /// Do not call this directly from Rust code.
    /// It's only meant to be used from the Rust host environment.
    #[doc(hidden)]
    pub fn set_stack_pointer(i: i32);
}

// This is just to ensure that function is included in the output binary.
// There's probably a better way to ensure that.
/*
#[doc(hidden)]
#[no_mangle]
pub extern "C" fn __do_not_call() {
    unsafe {
        set_stack_pointer(0);
    }
}
*/
