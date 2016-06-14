// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core_foundation::base::{CFRelease, CFRetain, CFTypeID, CFTypeRef, TCFType};
use core_foundation::string::{ CFStringRef, CFString };
use libc::{c_void, size_t};
use std::mem;
use std::ptr;

use std::string::ToString;
use std::str::FromStr;



#[derive(Debug, Clone)]
pub enum ColorSpace {
    // Gray,
    // RGB,
    // CMYK,
    // RGBLinear,
    // AdobeRGB1998,
    // SRGB,
    // GrayGamma,
    // XYZ,
    // ACESCGLinear,
    // ITUR_709,    // Y'CbCr
    // ITUR_2020,   // Y'CbCr
    // ROMMRGB,
    // DCIP3,
    // DisplayP3

    RGB,
    RGBA,
    ARGB,

    BGR,
    BGRA,
    ABGR,

    YCbCr,
    // CMYK
    // Gray
    // SRGB
    // XYZ
}
// impl ToString for ColorSpace {
//     fn to_string(&self) -> String {
//         match *self {
//             ColorSpace::Gray         => "kCGColorSpaceGenericGray".to_string(),
//             ColorSpace::RGB          => "kCGColorSpaceGenericRGB".to_string(),
//             ColorSpace::CMYK         => "kCGColorSpaceGenericCMYK".to_string(),
//             ColorSpace::RGBLinear    => "kCGColorSpaceGenericRGBLinear".to_string(),
//             ColorSpace::AdobeRGB1998 => "kCGColorSpaceAdobeRGB1998".to_string(),
//             ColorSpace::SRGB         => "kCGColorSpaceSRGB".to_string(),
//             ColorSpace::GrayGamma    => "kCGColorSpaceGenericGrayGamma2_2".to_string(),
//             ColorSpace::XYZ          => "kCGColorSpaceGenericXYZ".to_string(),
//             ColorSpace::ACESCGLinear => "kCGColorSpaceACESCGLinear".to_string(),
//             ColorSpace::ITUR_709     => "kCGColorSpaceITUR_709".to_string(),
//             ColorSpace::ITUR_2020    => "kCGColorSpaceITUR_2020".to_string(),
//             ColorSpace::ROMMRGB      => "kCGColorSpaceROMMRGB".to_string(),
//             ColorSpace::DCIP3        => "kCGColorSpaceDCIP3".to_string(),
//             ColorSpace::DisplayP3    => "kCGColorSpaceDisplayP3".to_string()
//         }
//     }
// }
// impl FromStr for ColorSpace {
//     type Err = &'static str;
//     fn from_str(s: &str) -> Result<ColorSpace, &'static str> {
//         match s {
//             "Gray"         => Ok(ColorSpace::Gray),
//             "RGB"          => Ok(ColorSpace::RGB),
//             "CMYK"         => Ok(ColorSpace::CMYK),
//             "RGBLinear"    => Ok(ColorSpace::RGBLinear),
//             "AdobeRGB1998" => Ok(ColorSpace::AdobeRGB1998),
//             "SRGB"         => Ok(ColorSpace::SRGB),
//             "GrayGamma"    => Ok(ColorSpace::GrayGamma),
//             "XYZ"          => Ok(ColorSpace::XYZ),
//             "ACESCGLinear" => Ok(ColorSpace::ACESCGLinear),
//             "ITUR_709"     => Ok(ColorSpace::ITUR_709), 
//             "ITUR_2020"    => Ok(ColorSpace::ITUR_2020),
//             "ROMMRGB"      => Ok(ColorSpace::ROMMRGB),
//             "DCIP3"        => Ok(ColorSpace::DCIP3),
//             "DisplayP3"    => Ok(ColorSpace::DisplayP3),
//             _              => Err("Unknow ColorSpace name")
//         }
//     }
// }

// impl ColorSpace {
//     pub fn to_cfstring(&self) -> CFString {
//         match *self {
//             ColorSpace::Gray         => CFString::new("kCGColorSpaceGenericGray"),
//             ColorSpace::RGB          => CFString::new("kCGColorSpaceGenericRGB"),
//             ColorSpace::CMYK         => CFString::new("kCGColorSpaceGenericCMYK"),
//             ColorSpace::RGBLinear    => CFString::new("kCGColorSpaceGenericRGBLinear"),
//             ColorSpace::AdobeRGB1998 => CFString::new("kCGColorSpaceAdobeRGB1998"),
//             ColorSpace::SRGB         => CFString::new("kCGColorSpaceSRGB"),
//             ColorSpace::GrayGamma    => CFString::new("kCGColorSpaceGenericGrayGamma2_2"),
//             ColorSpace::XYZ          => CFString::new("kCGColorSpaceGenericXYZ"),
//             ColorSpace::ACESCGLinear => CFString::new("kCGColorSpaceACESCGLinear"),
//             ColorSpace::ITUR_709     => CFString::new("kCGColorSpaceITUR_709"),
//             ColorSpace::ITUR_2020    => CFString::new("kCGColorSpaceITUR_2020"),
//             ColorSpace::ROMMRGB      => CFString::new("kCGColorSpaceROMMRGB"),
//             ColorSpace::DCIP3        => CFString::new("kCGColorSpaceDCIP3"),
//             ColorSpace::DisplayP3    => CFString::new("kCGColorSpaceDisplayP3")
//         }
//     }
//     pub fn to_CGColorSpaceRef(&self) -> CGColorSpaceRef {
//         unsafe{
//             CGColorSpaceCreateWithName(self.to_cfstring.0)
//         }
//     }
// }


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

