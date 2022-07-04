use ctor::*;
use crate::objc::*;
use std::mem;

extern "C" fn app_del_did_finish_launching(obj: Id, _cmd: Sel, _application: usize, _options: usize) -> Bool {
    unsafe {
        // [[MyViewController alloc] init]
        let view_controller = objc_msgSend(objc_msgSend(rust_objc_get_class("MyViewController"), rust_sel_get_uid("alloc")), rust_sel_get_uid("init"));

        // [[UIScreen mainScreen] bounds]
        let bounds: CGRect = rust_objc_msg_send_ret_cgrect(objc_msgSend(rust_objc_get_class("UIScreen"), rust_sel_get_uid("mainScreen")), rust_sel_get_uid("bounds"));

        // [[UIWindow alloc] initWithFrame:bounds]
        let window = objc_msgSend(objc_msgSend(rust_objc_get_class("UIWindow"), rust_sel_get_uid("alloc")), rust_sel_get_uid("initWithFrame:"), bounds);

        // self.window = window
        let ivar = rust_class_get_instance_variable(rust_objc_get_class("MyAppDelegate").to_class(), "window");
        object_setIvar(obj, ivar, window);

        objc_msgSend(window, rust_sel_get_uid("setRootViewController:"), view_controller);

        objc_msgSend(window, rust_sel_get_uid("makeKeyAndVisible"));
    }
    
    return Bool::Yes;
}

#[ctor]
fn init_app_del() {
    let app_del_class = rust_objc_allocate_class_pair(rust_objc_get_class("UIResponder"), "MyAppDelegate", 0);

    rust_class_add_ivar(app_del_class, "window", mem::size_of::<Id>(), (mem::size_of::<Id>() as f64).log2() as u8, "@");

    let cast_fn: Imp = unsafe {
        *(&app_del_did_finish_launching as *const _ as usize as *const Imp)
    };
    rust_class_add_method(app_del_class, rust_sel_get_uid("application:didFinishLaunchingWithOptions:"), cast_fn, "i@:@@");

    unsafe {
        objc_registerClassPair(app_del_class);
    }
}