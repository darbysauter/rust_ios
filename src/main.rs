#![feature(c_variadic)]
#![feature(vec_into_raw_parts)]

use std::ffi::CString;
use std::os::raw::c_char;
use rust_ios::objc::*;

fn main() {
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    let (argv, argc, _cap) = c_args.into_raw_parts();

    // Create an @autoreleasepool, using the old-stye API. 
    // Note that while NSAutoreleasePool IS deprecated, it still exists 
    // in the APIs for a reason, and we leverage that here. In a perfect 
    // world we wouldn't have to worry about this, but, remember, this is Rust.
    let autorelease_pool = unsafe {
        // [[NSAutoreleasePool alloc] init];
        objc_msgSend(objc_msgSend(  rust_objc_get_class("NSAutoreleasePool"), 
                                    rust_sel_register_name("alloc")), 
                                        rust_sel_register_name("init"))
    };

    unsafe {
        UIApplicationMain(argc.try_into().unwrap(), argv, 0, rust_cfstr("AppDelegate"));
        // [autorelease_pool drain];
        objc_msgSend(autorelease_pool, rust_sel_register_name("drain"));
    }
}