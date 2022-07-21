use std::os::raw::{ c_char, c_int };
use std::mem;
use std::ffi::CString;
use cstr::cstr;

use core::fmt;

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
pub struct Property(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CGContextRef(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct NSString(usize);

pub type Bool = bool;
pub const YES: Bool = true;
pub const NO: Bool = false;

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
pub struct UIEdgeInsets {
    pub top: f64,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
}

impl UIEdgeInsets {
    pub fn new(top: f64, left: f64, bottom: f64, right: f64) -> UIEdgeInsets {
        UIEdgeInsets { top, left, bottom, right }
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
    pub fn NSStringFromClass(aClass: *mut Id) -> *const NSString;
    pub fn object_setIvar(obj: *mut Id, ivar: Ivar, value: *mut Id);
    pub fn class_getSuperclass(cls: *mut Id) -> *mut Id;
    pub fn object_getClass(obj: *mut Id) -> *mut Id;
    pub fn objc_msgSendSuper(sup: &ObjcSuper, sel: Sel, args: ...) -> *mut Id;
    
    pub fn objc_msgSend(obj: *mut Id, sel: Sel, args: ...) -> *mut Id;
    
    pub fn class_getInstanceVariable(cls: *mut Id, name: *const c_char) -> Ivar;
    pub fn class_getProperty(cls: *mut Id, name: *const c_char) -> Ivar;
    pub fn sel_registerName(c: *const c_char) -> Sel;
    pub fn sel_getUid(c: *const c_char) -> Sel;
    pub fn objc_getClass(c: *const c_char) -> *mut Id;
    pub fn __CFStringMakeConstantString(c: *const c_char) -> usize;
    pub fn objc_allocateClassPair(superclass: *mut Id, name: *const c_char, extraBytes: usize) -> *mut Id;
    pub fn class_addIvar(cls: *mut Id, name: *const c_char, size: usize, alignment: u8, types: *const c_char) -> Bool;
    pub fn class_addMethod(cls: *mut Id, name: Sel, imp: usize, types: *const c_char) -> Bool;
    pub fn NSLog(string: *mut Id);
}

#[macro_export]
macro_rules! nslog {
    ($($arg:tt)*) => ($crate::objc::_rust_log(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! nslogln {
    () => ($crate::nslog!("\n"));
    ($($arg:tt)*) => ($crate::nslog!("{}\n", format_args!($($arg)*)));
}

pub fn _rust_log(s: fmt::Arguments) {
    unsafe {
        let s = format!("LOG: {}", s);
        let c_str = CString::new(s).unwrap();
        let nsstring: *mut Id = rust_msg_send_1(rust_msg_send(objc_getClass(cstr!("NSString").as_ptr()), sel_getUid(cstr!("alloc").as_ptr())), sel_getUid(cstr!("initWithUTF8String:").as_ptr()), c_str.as_ptr());
        NSLog(nsstring);
    }
}

pub unsafe extern "C" fn rust_msg_send<R>(a: *mut Id, b: Sel) -> R {
    let func = msg_send_fn::<R>();
    func(a, b)
}

fn msg_send_fn<R>() -> unsafe extern "C" fn(*mut Id, Sel) -> R {
    unsafe { mem::transmute(objc_msgSend as *const extern "C" fn(*mut Id, Sel) -> R) }
}

pub unsafe extern "C" fn rust_msg_send_1<R, T>(a: *mut Id, b: Sel, c: T) -> R {
    let func = msg_send_fn_1::<R, T>();
    func(a, b, c)
}

fn msg_send_fn_1<R, T>() -> unsafe extern "C" fn(*mut Id, Sel, T) -> R {
    unsafe { mem::transmute(objc_msgSend as *const extern "C" fn(*mut Id, Sel, T) -> R) }
}

pub unsafe extern "C" fn rust_msg_send_2<R, T, U>(a: *mut Id, b: Sel, c: T, d: U) -> R {
    let func = msg_send_fn_2::<R, T, U>();
    func(a, b, c, d)
}

fn msg_send_fn_2<R, T, U>() -> unsafe extern "C" fn(*mut Id, Sel, T, U) -> R {
    unsafe { mem::transmute(objc_msgSend as *const extern "C" fn(*mut Id, Sel, T, U) -> R) }
}

pub unsafe extern "C" fn rust_msg_send_3<R, T, U, V>(a: *mut Id, b: Sel, c: T, d: U, e: V) -> R {
    let func = msg_send_fn_3::<R, T, U, V>();
    func(a, b, c, d, e)
}

fn msg_send_fn_3<R, T, U, V>() -> unsafe extern "C" fn(*mut Id, Sel, T, U, V) -> R {
    unsafe { mem::transmute(objc_msgSend as *const extern "C" fn(*mut Id, Sel, T, U, V) -> R) }
}

pub unsafe extern "C" fn rust_msg_send_4<R, T, U, V, W>(a: *mut Id, b: Sel, c: T, d: U, e: V, f: W) -> R {
    let func = msg_send_fn_4::<R, T, U, V, W>();
    func(a, b, c, d, e, f)
}

fn msg_send_fn_4<R, T, U, V, W>() -> unsafe extern "C" fn(*mut Id, Sel, T, U, V, W) -> R {
    unsafe { mem::transmute(objc_msgSend as *const extern "C" fn(*mut Id, Sel, T, U, V, W) -> R) }
}

pub unsafe extern "C" fn rust_msg_send_7<R, T, U, V, W, X, Y, Z>(a: *mut Id, b: Sel, c: T, d: U, e: V, f: W, g: X, h: Y, i: Z) -> R {
    let func = msg_send_fn_7::<R, T, U, V, W, X, Y, Z>();
    func(a, b, c, d, e, f, g, h, i)
}

fn msg_send_fn_7<R, T, U, V, W, X, Y, Z>() -> unsafe extern "C" fn(*mut Id, Sel, T, U, V, W, X, Y, Z) -> R {
    unsafe { mem::transmute(objc_msgSend as *const extern "C" fn(*mut Id, Sel, T, U, V, W, X, Y, Z) -> R) }
}

pub unsafe extern "C" fn rust_msg_send_super<R>(a: &ObjcSuper, b: Sel) -> R {
    let func = msg_send_super_fn::<R>();
    func(a, b)
}

fn msg_send_super_fn<R>() -> unsafe extern "C" fn(&ObjcSuper, Sel) -> R {
    unsafe { mem::transmute(objc_msgSendSuper as *const extern "C" fn(&ObjcSuper, Sel) -> R) }
}

pub unsafe extern "C" fn rust_msg_send_super_2<R, T, U>(a: &ObjcSuper, b: Sel, c: T, d: U) -> R {
    let func = msg_send_super_fn_2::<R, T, U>();
    func(a, b, c, d)
}

fn msg_send_super_fn_2<R, T, U>() -> unsafe extern "C" fn(&ObjcSuper, Sel, T, U) -> R {
    unsafe { mem::transmute(objc_msgSendSuper as *const extern "C" fn(&ObjcSuper, Sel, T, U) -> R) }
}