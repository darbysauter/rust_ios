use ctor::*;
use crate::objc::*;
use std::mem;

#[repr(C)]
pub struct AppDel {
    pub isa: Class,
    pub window: Id,
}

extern "C" fn app_del_did_finish_launching(obj: &mut AppDel, _cmd: usize, _application: usize, _options: usize) -> Bool {
    unsafe {
        obj.window = objc_msgSend(rust_objc_get_class("UIWindow") as Id, rust_sel_get_uid("alloc"));
        obj.window = objc_msgSend(obj.window, rust_sel_get_uid("initWithFrame:"), CGRect::new(0.0, 0.0, 320.0, 480.0));

        let view_controller = objc_msgSend(objc_msgSend(rust_objc_get_class("UIViewController") as Id, rust_sel_get_uid("alloc")), rust_sel_get_uid("init"));

        let view = objc_msgSend(objc_msgSend(rust_objc_get_class("View") as Id, rust_sel_get_uid("alloc")), rust_sel_get_uid("initWithFrame:"), CGRect::new(0.0, 0.0, 320.0, 480.0));

        objc_msgSend(objc_msgSend(view_controller, rust_sel_get_uid("view")), rust_sel_get_uid("addSubview:"), view);
        objc_msgSend(obj.window, rust_sel_get_uid("setRootViewController:"), view_controller);

        objc_msgSend(obj.window, rust_sel_get_uid("makeKeyAndVisible"));
    }
    
    return Bool::Yes;
}

#[ctor]
fn init_app_del() {
    let app_del_class = rust_objc_allocate_class_pair(rust_objc_get_class("UIResponder"), "AppDelegate", 0);

    rust_class_add_ivar(app_del_class, "window", mem::size_of::<Id>(), 0, "@");

    let cast_fn: Imp = unsafe {
        *(&app_del_did_finish_launching as *const _ as usize as *const Imp)
    };
    rust_class_add_method(app_del_class, rust_sel_get_uid("application:didFinishLaunchingWithOptions:"), cast_fn, "i@:@@");

    unsafe {
        objc_registerClassPair(app_del_class);
    }
}