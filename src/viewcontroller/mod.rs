use crate::objc::*;

extern "C" fn my_viewcontroller_init(obj: Id, _cmd: Sel) -> Id {
    unsafe {
        let sup = class_getSuperclass(object_getClass(obj));

        let sup_info = ObjcSuper::new(obj, sup);

        // [super initWithNibName:nil bundle:nil]
        objc_msgSendSuper(&sup_info, rust_sel_get_uid("initWithNibName:bundle:"), 0, 0)
    }
}

extern "C" fn my_viewcontroller_load_view(obj: Id, _cmd: Sel) {
    unsafe {
        let sup = class_getSuperclass(object_getClass(obj));

        let sup_info = ObjcSuper::new(obj, sup);

        // [super initWithNibName:nil bundle:nil]
        objc_msgSendSuper(&sup_info, rust_sel_get_uid("loadView"));
    
        // self.view.backgroundColor = [UIColor colorWithHue:0.0 saturation:0.0 brightness:0.2 alpha:1.0];
        let color = objc_msgSend(rust_objc_get_class("UIColor"), rust_sel_get_uid("colorWithHue:saturation:brightness:alpha:"), 0.0f64, 0.0f64, 0.2f64, 1.0f64);
        objc_msgSend(objc_msgSend(obj, rust_sel_get_uid("view")), rust_sel_get_uid("setBackgroundColor:"), color);
    }
}

pub fn init_my_viewcontroller() {
    let my_viewcontroller_class = rust_objc_allocate_class_pair(rust_objc_get_class("UIViewController"), "MyViewController", 0);

    let my_vc_init: Imp = unsafe {
        *(&my_viewcontroller_init as *const _ as usize as *const Imp)
    };
    rust_class_add_method(my_viewcontroller_class, rust_sel_get_uid("init"), my_vc_init, "@@:");
    let my_vc_load_view: Imp = unsafe {
        *(&my_viewcontroller_load_view as *const _ as usize as *const Imp)
    };
    rust_class_add_method(my_viewcontroller_class, rust_sel_get_uid("loadView"), my_vc_load_view, "v@:");

    unsafe {
        objc_registerClassPair(my_viewcontroller_class);
    }
}