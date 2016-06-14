// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
use core_foundation::string::{CFStringRef, CFString};
use libc::{c_void, size_t};
use std::mem;
use std::ptr;
use std::string::ToString;
use std::str::FromStr;

// Document: https://developer.apple.com/library/mac/documentation/GraphicsImaging/Reference/CGColorSpace/index.html
//           #//apple_ref/doc/constant_group/Color_Space_Names
#[derive(Debug, Clone)]
pub enum ColorSpace {
    RGB,
    RGBA,
    ARGB,

    BGR,
    BGRA,
    ABGR,

    YCbCr,
    // TODO: CMYK, Gray, SRGB, XYZ, ...
}

#[repr(C)]
struct __CGColorSpace;

pub type CGColorSpaceRef = *const __CGColorSpace;

pub struct CGColorSpace {
    pub obj: CGColorSpaceRef,
}

impl Drop for CGColorSpace {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl Clone for CGColorSpace {
    fn clone(&self) -> CGColorSpace {
        unsafe {
            TCFType::wrap_under_get_rule(self.as_concrete_TypeRef())
        }
    }
}

impl TCFType<CGColorSpaceRef> for CGColorSpace {
    #[inline]
    fn as_concrete_TypeRef(&self) -> CGColorSpaceRef {
        self.obj
    }

    #[inline]
    unsafe fn wrap_under_get_rule(reference: CGColorSpaceRef) -> CGColorSpace {
        let reference: CGColorSpaceRef = mem::transmute(CFRetain(mem::transmute(reference)));
        TCFType::wrap_under_create_rule(reference)
    }

    #[inline]
    fn as_CFTypeRef(&self) -> CFTypeRef {
        unsafe {
            mem::transmute(self.as_concrete_TypeRef())
        }
    }

    #[inline]
    unsafe fn wrap_under_create_rule(obj: CGColorSpaceRef) -> CGColorSpace {
        CGColorSpace {
            obj: obj,
        }
    }

    #[inline]
    fn type_id() -> CFTypeID {
        unsafe {
            CGColorSpaceGetTypeID()
        }
    }
}

impl CGColorSpace {
    pub fn create_device_rgb() -> CGColorSpace {
        unsafe {
            let result = CGColorSpaceCreateDeviceRGB();
            TCFType::wrap_under_create_rule(result)
        }
    }
}

impl ToString for CGColorSpace {
    fn to_string(&self) -> String {
        unsafe {
            CFString(CGColorSpaceCopyName(self.obj)).to_string()
        }
    }
}
impl FromStr for CGColorSpace {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<CGColorSpace, &'static str> {
        unsafe {
            match s {
                "kCGColorSpaceGenericGray"
                | "kCGColorSpaceGenericRGB"
                | "kCGColorSpaceGenericCMYK"
                | "kCGColorSpaceGenericRGBLinear"
                | "kCGColorSpaceAdobeRGB1998"
                | "kCGColorSpaceSRGB"
                | "kCGColorSpaceGenericGrayGamma2_2"
                | "kCGColorSpaceGenericXYZ"
                | "kCGColorSpaceACESCGLinear"
                | "kCGColorSpaceITUR_709"
                | "kCGColorSpaceITUR_2020"
                | "kCGColorSpaceROMMRGB"
                | "kCGColorSpaceDCIP3"
                | "kCGColorSpaceDisplayP3" => {
                    Ok(CGColorSpace{
                        obj: CGColorSpaceCreateWithName(CFString::new(s).0)
                    })
                },
                _    => {
                    Err("error...")
                }
            }
        }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    pub fn CGColorSpaceCreateDeviceCMYK() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateDeviceRGB()  -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateWithPlatformColorSpace(_ref: *const c_void) -> CGColorSpaceRef;
    pub fn CGColorSpaceCreateWithName(name: CFStringRef) -> CGColorSpaceRef;
    pub fn CGColorSpaceCopyName(space: CGColorSpaceRef)  -> CFStringRef;
    pub fn CGColorSpaceGetTypeID() -> CFTypeID;
}

