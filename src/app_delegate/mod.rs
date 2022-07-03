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
        // we +alloc and -initWithFrame: our window here, so that we can have it show on screen (eventually).
        // this entire method is the objc-runtime based version of the standard View-Based application's launch code, so nothing here really should surprise you.
        // one thing important to note, though is that we use `sel_getUid()` instead of @selector().
        // this is because @selector is an objc language construct, and the application would not have been created in C if I used @selector.
        obj.window = objc_msgSend(rust_objc_get_class("UIWindow") as Id, rust_sel_get_uid("alloc"));
        obj.window = objc_msgSend(obj.window, rust_sel_get_uid("initWithFrame:"), CGRect::new(0.0, 0.0, 320.0, 480.0));

        // here, we are creating our view controller, and our view. note the use of objc_getClass, because we cannot reference UIViewController directly in C.
        let view_controller = objc_msgSend(objc_msgSend(rust_objc_get_class("UIViewController") as Id, rust_sel_get_uid("alloc")), rust_sel_get_uid("init"));

        // creating our custom view class, there really isn't too much 
        // to say here other than we are hard-coding the screen's bounds, 
        // because returning a struct from a `objc_msgSend()` (via 
        // [[UIScreen mainScreen] bounds]) requires a different function call
        // and is finicky at best.
        let view = objc_msgSend(objc_msgSend(rust_objc_get_class("View") as Id, rust_sel_get_uid("alloc")), rust_sel_get_uid("initWithFrame:"), CGRect::new(0.0, 0.0, 320.0, 480.0));

        // here we simply add the view to the view controller, and add the view_controller to the window.
        objc_msgSend(objc_msgSend(view_controller, rust_sel_get_uid("view")), rust_sel_get_uid("addSubview:"), view);
        objc_msgSend(obj.window, rust_sel_get_uid("setRootViewController:"), view_controller);

        // finally, we display the window on-screen.
        objc_msgSend(obj.window, rust_sel_get_uid("makeKeyAndVisible"));
    }
    
    return Bool::Yes;
}

#[ctor]
fn init_app_del() {
    // This is objc-runtime gibberish at best. We are creating a class with the 
    // name "AppDelegate" that is a subclass of "UIResponder". Note we do not need
    // to register for the UIApplicationDelegate protocol, that really is simply for 
    // Xcode's autocomplete, we just need to implement the method and we are golden.
    let app_del_class = rust_objc_allocate_class_pair(rust_objc_get_class("UIResponder") as Id, "AppDelegate", 0);

    // Here, we tell the objc runtime that we have a variable named "window" of type 'id'
    rust_class_add_ivar(app_del_class, "window", mem::size_of::<Id>(), 0, "@");

    // We tell the objc-runtime that we have an implementation for the method
    // -application:didFinishLaunchingWithOptions:, and link that to our custom 
    // function defined above. Notice the final parameter. This tells the runtime
    // the types of arguments received by the function.
    let cast_fn: Imp = unsafe {
        *(&app_del_did_finish_launching as *const _ as usize as *const Imp)
    };
    rust_class_add_method(app_del_class, rust_sel_get_uid("application:didFinishLaunchingWithOptions:"), cast_fn, "i@:@@");

    // Finally we tell the runtime that we have finished describing the class and 
    // we can let the rest of the application use it.
    unsafe {
        objc_registerClassPair(app_del_class);
    }
}