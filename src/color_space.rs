// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation_sys::base::{CFDowncast, CFObject, CFType, CFTypeID};
use core_foundation_sys::sync::{CFRef, CFShared};

pub type CGColorSpaceRef = CFRef<CGColorSpace>;

#[repr(C)]
pub struct CGColorSpace { obj: CFObject }

unsafe impl Send for CGColorSpace {}
unsafe impl Sync for CGColorSpace {}

unsafe impl CFType for CGColorSpace {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CGColorSpace {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CGColorSpaceGetTypeID() }
    }
}

impl CGColorSpace {
    #[inline]
    pub fn new_device_rgb() -> CGColorSpaceRef {
        unsafe { CFRef::from_retained(CGColorSpaceCreateDeviceRGB()) }
    }
}

extern {
    fn CGColorSpaceCreateDeviceRGB() -> *const CFShared<CGColorSpace>;
    fn CGColorSpaceGetTypeID() -> CFTypeID;
}
