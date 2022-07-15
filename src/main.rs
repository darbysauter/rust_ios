#![feature(c_variadic)]
#![feature(vec_into_raw_parts)]
#![feature(lang_items)]
#![feature(termination_trait_lib)]
#![feature(start)]

use rust_ios::objc::*;
use rust_ios::viewcontroller::init_my_viewcontroller;
use rust_ios::app_delegate::init_app_del;
use cstr::cstr;
use std::ffi::CStr;

extern "C" {
    fn actual_main(argc: isize, argv: *const *const u8);
    fn initAppDel();
    fn initMyVC();
}

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    unsafe {
        actual_main(argc, argv);
        // initAppDel();
        // initMyVC();
    }

    // init_app_del();
    // init_my_viewcontroller();

    // // Create an @autoreleasepool, using the old-stye API. 
    // // Note that while NSAutoreleasePool IS deprecated, it still exists 
    // // in the APIs for a reason, and we leverage that here. In a perfect 
    // // world we wouldn't have to worry about this, but, remember, this is Rust.
    // let autorelease_pool = unsafe {
    //     // [[NSAutoreleasePool alloc] init];
    //     rust_msg_send(rust_msg_send(  objc_getClass(cstr!("NSAutoreleasePool").as_ptr()), 
    //                                 sel_registerName(cstr!("alloc").as_ptr())), 
    //                                     sel_registerName(cstr!("init").as_ptr()))
    // };

    // unsafe {
    //     // [MyAppDelegate class]
    //     let app_del_class = rust_msg_send(objc_getClass(cstr!("MyAppDelegate").as_ptr()), sel_getUid(cstr!("class").as_ptr()));
    //     // [UIApplication class]
    //     let ui_app_class = rust_msg_send(objc_getClass(cstr!("UIApplication").as_ptr()), sel_getUid(cstr!("class").as_ptr()));
    //     UIApplicationMain(argc.try_into().unwrap(), argv as usize as *const *const i8, NSStringFromClass(ui_app_class), NSStringFromClass(app_del_class));
    //     // [autorelease_pool drain];
    //     rust_msg_send::<()>(autorelease_pool, sel_registerName(cstr!("drain").as_ptr()));
    // }

    0
}
