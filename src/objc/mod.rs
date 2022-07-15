use std::ffi::CString;
use std::os::raw::{ c_char, c_int };
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Class(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Id(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Sel(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Ivar(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CGContextRef(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NSString(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Bool {
    No = 0,
    Yes = 1,
}

pub type Imp = extern "C" fn(*mut Id, Sel, ...);
pub type MsgSendRetCGRect = extern "C" fn(*mut Id, Sel, ...) -> CGRect;

#[repr(C)]
pub struct CGSize {
    pub width: f64,
    pub height: f64,
}

impl CGSize {
    pub fn new(width: f64, height: f64) -> CGSize {
        CGSize { width, height, }
    }
}

#[repr(C)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

impl CGPoint {
    pub fn new(x: f64, y: f64) -> CGPoint {
        CGPoint { x, y, }
    }
}

#[repr(C)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> CGRect {
        CGRect {    origin: CGPoint::new(x, y),
                    size: CGSize::new(width, height), }
    }
}

#[repr(C)]
pub struct ObjcSuper {
    receiver: *mut Id,
    super_class: *mut Id,
}

impl ObjcSuper {
    pub fn new(receiver: *mut Id, super_class: *mut Id) -> ObjcSuper {
        ObjcSuper { receiver, super_class }
    }
}

extern "C" {
    pub fn UIApplicationMain(argc: c_int, argv: *const *const c_char, principalClassName: *const NSString, delegateClassName: *const NSString) -> c_int;
    pub fn objc_registerClassPair(cls: *mut Id);
    pub fn UIGraphicsGetCurrentContext() -> CGContextRef;
    pub fn CGContextSetFillColor(c: CGContextRef, components: *const f64);
    pub fn CGContextAddRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextFillPath(c: CGContextRef);
    pub fn NSStringFromClass(aClass: Class) -> *const NSString;
    pub fn object_setIvar(obj: *mut Id, ivar: Ivar, value: *mut Id);
    pub fn class_getSuperclass(cls: *mut Id) -> *mut Id;
    pub fn object_getClass(obj: *mut Id) -> *mut Id;
    pub fn objc_msgSendSuper(sup: &ObjcSuper, sel: Sel, args: ...) -> *mut Id;
    
    pub fn objc_msgSend(obj: *mut Id, sel: Sel, args: ...) -> *mut Id;
    
    pub fn class_getInstanceVariable(cls: *mut Id, name: *const c_char) -> Ivar;
    pub fn sel_registerName(c: *const c_char) -> Sel;
    pub fn sel_getUid(c: *const c_char) -> Sel;
    pub fn objc_getClass(c: *const c_char) -> *mut Id;
    pub fn __CFStringMakeConstantString(c: *const c_char) -> usize;
    pub fn objc_allocateClassPair(superclass: *mut Id, name: *const c_char, extraBytes: usize) -> *mut Id;
    pub fn class_addIvar(cls: *mut Id, name: *const c_char, size: usize, alignment: u8, types: *const c_char) -> Bool;
    pub fn class_addMethod(cls: *mut Id, name: Sel, imp: usize, types: *const c_char) -> Bool;
}

// pub unsafe extern "C" fn rust_objc_msg_send_ret_cgrect(obj: Id, sel: Sel, args: ...) -> CGRect {
//     let cast_fn: MsgSendRetCGRect = *(&objc_msgSend as *const _ as usize as *const MsgSendRetCGRect);
//     cast_fn(obj, sel, args)
// }

pub unsafe extern "C" fn rust_msg_send<R>(a: *mut Id, b: Sel, c: ...) -> R {
    let func = msg_send_fn();
    func(a, b, c)
}

fn msg_send_fn<R>() -> unsafe extern fn(*mut Id, Sel, ...) -> R {
    unsafe { mem::transmute(objc_msgSend as unsafe extern "C" fn(_, _, ...) -> _) }
}

pub unsafe extern "C" fn rust_msg_send_super<R>(a: &ObjcSuper, b: Sel, c: ...) -> R {
    let func = msg_send_super_fn();
    func(a, b, c)
}

fn msg_send_super_fn<R>() -> unsafe extern fn(&ObjcSuper, Sel, ...) -> R {
    unsafe { mem::transmute(objc_msgSendSuper as unsafe extern "C" fn(_, _, ...) -> _) }
}