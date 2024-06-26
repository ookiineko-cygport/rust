// See rsbegin.rs for details.

#![feature(no_core)]
#![feature(lang_items)]
#![feature(auto_traits)]
#![crate_type = "rlib"]
#![no_core]

#[lang = "sized"]
trait Sized {}
#[lang = "sync"]
trait Sync {}
impl<T> Sync for T {}
#[lang = "copy"]
trait Copy {}
#[lang = "freeze"]
auto trait Freeze {}

#[lang = "drop_in_place"]
#[inline]
#[allow(unconditional_recursion)]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    drop_in_place(to_drop);
}

#[cfg(all(any(target_os = "cygwin", all(target_os = "windows", target_env = "gnu")), target_arch = "x86"))]
pub mod eh_frames {
    // Terminate the frame unwind info section with a 0 as a sentinel;
    // this would be the 'length' field in a real FDE.
    #[no_mangle]
    #[link_section = ".eh_frame"]
    pub static __EH_FRAME_END__: u32 = 0;
}
