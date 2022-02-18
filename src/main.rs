use std::ffi::c_void;
use std::os::raw;
use std::panic;

extern "C" {
    pub fn Fl_awake_callback(
        handler: Fl_Awake_Handler,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}

pub type Fl_Awake_Handler =
::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>;

pub fn awake_callback<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Fl_Awake_Handler = Some(shim);
        Fl_awake_callback(callback, data);
    }
}

pub fn minimal_fix_awake_callback<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let mut a: Box<Box<dyn FnMut()>> = Box::from_raw(data as *mut Box<dyn FnMut()>);
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Fl_Awake_Handler = Some(shim);
        Fl_awake_callback(callback, data);
    }
}

// F can be lifted into inner fn shim, however it must increase binary code size,
// e.g. monomorphisation of fn shim for each used F
pub fn please_no_leaks_awake_callback<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim<F>(data: *mut raw::c_void) {
                // and don't forget cast back to box for a heap deallocation at the end of the scope
            let f = Box::<F>::from_raw(data as *mut F);
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {f}));
        }
        let data = Box::into_raw(Box::new(cb)) as *mut raw::c_void;
        let callback: Fl_Awake_Handler = Some(shim::<F>);
        Fl_awake_callback(callback, data);
    }
}

// fn main() {
//     awake_callback(|| println!("It's dangerous to go alone! Take my leak!"));
// }
// produces:
// valgrind --leak-check=full --leak-resolution=high --track-origins=yes  ./leakingcast
// ==10414== Memcheck, a memory error detector
// ==10414== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
// ==10414== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
// ==10414== Command: ./leakingcast
// ==10414==
// Callback address: 0x10f980, data address: 0x5a72a60
// It's dangerous to go alone! Take my leak!
// ==10414==
// ==10414== HEAP SUMMARY:
// ==10414==     in use at exit: 16 bytes in 1 blocks
// ==10414==   total heap usage: 11 allocs, 10 frees, 4,117 bytes allocated
// ==10414==
// ==10414== 16 bytes in 1 blocks are definitely lost in loss record 1 of 1
// ==10414==    at 0x4C31B0F: malloc (in /usr/lib/valgrind/vgpreload_memcheck-amd64-linux.so)
// ==10414==    by 0x11053B: alloc::alloc::alloc (alloc.rs:87)
// ==10414==    by 0x110666: alloc::alloc::Global::alloc_impl (alloc.rs:169)
// ==10414==    by 0x110789: <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:229)
// ==10414==    by 0x11085C: alloc::alloc::exchange_malloc (alloc.rs:318)
// ==10414==    by 0x11095A: new<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output=()>, alloc::alloc::Global>> (boxed.rs:195)
// ==10414==    by 0x11095A: leakingcast::awake_callback (main.rs:22)
// ==10414==    by 0x10F9E5: leakingcast::main (main.rs:59)
// ==10414==    by 0x10F47A: core::ops::function::FnOnce::call_once (function.rs:227)
// ==10414==    by 0x10FD8D: std::sys_common::backtrace::__rust_begin_short_backtrace (backtrace.rs:122)
// ==10414==    by 0x10F3A0: std::rt::lang_start::{{closure}} (rt.rs:145)
// ==10414==    by 0x124000: call_once<(), (dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (function.rs:259)
// ==10414==    by 0x124000: do_call<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panicking.rs:492)
// ==10414==    by 0x124000: try<i32, &(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe)> (panicking.rs:456)
// ==10414==    by 0x124000: catch_unwind<&(dyn core::ops::function::Fn<(), Output=i32> + core::marker::Sync + core::panic::unwind_safe::RefUnwindSafe), i32> (panic.rs:137)
// ==10414==    by 0x124000: {closure#2} (rt.rs:128)
// ==10414==    by 0x124000: do_call<std::rt::lang_start_internal::{closure_env#2}, isize> (panicking.rs:492)
// ==10414==    by 0x124000: try<isize, std::rt::lang_start_internal::{closure_env#2}> (panicking.rs:456)
// ==10414==    by 0x124000: catch_unwind<std::rt::lang_start_internal::{closure_env#2}, isize> (panic.rs:137)
// ==10414==    by 0x124000: std::rt::lang_start_internal (rt.rs:128)
// ==10414==    by 0x10F36F: std::rt::lang_start (rt.rs:144)
// ==10414==
// ==10414== LEAK SUMMARY:
// ==10414==    definitely lost: 16 bytes in 1 blocks
// ==10414==    indirectly lost: 0 bytes in 0 blocks
// ==10414==      possibly lost: 0 bytes in 0 blocks
// ==10414==    still reachable: 0 bytes in 0 blocks
// ==10414==         suppressed: 0 bytes in 0 blocks
// ==10414==
// ==10414== For counts of detected and suppressed errors, rerun with: -v
// ==10414== ERROR SUMMARY: 1 errors from 1 contexts (suppressed: 0 from 0)



fn main() {
    minimal_fix_awake_callback(|| println!("It's dangerous to go alone! Take my leak!"));
}
// produces:
// valgrind --leak-check=full --leak-resolution=high --track-origins=yes  ./leakingcast
// ==10680== Memcheck, a memory error detector
// ==10680== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
// ==10680== Using Valgrind-3.13.0 and LibVEX; rerun with -h for copyright info
// ==10680== Command: ./leakingcast
// ==10680==
// Callback address: 0x10f9f0, data address: 0x5a72a60
// It's dangerous to go alone! Take my leak!
// ==10680==
// ==10680== HEAP SUMMARY:
// ==10680==     in use at exit: 0 bytes in 0 blocks
// ==10680==   total heap usage: 11 allocs, 11 frees, 4,117 bytes allocated
// ==10680==
// ==10680== All heap blocks were freed -- no leaks are possible
// ==10680==
// ==10680== For counts of detected and suppressed errors, rerun with: -v
// ==10680== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)