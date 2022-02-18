#![feature(unboxed_closures)]
#![feature(fn_traits)]

// needs nightly currently (1.60)

use std::os::raw;
use std::panic;

pub struct WatchMyClosure;

impl FnOnce<()> for WatchMyClosure {
    type Output = ();

    extern "rust-call" fn call_once(self, args: ()) -> Self::Output {
        println!("Calling WatchMyClosure for FnOnce");
    }
}

impl FnMut<()> for WatchMyClosure {
    extern "rust-call" fn call_mut(&mut self, args: ()) -> Self::Output {
        println!("Calling WatchMyClosure for FnMut");
    }
}

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

fn main() {
    // awake_callback(|| println!("It's dangerous to go alone! Take my leak!"));
    // awake_callback(|| println!("It's dangerous to go alone! Take my leak!"));
    // awake_callback(|| println!("It's dangerous to go alone! Take my leak!"));

    // valgrind --tool=memcheck --leak-check=full --leak-resolution=high --track-origins=yes --num-callers=128 ./leakingcast
    // gives:
    // ==31460== LEAK SUMMARY:
    // ==31460==    definitely lost: 48 bytes in 3 blocks
    // ==31460==    indirectly lost: 0 bytes in 0 blocks
    // ==31460==      possibly lost: 0 bytes in 0 blocks
    // ==31460==    still reachable: 0 bytes in 0 blocks
    // ==31460==         suppressed: 0 bytes in 0 blocks
    // 16 must be the size of the Box repr on my target

    please_no_leaks_awake_callback(|| println!("Now it's not dangerous!"));
    please_no_leaks_awake_callback(|| println!("Now it's not dangerous!"));
    please_no_leaks_awake_callback(|| println!("Now it's not dangerous!"));

    // valgrind --tool=memcheck --leak-check=full --leak-resolution=high --track-origins=yes --num-callers=128 ./leakingcast
    // gives:
    // All heap blocks were freed -- no leaks are possible
}