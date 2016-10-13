// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation_sys::base::{CFDowncast, CFObject, CFType, CFTypeID};
use core_foundation_sys::string::{CFString, CFStringRef};
use core_foundation_sys::sync::{CFRef, CFShared};
use data_provider::CGDataProvider;

pub type CGFontRef = CFRef<CGFont>;

#[repr(C)]
pub struct CGFont { obj: CFObject }

unsafe impl Send for CGFont {}
unsafe impl Sync for CGFont {}

unsafe impl CFType for CGFont {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CGFont {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CGFontGetTypeID() }
    }
}

impl CGFont {
    pub fn from_provider(provider: &CGDataProvider) -> Result<CGFontRef, ()> {
        unsafe {
            CFRef::try_from_retained(CGFontCreateWithDataProvider(provider))
        }
    }

    pub fn from_name(name: &CFString) -> Result<CGFontRef, ()> {
        unsafe { CFRef::try_from_retained(CGFontCreateWithFontName(name)) }
    }

    pub fn postscript_name(&self) -> CFStringRef {
        unsafe { CFRef::from_retained(CGFontCopyPostScriptName(self)) }
    }
}

pub type CGGlyph = u16;

extern {
    fn CGFontGetTypeID() -> CFTypeID;

    fn CGFontCreateWithDataProvider(
            provider: &CGDataProvider)
            -> *const CFShared<CGFont>;

    fn CGFontCreateWithFontName(name: &CFString) -> *const CFShared<CGFont>;
    fn CGFontCopyPostScriptName(font: &CGFont) -> *const CFShared<CFString>;
}
