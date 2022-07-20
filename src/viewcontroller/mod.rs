use crate::objc::*;
use cstr::cstr;
use std::ffi::CStr;

extern "C" fn my_viewcontroller_init(obj: *mut Id, _cmd: Sel) -> Id {
    unsafe {
        let sup = class_getSuperclass(object_getClass(obj));

        let sup_info = ObjcSuper::new(obj, sup);

        // [super initWithNibName:nil bundle:nil]
        let ret = rust_msg_send_super_2(&sup_info, sel_getUid(cstr!("initWithNibName:bundle:").as_ptr()), 0, 0);
        ret
    }
}

extern "C" fn my_viewcontroller_load_view(obj: *mut Id, _cmd: Sel) {
    unsafe {
        let sup = class_getSuperclass(object_getClass(obj));

        let sup_info = ObjcSuper::new(obj, sup);

        // [super initWithNibName:nil bundle:nil]
        rust_msg_send_super::<()>(&sup_info, sel_getUid(cstr!("loadView").as_ptr()));
    
        // self.view.backgroundColor = [UIColor colorWithHue:0.0 saturation:0.0 brightness:0.2 alpha:1.0];
        let color: *mut Id = rust_msg_send_4(objc_getClass(cstr!("UIColor").as_ptr()), sel_getUid(cstr!("colorWithHue:saturation:brightness:alpha:").as_ptr()), 0.0f64, 0.0f64, 0.2f64, 1.0f64);
        rust_msg_send_1::<(), *mut Id>(rust_msg_send(obj, sel_getUid(cstr!("view").as_ptr())), sel_getUid(cstr!("setBackgroundColor:").as_ptr()), color);
    }
}

pub fn init_my_viewcontroller() {
    unsafe {
        let my_viewcontroller_class = objc_allocateClassPair(objc_getClass(cstr!("UIViewController").as_ptr()), cstr!("MyViewController").as_ptr(), 0);

        let my_vc_init: usize = my_viewcontroller_init as *const () as usize;
        class_addMethod(my_viewcontroller_class, sel_getUid(cstr!("init").as_ptr()), my_vc_init, cstr!("@@:").as_ptr());
        let my_vc_load_view: usize = my_viewcontroller_load_view as *const () as usize;
        class_addMethod(my_viewcontroller_class, sel_getUid(cstr!("loadView").as_ptr()), my_vc_load_view, cstr!("v@:").as_ptr());

        objc_registerClassPair(my_viewcontroller_class);
    }
}