use crate::objc::*;
use std::mem;
use cstr::cstr;

pub extern "C" fn app_del_did_finish_launching(obj: *mut Id, _cmd: Sel, _application: usize, _options: usize) -> Bool {
    unsafe {
        // [[MyViewController alloc] init]
        let view_controller: *mut Id = 
        rust_msg_send(
            rust_msg_send(objc_getClass(cstr!("MyViewController").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), 
                sel_getUid(cstr!("init").as_ptr()));

        // [[UIScreen mainScreen] bounds]
        let bounds: CGRect = 
        rust_msg_send(
            rust_msg_send(objc_getClass(cstr!("UIScreen").as_ptr()), sel_getUid(cstr!("mainScreen").as_ptr())), 
                sel_getUid(cstr!("bounds").as_ptr()));
        // let bounds: CGRect = CGRect::new(0.0, 0.0, 400.0, 400.0);

        // [[UIWindow alloc] initWithFrame:bounds]
        let window: *mut Id = 
        rust_msg_send(
            rust_msg_send(objc_getClass(cstr!("UIWindow").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), 
                sel_getUid(cstr!("initWithFrame:").as_ptr()), bounds.origin.x, bounds.origin.y, bounds.size.width, bounds.size.height);

        // self.window = window
        let ivar = class_getInstanceVariable(objc_getClass(cstr!("MyAppDelegate").as_ptr()), cstr!("window").as_ptr());
        object_setIvar(obj, ivar, window);

        rust_msg_send::<()>(window, sel_getUid(cstr!("setRootViewController:").as_ptr()), view_controller);

        rust_msg_send::<()>(window, sel_getUid(cstr!("makeKeyAndVisible").as_ptr()));
    }
    
    return Bool::Yes;
}

pub fn init_app_del() {
    unsafe {
        let app_del_class = objc_allocateClassPair(objc_getClass(cstr!("UIResponder").as_ptr()), cstr!("MyAppDelegate").as_ptr(), 0);

        class_addIvar(app_del_class, cstr!("window").as_ptr(), mem::size_of::<Id>(), (mem::size_of::<Id>() as f64).log2() as u8, cstr!("@").as_ptr());

        let cast_fn: usize = app_del_did_finish_launching as *const () as usize;
        class_addMethod(app_del_class, sel_getUid(cstr!("application:didFinishLaunchingWithOptions:").as_ptr()), cast_fn, cstr!("i@:@@").as_ptr());

        objc_registerClassPair(app_del_class);
    }
}