use crate::objc::*;
use crate::run_payload::{ run_on_app_start, run_on_button_press };
use cstr::cstr;
use std::thread;

extern "C" fn my_viewcontroller_init(obj: *mut Id, _cmd: Sel) -> Id {
    unsafe {
        let sup = class_getSuperclass(object_getClass(obj));

        let sup_info = ObjcSuper::new(obj, sup);

        // [super initWithNibName:nil bundle:nil]
        let ret = rust_msg_send_super_2(&sup_info, sel_getUid(cstr!("initWithNibName:bundle:").as_ptr()), 0, 0);
        ret
    }
}

const UIBUTTON_TYPE_SYSTEM: i64 = 1;
const NSLAYOUT_ATTRIBUTE_CENTER_X: i64 = 9;
const NSLAYOUT_ATTRIBUTE_CENTER_Y: i64 = 10;
const NSLAYOUT_RELATION_EQUAL: i64 = 0;
const UICONTROL_STATE_NORMAL: u64 = 0;
const UICONTROL_STATE_HIGHLIGHTED: u64 = 1 << 0;
const UICONTROL_EVENT_TOUCH_UP_INSIDE: u64 = 1 << 6;

static mut BUTTON_REF: usize = 0;

extern "C" fn my_viewcontroller_load_view(obj: *mut Id, _cmd: Sel) {
    unsafe {
        let sup = class_getSuperclass(object_getClass(obj));

        let sup_info = ObjcSuper::new(obj, sup);

        // [super initWithNibName:nil bundle:nil]
        rust_msg_send_super::<()>(&sup_info, sel_getUid(cstr!("loadView").as_ptr()));

        let self_view: *mut Id = rust_msg_send(obj, sel_getUid(cstr!("view").as_ptr()));
    
        // self.view.backgroundColor = [UIColor colorWithHue:0.0 saturation:0.0 brightness:0.2 alpha:1.0];
        let color: *mut Id = rust_msg_send_4(objc_getClass(cstr!("UIColor").as_ptr()), sel_getUid(cstr!("colorWithHue:saturation:brightness:alpha:").as_ptr()), 0.0f64, 0.0f64, 0.2f64, 1.0f64);
        rust_msg_send_1::<(), *mut Id>(self_view, sel_getUid(cstr!("setBackgroundColor:").as_ptr()), color);

        // button = [UIButton buttonWithType:UIButtonTypeSystem];
        let button: *mut Id = rust_msg_send_1(objc_getClass(cstr!("UIButton").as_ptr()), sel_getUid(cstr!("buttonWithType:").as_ptr()), UIBUTTON_TYPE_SYSTEM);
        BUTTON_REF = button as usize;
        // button.translatesAutoresizingMaskIntoConstraints = NO;
        let nsstring: *mut Id = rust_msg_send_1(rust_msg_send(objc_getClass(cstr!("NSString").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), sel_getUid(cstr!("initWithUTF8String:").as_ptr()), cstr!("translatesAutoresizingMaskIntoConstraints").as_ptr());
        let nsnumber_bool: *mut Id = rust_msg_send_1(objc_getClass(cstr!("NSNumber").as_ptr()), sel_getUid(cstr!("numberWithBool:").as_ptr()), NO);
        rust_msg_send_2::<(), *mut Id, *mut Id>(button, sel_getUid(cstr!("setValue:forKey:").as_ptr()), nsnumber_bool, nsstring);

        // [button setTitle:@"Run" forState:UIControlStateNormal];
        let nsstring: *mut Id = rust_msg_send_1(rust_msg_send(objc_getClass(cstr!("NSString").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), sel_getUid(cstr!("initWithUTF8String:").as_ptr()), cstr!("Run").as_ptr());
        rust_msg_send_2::<(), *mut Id, u64>(button, sel_getUid(cstr!("setTitle:forState:").as_ptr()), nsstring, UICONTROL_STATE_NORMAL);

        // [button setTitleColor:[UIColor colorWithHue:0.0 saturation:0.0 brightness:1.0 alpha:1.0] forState:UIControlStateNormal];
        let color1: *mut Id = rust_msg_send_4(objc_getClass(cstr!("UIColor").as_ptr()), sel_getUid(cstr!("colorWithHue:saturation:brightness:alpha:").as_ptr()), 0.0f64, 0.0f64, 1.0f64, 1.0f64);
        rust_msg_send_2::<(), *mut Id, u64>(button, sel_getUid(cstr!("setTitleColor:forState:").as_ptr()), color1, UICONTROL_STATE_NORMAL);

        // [button setTitleColor:[UIColor colorWithHue:0.0 saturation:0.0 brightness:0.7 alpha:1.0] forState:UIControlStateHighlighted];
        let color2: *mut Id = rust_msg_send_4(objc_getClass(cstr!("UIColor").as_ptr()), sel_getUid(cstr!("colorWithHue:saturation:brightness:alpha:").as_ptr()), 0.0f64, 0.0f64, 0.7f64, 1.0f64);
        rust_msg_send_2::<(), *mut Id, u64>(button, sel_getUid(cstr!("setTitleColor:forState:").as_ptr()), color2, UICONTROL_STATE_HIGHLIGHTED);

        // [button setBackgroundColor:[UIColor colorWithRed:0.00 green:0.00 blue:1.00 alpha:1.0]];
        let color3: *mut Id = rust_msg_send_4(objc_getClass(cstr!("UIColor").as_ptr()), sel_getUid(cstr!("colorWithRed:green:blue:alpha:").as_ptr()), 0.0f64, 0.0f64, 1.0f64, 1.0f64);
        rust_msg_send_1::<(), *mut Id>(button, sel_getUid(cstr!("setBackgroundColor:").as_ptr()), color3);

        // button.titleEdgeInsets = UIEdgeInsetsMake(0, 0, 0, 0);
        let edge_insets = UIEdgeInsets::new(0.0, 0.0, 0.0, 0.0);
        let edge_insets: *mut Id = rust_msg_send_1(objc_getClass(cstr!("NSValue").as_ptr()), sel_getUid(cstr!("valueWithEdgeInsets:").as_ptr()), edge_insets);
        let nsstring: *mut Id = rust_msg_send_1(rust_msg_send(objc_getClass(cstr!("NSString").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), sel_getUid(cstr!("initWithUTF8String:").as_ptr()), cstr!("titleEdgeInsets").as_ptr());
        rust_msg_send_2::<(), *mut Id, *mut Id>(button, sel_getUid(cstr!("setValue:forKey:").as_ptr()), edge_insets, nsstring);

        // button.titleLabel.font = [UIFont systemFontOfSize:30];
        let nsstring: *mut Id = rust_msg_send_1(rust_msg_send(objc_getClass(cstr!("NSString").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), sel_getUid(cstr!("initWithUTF8String:").as_ptr()), cstr!("titleLabel").as_ptr());
        let title_label: *mut Id = rust_msg_send_1(button, sel_getUid(cstr!("valueForKey:").as_ptr()), nsstring);
        let nsstring: *mut Id = rust_msg_send_1(rust_msg_send(objc_getClass(cstr!("NSString").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), sel_getUid(cstr!("initWithUTF8String:").as_ptr()), cstr!("font").as_ptr());
        let font: *mut Id = rust_msg_send_1(objc_getClass(cstr!("UIFont").as_ptr()), sel_getUid(cstr!("systemFontOfSize:").as_ptr()), 30.0f64);
        rust_msg_send_2::<(), *mut Id, *mut Id>(title_label, sel_getUid(cstr!("setValue:forKey:").as_ptr()), font, nsstring);

        // [button addTarget:self action:@selector(actionJailbreak) forControlEvents:UIControlEventTouchUpInside];
        rust_msg_send_3::<(), *mut Id, Sel, u64>(button, sel_getUid(cstr!("addTarget:action:forControlEvents:").as_ptr()), obj, sel_getUid(cstr!("buttonTapped").as_ptr()), UICONTROL_EVENT_TOUCH_UP_INSIDE);
        
        // [self.view addSubview:button];
        rust_msg_send_1::<(), *mut Id>(self_view, sel_getUid(cstr!("addSubview:").as_ptr()), button);
        // [NSLayoutConstraint constraintWithItem:button attribute:NSLayoutAttributeCenterX relatedBy:NSLayoutRelationEqual toItem:self.view attribute:NSLayoutAttributeCenterX multiplier:1.0 constant:0.0]
        let constraint1: *mut Id = rust_msg_send_7(objc_getClass(cstr!("NSLayoutConstraint").as_ptr()), sel_getUid(cstr!("constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:").as_ptr()),
            button, NSLAYOUT_ATTRIBUTE_CENTER_X, NSLAYOUT_RELATION_EQUAL, self_view, NSLAYOUT_ATTRIBUTE_CENTER_X, 1.0f64, 0.0f64);

        // [NSLayoutConstraint constraintWithItem:button attribute:NSLayoutAttributeCenterY relatedBy:NSLayoutRelationEqual toItem:self.view attribute:NSLayoutAttributeCenterY multiplier:1.1 constant:0.0]
        let constraint2: *mut Id = rust_msg_send_7(objc_getClass(cstr!("NSLayoutConstraint").as_ptr()), sel_getUid(cstr!("constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:").as_ptr()),
            button, NSLAYOUT_ATTRIBUTE_CENTER_Y, NSLAYOUT_RELATION_EQUAL, self_view, NSLAYOUT_ATTRIBUTE_CENTER_Y, 1.1f64, 0.0f64);
        // [self.view addConstraint:constraint1];
        rust_msg_send_1::<(), *mut Id>(self_view, sel_getUid(cstr!("addConstraint:").as_ptr()), constraint1);
        // [self.view addConstraint:constraint2];
        rust_msg_send_1::<(), *mut Id>(self_view, sel_getUid(cstr!("addConstraint:").as_ptr()), constraint2);
    }

    thread::spawn(|| {
        run_on_app_start();
    });
}


fn my_viewcontroller_disable_button() {
    unsafe {
        let button = BUTTON_REF as *mut Id;
        rust_msg_send_1::<(), Bool>(button, sel_getUid(cstr!("setEnabled:").as_ptr()), NO);

        let color3: *mut Id = rust_msg_send_4(objc_getClass(cstr!("UIColor").as_ptr()), sel_getUid(cstr!("colorWithRed:green:blue:alpha:").as_ptr()), 0.0f64, 0.0f64, 0.0f64, 0.1f64);
        rust_msg_send_1::<(), *mut Id>(button, sel_getUid(cstr!("setBackgroundColor:").as_ptr()), color3);
    }
}

extern "C" fn my_viewcontroller_enable_button(_obj: *mut Id, _cmd: Sel) {
    unsafe {
        let button = BUTTON_REF as *mut Id;
        rust_msg_send_1::<(), Bool>(button, sel_getUid(cstr!("setEnabled:").as_ptr()), YES);

        let color3: *mut Id = rust_msg_send_4(objc_getClass(cstr!("UIColor").as_ptr()), sel_getUid(cstr!("colorWithRed:green:blue:alpha:").as_ptr()), 0.0f64, 0.0f64, 1.0f64, 1.0f64);
        rust_msg_send_1::<(), *mut Id>(button, sel_getUid(cstr!("setBackgroundColor:").as_ptr()), color3);
    }
}

extern "C" fn my_viewcontroller_button_tapped(obj: *mut Id, _cmd: Sel) {
    my_viewcontroller_disable_button();
    let obj = obj as usize;
    thread::spawn(move || {
        run_on_button_press();
        unsafe {
            let obj = obj as *mut Id;
            rust_msg_send_3::<(), Sel, *mut Id, Bool>(obj, sel_getUid(cstr!("performSelectorOnMainThread:withObject:waitUntilDone:").as_ptr()), sel_getUid(cstr!("enableButton").as_ptr()), obj, NO);
        }
    });
}

pub fn init_my_viewcontroller() {
    unsafe {
        let my_viewcontroller_class = objc_allocateClassPair(objc_getClass(cstr!("UIViewController").as_ptr()), cstr!("MyViewController").as_ptr(), 0);

        let my_vc_init: usize = my_viewcontroller_init as *const () as usize;
        class_addMethod(my_viewcontroller_class, sel_getUid(cstr!("init").as_ptr()), my_vc_init, cstr!("@@:").as_ptr());
        let my_vc_load_view: usize = my_viewcontroller_load_view as *const () as usize;
        class_addMethod(my_viewcontroller_class, sel_getUid(cstr!("loadView").as_ptr()), my_vc_load_view, cstr!("v@:").as_ptr());
        let my_vc_button_tapped: usize = my_viewcontroller_button_tapped as *const () as usize;
        class_addMethod(my_viewcontroller_class, sel_getUid(cstr!("buttonTapped").as_ptr()), my_vc_button_tapped, cstr!("v@:").as_ptr());
        let my_vc_enable_button: usize = my_viewcontroller_enable_button as *const () as usize;
        class_addMethod(my_viewcontroller_class, sel_getUid(cstr!("enableButton").as_ptr()), my_vc_enable_button, cstr!("v@:").as_ptr());

        objc_registerClassPair(my_viewcontroller_class);
    }
}