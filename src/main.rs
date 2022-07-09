#![feature(c_variadic)]
#![feature(vec_into_raw_parts)]
#![feature(lang_items)]
#![feature(termination_trait_lib)]
#![feature(start)]

use rust_ios::objc::*;
use rust_ios::viewcontroller::init_my_viewcontroller;
use rust_ios::app_delegate::init_app_del;

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
        objc_msgSend(objc_msgSend(  rust_objc_get_class("NSAutoreleasePool"), 
                                    rust_sel_register_name("alloc")), 
                                        rust_sel_register_name("init"))
    };

    unsafe {
        // [MyAppDelegate class]
        let app_del_class = objc_msgSend(rust_objc_get_class("MyAppDelegate"), rust_sel_get_uid("class"));
        // [UIApplication class]
        let ui_app_class = objc_msgSend(rust_objc_get_class("UIApplication"), rust_sel_get_uid("class"));
        UIApplicationMain(argc.try_into().unwrap(), argv as usize as *const *const i8, NSStringFromClass(ui_app_class.to_class()), NSStringFromClass(app_del_class.to_class()));
        // [autorelease_pool drain];
        objc_msgSend(autorelease_pool, rust_sel_register_name("drain"));
    }

    0
}
