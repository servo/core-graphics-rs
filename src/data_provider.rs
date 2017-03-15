// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};

use libc::{c_void, size_t, off_t};
use std::mem;
use std::ptr;
use std::slice;
use std::rc::Rc;
use std::sync::Arc;

pub type CGDataProviderGetBytesCallback = Option<unsafe extern fn (*mut c_void, *mut c_void, size_t) -> size_t>;
pub type CGDataProviderReleaseInfoCallback = Option<unsafe extern fn (*mut c_void)>;
pub type CGDataProviderRewindCallback = Option<unsafe extern fn (*mut c_void)>;
pub type CGDataProviderSkipBytesCallback = Option<unsafe extern fn (*mut c_void, size_t)>;
pub type CGDataProviderSkipForwardCallback = Option<unsafe extern fn (*mut c_void, off_t) -> off_t>;

pub type CGDataProviderGetBytePointerCallback = Option<unsafe extern fn (*mut c_void) -> *mut c_void>;
pub type CGDataProviderGetBytesAtOffsetCallback = Option<unsafe extern fn (*mut c_void, *mut c_void, size_t, size_t)>;
pub type CGDataProviderReleaseBytePointerCallback = Option<unsafe extern fn (*mut c_void, *const c_void)>;
pub type CGDataProviderReleaseDataCallback = Option<unsafe extern fn (*mut c_void, *const c_void, size_t)>;
pub type CGDataProviderGetBytesAtPositionCallback = Option<unsafe extern fn (*mut c_void, *mut c_void, off_t, size_t)>;

#[repr(C)]
pub struct __CGDataProvider;

pub type CGDataProviderRef = *const __CGDataProvider;

pub struct CGDataProvider {
    obj: CGDataProviderRef,
}

impl Drop for CGDataProvider {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl TCFType<CGDataProviderRef> for CGDataProvider {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CGDataProviderRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CGDataProviderRef) -> CGDataProvider {
        let reference: CGDataProviderRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CGDataProviderRef) -> CGDataProvider {
        CGDataProvider {
            obj: obj,
        }
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe {
            CGDataProviderGetTypeID()
        }
    }
}

impl CGDataProvider {
    pub fn from_buffer(buffer: &[u8]) -> CGDataProvider {
        unsafe {
            let result = CGDataProviderCreateWithData(ptr::null_mut(),
                                                      buffer.as_ptr() as *const c_void,
                                                      buffer.len() as size_t,
                                                      None);
            TCFType::wrap_under_create_rule(result)
        }
    }

    pub fn from_boxed_slice(slice: Box<[u8]>) -> CGDataProvider {
        unsafe extern fn unbox(_: *mut c_void, ptr: *const c_void, size: size_t) {
            let slice = slice::from_raw_parts_mut(ptr as *mut u8, size);
            let output: Box<[u8]> = Box::from_raw(slice);
            drop(output);
        }
        unsafe {
            let result = CGDataProviderCreateWithData(ptr::null_mut(),
                                                      slice.as_ptr() as *const c_void,
                                                      slice.len() as size_t,
                                                      Some(unbox));
            TCFType::wrap_under_create_rule(result)
        }
    }

    pub fn from_rc_vec(data: Rc<Vec<u8>>) -> CGDataProvider {
        unsafe extern fn unbox(info: *mut c_void, _: *const c_void, _: size_t) {
            let output: Rc<Vec<u8>> = mem::transmute(info);
            drop(output);
        }
        unsafe {
            let ptr = data.as_ptr();
            let size = data.len();
            let result = CGDataProviderCreateWithData(mem::transmute(data),
                                                      ptr as *const c_void,
                                                      size as size_t,
                                                      Some(unbox));
            TCFType::wrap_under_create_rule(result)
        }
    }

    pub fn from_arc_vec(data: Arc<Vec<u8>>) -> CGDataProvider {
        unsafe extern fn unbox(info: *mut c_void, _: *const c_void, _: size_t) {
            let output: Arc<Vec<u8>> = mem::transmute(info);
            drop(output);
        }
        unsafe {
            let ptr = data.as_ptr();
            let size = data.len();
            let result = CGDataProviderCreateWithData(mem::transmute(data),
                                                      ptr as *const c_void,
                                                      size as size_t,
                                                      Some(unbox));
            TCFType::wrap_under_create_rule(result)
        }
    }

}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    //fn CGDataProviderCopyData
    //fn CGDataProviderCreateDirect
    //fn CGDataProviderCreateSequential
    //fn CGDataProviderCreateWithCFData
    fn CGDataProviderCreateWithData(info: *mut c_void,
                                    data: *const c_void,
                                    size: size_t,
                                    releaseData: CGDataProviderReleaseDataCallback
                                   ) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithURL
    fn CGDataProviderGetTypeID() -> CFTypeID;
    //fn CGDataProviderRelease(provider: CGDataProviderRef);
    //fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
