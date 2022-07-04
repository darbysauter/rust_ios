use std::ffi::CString;
use std::os::raw::{ c_char, c_int };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Class(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Id(usize);

impl Id {
    pub fn to_class(self) -> Class {
        Class { 0: self.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Sel(usize);

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

pub type Imp = extern "C" fn(Id, Sel, ...);

#[repr(C)]
pub struct CGSize {
    width: f64,
    height: f64,
}

impl CGSize {
    pub fn new(width: f64, height: f64) -> CGSize {
        CGSize { width, height, }
    }
}

#[repr(C)]
pub struct CGPoint {
    x: f64,
    y: f64,
}

impl CGPoint {
    pub fn new(x: f64, y: f64) -> CGPoint {
        CGPoint { x, y, }
    }
}

#[repr(C)]
pub struct CGRect {
    origin: CGPoint,
    size: CGSize,
}

impl CGRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> CGRect {
        CGRect {    origin: CGPoint::new(x, y),
                    size: CGSize::new(width, height), }
    }
}

#[link(name = "Foundation", kind = "framework")]
#[link(name = "UIKit", kind = "framework")]
extern "C" {
    pub fn UIApplicationMain(argc: c_int, argv: *const *const c_char, principalClassName: *const NSString, delegateClassName: *const NSString) -> c_int;
    pub fn objc_msgSend(obj: Id, sel: Sel, args: ...) -> Id;
    pub fn objc_registerClassPair(cls: Class);
    pub fn UIGraphicsGetCurrentContext() -> CGContextRef;
    pub fn CGContextSetFillColor(c: CGContextRef, components: *const f64);
    pub fn CGContextAddRect(c: CGContextRef, rect: CGRect);
    pub fn CGContextFillPath(c: CGContextRef);
    pub fn NSStringFromClass(aClass: Class) -> *const NSString;

    fn sel_registerName(c: *const c_char) -> Sel;
    fn sel_getUid(c: *const c_char) -> Sel;
    fn objc_getClass(c: *const c_char) -> Id;
    fn __CFStringMakeConstantString(c: *const c_char) -> usize;
    fn objc_allocateClassPair(superclass: Id, name: *const c_char, extraBytes: usize) -> Class;
    fn class_addIvar(cls: Class, name: *const c_char, size: usize, alignment: u8, types: *const c_char) -> Bool;
    fn class_addMethod(cls: Class, name: Sel, imp: Imp, types: *const c_char) -> Bool;
}

pub fn rust_sel_register_name(name: &str) -> Sel {
    let c_string = CString::new(name).expect("CString::new failed");
    let sel = unsafe {
        sel_registerName(c_string.as_ptr())
    };
    sel
}

pub fn rust_sel_get_uid(name: &str) -> Sel {
    let c_string = CString::new(name).expect("CString::new failed");
    let uid = unsafe {
        sel_getUid(c_string.as_ptr())
    };
    uid
}

pub fn rust_objc_get_class(name: &str) -> Id {
    let c_string = CString::new(name).expect("CString::new failed");
    let class = unsafe {
        objc_getClass(c_string.as_ptr())
    };
    class
}

pub fn rust_cfstr(s: &str) -> usize {
    // Notice the use of CFSTR here. We cannot use an objective-c string 
    // literal @"someStr", as that would be using objective-c, obviously.
    let string = CString::new(s).expect("CString::new failed");
    unsafe {
        __CFStringMakeConstantString(string.as_ptr())
    }
}

pub fn rust_objc_allocate_class_pair(superclass: Id, name: &str, extra_bytes: usize) -> Class {
    let string = CString::new(name).expect("CString::new failed");
    unsafe {
        objc_allocateClassPair(superclass, string.as_ptr(), extra_bytes)
    }
}

pub fn rust_class_add_ivar(cls: Class, name: &str, size: usize, alignment: u8, types: &str) -> Bool {
    let name = CString::new(name).expect("CString::new failed");
    let types = CString::new(types).expect("CString::new failed");
    unsafe {
        class_addIvar(cls, name.as_ptr(), size, alignment, types.as_ptr())
    }
}

pub fn rust_class_add_method(cls: Class, name: Sel, imp: Imp, types: &str) -> Bool {
    let types = CString::new(types).expect("CString::new failed");
    unsafe {
        class_addMethod(cls, name, imp, types.as_ptr())
    }
}