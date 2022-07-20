#![feature(c_variadic)]
#![feature(vec_into_raw_parts)]
#![feature(lang_items)]
#![feature(termination_trait_lib)]
#![feature(start)]

use rust_ios::objc::*;
use rust_ios::viewcontroller::init_my_viewcontroller;
use rust_ios::app_delegate::init_app_del;
use cstr::cstr;

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    init_app_del();
    init_my_viewcontroller();

    // Create an @autoreleasepool, using the old-stye API. 
    // Note that while NSAutoreleasePool IS deprecated, it still exists 
    // in the APIs for a reason, and we leverage that here. In a perfect 
    // world we wouldn't have to worry about this, but, remember, this is Rust.
    let autorelease_pool = unsafe {
        // [[NSAutoreleasePool alloc] init];
        rust_msg_send(rust_msg_send(  objc_getClass(cstr!("NSAutoreleasePool").as_ptr()), 
                                    sel_registerName(cstr!("alloc").as_ptr())), 
                                        sel_registerName(cstr!("init").as_ptr()))
    };

    unsafe {
        // [MyAppDelegate class]
        let app_del_class = rust_msg_send(objc_getClass(cstr!("MyAppDelegate").as_ptr()), sel_getUid(cstr!("class").as_ptr()));
        let app_del_str = NSStringFromClass(app_del_class);
        // [UIApplication class]
        let ui_app_class = rust_msg_send(objc_getClass(cstr!("UIApplication").as_ptr()), sel_getUid(cstr!("class").as_ptr()));
        let ui_app_str = NSStringFromClass(ui_app_class);
        UIApplicationMain(argc.try_into().unwrap(), argv as usize as *const *const i8, ui_app_str, app_del_str);
        // [autorelease_pool drain];
        rust_msg_send::<()>(autorelease_pool, sel_registerName(cstr!("drain").as_ptr()));
    }

    0
}
