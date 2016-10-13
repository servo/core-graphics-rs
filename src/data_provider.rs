// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation_sys::base::{CFDowncast, CFObject, CFType, CFTypeID};
use core_foundation_sys::data::{CFData};
use core_foundation_sys::sync::{CFRef, CFShared};

pub type CGDataProviderRef = CFRef<CGDataProvider>;

#[repr(C)]
pub struct CGDataProvider { obj: CFObject }

unsafe impl Send for CGDataProvider {}
unsafe impl Sync for CGDataProvider {}

unsafe impl CFType for CGDataProvider {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CGDataProvider {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CGDataProviderGetTypeID() }
    }
}

impl CGDataProvider {
    pub fn from_data(data: &CFShared<CFData>) -> CGDataProviderRef {
        unsafe {
            CFRef::from_retained(CGDataProviderCreateWithCFData(data))
        }
    }
}

extern {
    pub fn CGDataProviderGetTypeID() -> CFTypeID;

    pub fn CGDataProviderCreateWithCFData(
            data: &CFShared<CFData>)
            -> *const CFShared<CGDataProvider>;
}
