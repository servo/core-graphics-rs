// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID, TCFType};
use core_foundation::data::{CFData, CFDataRef};

use libc::{c_void, size_t, off_t};
use std::ptr;
use std::slice;
use std::rc::Rc;
use std::sync::Arc;

use foreign_types::{ForeignType, ForeignTypeRef};

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

foreign_type! {
    #[doc(hidden)]
    type CType = ::sys::CGDataProvider;
    fn drop = |cs| CFRelease(cs as *mut _);
    fn clone = |p| CFRetain(p as *const _) as *mut _;
    pub struct CGDataProvider;
    pub struct CGDataProviderRef;
}

impl CGDataProvider {
    pub fn type_id() -> CFTypeID {
        unsafe {
            CGDataProviderGetTypeID()
        }
    }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        unsafe {
            let result = CGDataProviderCreateWithData(ptr::null_mut(),
                                                      buffer.as_ptr() as *const c_void,
                                                      buffer.len() as size_t,
                                                      None);
            CGDataProvider::from_ptr(result)
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

impl CGDataProviderRef {
    /// Creates a copy of the data from the underlying `CFDataProviderRef`.
    pub fn copy_data(&self) -> CFData {
        unsafe { CFData::wrap_under_create_rule(CGDataProviderCopyData(self.as_ptr())) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern {
    fn CGDataProviderCopyData(provider: ::sys::CGDataProviderRef) -> CFDataRef;
    //fn CGDataProviderCreateDirect
    //fn CGDataProviderCreateSequential
    //fn CGDataProviderCreateWithCFData
    fn CGDataProviderCreateWithData(info: *mut c_void,
                                    data: *const c_void,
                                    size: size_t,
                                    releaseData: CGDataProviderReleaseDataCallback
                                   ) -> ::sys::CGDataProviderRef;
    //fn CGDataProviderCreateWithFilename(filename: *c_char) -> CGDataProviderRef;
    //fn CGDataProviderCreateWithURL
    fn CGDataProviderGetTypeID() -> CFTypeID;
    //fn CGDataProviderRelease(provider: CGDataProviderRef);
    //fn CGDataProviderRetain(provider: CGDataProviderRef) -> CGDataProviderRef;
}
