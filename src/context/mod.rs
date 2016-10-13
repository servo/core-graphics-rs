// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::CGFloat;
use core_foundation_sys::base::{CFDowncast, CFObject, CFType, CFTypeID};

#[repr(C)]
pub struct CGContext { obj: CFObject }

unsafe impl Send for CGContext {}
unsafe impl Sync for CGContext {}

unsafe impl CFType for CGContext {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

unsafe impl CFDowncast for CGContext {
    #[inline]
    fn type_id() -> CFTypeID {
        unsafe { CGContextGetTypeID() }
    }
}

impl CGContext {
    pub fn set_rgb_fill_color(
            &mut self,
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat) {
        unsafe {
            CGContextSetRGBFillColor(self, red, green, blue, alpha)
        }
    }

    pub fn set_allows_font_smoothing(&mut self, allows_font_smoothing: bool) {
        unsafe {
            CGContextSetAllowsFontSmoothing(self, allows_font_smoothing)
        }
    }

    pub fn set_should_smooth_fonts(&mut self, should_smooth_fonts: bool) {
        unsafe {
            CGContextSetShouldSmoothFonts(self, should_smooth_fonts)
        }
    }

    pub fn set_allows_antialiasing(&mut self, allows_antialiasing: bool) {
        unsafe { CGContextSetAllowsAntialiasing(self, allows_antialiasing) }
    }

    pub fn set_should_antialias(&mut self, should_antialias: bool) {
        unsafe { CGContextSetShouldAntialias(self, should_antialias) }
    }
}

extern {
    pub fn CGContextGetTypeID() -> CFTypeID;

    pub fn CGContextSetAllowsFontSmoothing(
            c: &mut CGContext, allowsFontSmoothing: bool);

    pub fn CGContextSetShouldSmoothFonts(
            c: &mut CGContext, shouldSmoothFonts: bool);

    pub fn CGContextSetAllowsAntialiasing(
            c: &mut CGContext, allowsAntialiasing: bool);

    pub fn CGContextSetShouldAntialias(
            c: &mut CGContext, shouldAntialias: bool);

    pub fn CGContextSetRGBFillColor(
            context: &mut CGContext,
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat);
}
