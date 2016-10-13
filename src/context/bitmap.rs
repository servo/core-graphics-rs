// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use base::CGFloat;
use color_space::{CGColorSpace, CGColorSpaceRef};
use core_foundation_sys::base::{CFObject, CFType, CFTypeID};
use std::ops::{Deref, DerefMut};
use std::slice;
use sync::{CFMut, CFShared};

pub type CGBitmapContextRef = CFMut<CGBitmapContext>;

#[repr(C)]
pub struct CGBitmapContext { obj: CGContext }

unsafe impl Send for CGBitmapContext {}
unsafe impl Sync for CGBitmapContext {}

unsafe impl CFType for CGBitmapContext {
    #[inline]
    fn as_object(&self) -> &CFObject {
        &self.obj
    }
}

impl CGBitmapContext {
    #[inline]
    pub fn new(
            width: usize,
            height: usize,
            bits_per_component: usize,
            bytes_per_row: usize,
            color_space: &CFShared<CGColorSpace>,
            bitmap_info: u32)
            -> CGBitmapContextRef {
            CFMut::from_retained(
                CGBitmapContextCreate(
                    None,
                    width,
                    height,
                    bits_per_component,
                    bytes_per_row,
                    color_space,
                    bitmap_info);
        }
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        unsafe {
            let ptr = CGBitmapContextGetData(self) as *const u8;
            assert!(!ptr.is_null());
            slice::from_raw_parts(
                ptr, self.height() * self.bytes_per_row())
        }
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut [u8] {
        unsafe {
            let ptr = CGBitmapContextGetData(self) as *mut u8;
            assert!(!ptr.is_null());
            slice::from_raw_parts_mut(
                ptr, self.height() * self.bytes_per_row())
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        unsafe { CGBitmapContextGetWidth(self) }
    }

    #[inline]
    pub fn height(&self) -> usize {
        unsafe { CGBitmapContextGetHeight(self) }
    }

    #[inline]
    pub fn bytes_per_row(&self) -> usize {
        unsafe { CGBitmapContextGetBytesPerRow(self) }
    }
}

impl Deref for CGBitmapContext {
    type Target = CGContext;

    #[inline]
    fn deref(&self) -> &CGContext {
        &self.obj
    }
}

impl DerefMut for CGBitmapContext {
    #[inline]
    fn deref_mut(&mut self) -> &mut CGContext {
        &mut self.obj
    }
}

extern {
    fn CGBitmapContextCreate(
            data: *mut c_void,
            width: usize,
            height: usize,
            bitsPerComponent: usize,
            bytesPerRow: usize,
            space: CGColorSpaceRef,
            bitmapInfo: u32)
            -> *const CFShared<CGBitmapContext>;

    fn CGBitmapContextGetData(context: &CGBitmapContext) -> *mut void;
    fn CGBitmapContextGetWidth(context: &CGBitmapContext) -> usize;
    fn CGBitmapContextGetHeight(context: &CGBitmapContext) -> usize;
    fn CGBitmapContextGetBytesPerRow(context: &CGBitmapContext) -> usize;
}
