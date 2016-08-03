
use std::slice;
use std::ops::Deref;
use std::string::ToString;
use std::str::FromStr;
use libc::{ c_void, size_t };

use core_foundation::base::{ CFRelease };
use core_foundation::data::{ CFDataRef, CFDataGetBytePtr, CFDataGetLength };

use data_provider::{ CGDataProviderRef, CGDataProviderCopyData };
use color_space::{ CGColorSpaceRef, CGColorSpace, ColorSpace };

#[derive(Debug, Clone)]
pub enum Pixel {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    ARGB(u8, u8, u8, u8),
    BGR(u8, u8, u8),
    BGRA(u8, u8, u8, u8),
    ABGR(u8, u8, u8, u8),
    YCbCr(u8, u8, u8),    // YUV420P, 
    // TODO: CMYK, ITUR_709, ITUR_2020, SRGB ....
}

#[repr(C)]
struct __CGImageRef;

pub type CGImageRef = *const __CGImageRef;

#[derive(Debug, Clone)]
pub struct CGImage (CGImageRef);

impl CGImage {
    pub fn from_ref(_ref: CGImageRef) -> CGImage {
        CGImage(_ref)
    }
    pub fn width(&self) -> usize {
        unsafe {
            CGImageGetWidth(self.0) as usize
        }
    }
    pub fn height(&self) -> usize {
        unsafe {
            CGImageGetHeight(self.0) as usize
        }
    }
    pub fn pixel_bits(&self) -> usize {
        unsafe{
            CGImageGetBitsPerPixel(self.0) as usize
        }
    }
    pub fn pixel_width(&self) -> usize {
        self.pixel_bits()/8
    }
    pub fn colorspace(&self) -> Result<ColorSpace, &'static str> {
        unsafe {
            let color_space = CGColorSpace{ obj: CGImageGetColorSpace(self.0) };
            match color_space.to_string().as_ref() {
                "kCGColorSpaceGenericRGB" => {
                    match self.alphainfo().unwrap() {
                        ImageAlphaInfo::None
                        | ImageAlphaInfo::NoneSkipLast
                        | ImageAlphaInfo::NoneSkipFirst => {
                            Ok(ColorSpace::RGB)
                        },
                        ImageAlphaInfo::PremultipliedLast
                        | ImageAlphaInfo::Last => {
                            Ok(ColorSpace::RGBA)
                        },
                        ImageAlphaInfo::PremultipliedFirst
                        | ImageAlphaInfo::First => {
                            Ok(ColorSpace::ARGB)
                        },
                        _ => {
                            Err("Unsupport")
                        }
                    }
                },
                "kCGColorSpaceITUR_709" => {
                    match self.alphainfo().unwrap() {
                        ImageAlphaInfo::None
                        | ImageAlphaInfo::NoneSkipLast
                        | ImageAlphaInfo::NoneSkipFirst => {
                            Ok(ColorSpace::YCbCr)
                        },
                        _ => {
                            Err("Unsupport")
                        }
                    }
                },
                "kCGColorSpaceITUR_2020" => {
                    match self.alphainfo().unwrap() {
                        ImageAlphaInfo::None
                        | ImageAlphaInfo::NoneSkipLast
                        | ImageAlphaInfo::NoneSkipFirst => {
                            Ok(ColorSpace::YCbCr)
                        },
                        _ => {
                            Err("Unsupport")
                        }
                    }
                },
                // TODO: kCGColorSpaceGenericGray, kCGColorSpaceGenericCMYK ....
                _ => {
                    Err("Unsupport")
                }
            }
            
        }
    }
    pub fn alphainfo(&self) -> Result<ImageAlphaInfo, &'static str> {
        unsafe {
            ImageAlphaInfo::from_u32(CGImageGetAlphaInfo(self.0))
        }
    }
    pub fn data(&self) -> Vec<u8> {
        // Image Pixels Data
        unsafe {
            let cf_data = CGDataProviderCopyData(CGImageGetDataProvider(self.0));
            let raw_len = CFDataGetLength(cf_data) as usize;
            // Image size is inconsistent with W*H*D.
            assert_eq!(self.width()*self.height()*self.pixel_bits(), raw_len*8 );

            let data = slice::from_raw_parts(CFDataGetBytePtr(cf_data), raw_len);
            CFRelease(cf_data as *const c_void);

            data.to_vec()
        }
    }
}
impl Deref for CGImage {
    type Target = CGImageRef;
    fn deref(&self) -> &CGImageRef {
        &self.0
    }
}
impl Drop for CGImage {
    fn drop(&mut self) {
        unsafe {
            CGImageRelease(self.0)
        }
    }
}


#[derive(Debug, Clone)]
pub enum ImageAlphaInfo {
    None,               // There is no alpha channel.
    PremultipliedLast,  // RGBA
    PremultipliedFirst, // ARGB
    Last,               // RGBA
    First,              // ARGB
    NoneSkipLast,       // There is no alpha channel, This value is equivalent to kCGImageAlphaNone.
    NoneSkipFirst,      // There is no alpha channel, This value is equivalent to kCGImageAlphaNone.
    Only                // There is no color data, only an alpha channel
}
impl ImageAlphaInfo {
    pub fn from_u32(n: u32) -> Result<ImageAlphaInfo, &'static str> {
        match n {
            0 => Ok(ImageAlphaInfo::None),
            1 => Ok(ImageAlphaInfo::PremultipliedLast),
            2 => Ok(ImageAlphaInfo::PremultipliedFirst),
            3 => Ok(ImageAlphaInfo::Last),
            4 => Ok(ImageAlphaInfo::First),
            5 => Ok(ImageAlphaInfo::NoneSkipLast),
            6 => Ok(ImageAlphaInfo::NoneSkipFirst),
            7 => Ok(ImageAlphaInfo::Only),
            _ => Err("Unknow Image Alpha Info Number.")
        }
    }
    pub fn to_u32(&self) -> u32 {
        match *self {
            ImageAlphaInfo::None => 0,
            ImageAlphaInfo::PremultipliedLast  => 1,
            ImageAlphaInfo::PremultipliedFirst => 2,
            ImageAlphaInfo::Last  => 3,
            ImageAlphaInfo::First => 4,
            ImageAlphaInfo::NoneSkipLast  => 5,
            ImageAlphaInfo::NoneSkipFirst => 6,
            ImageAlphaInfo::Only  => 7
        }
    }
}

pub type CGImageAlphaInfo = u32;

#[link(name = "ApplicationServices", kind = "framework")]
extern {
    pub fn CGImageGetAlphaInfo(image: CGImageRef)  -> CGImageAlphaInfo;
    pub fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
    pub fn CGImageGetBitsPerComponent(image: CGImageRef) -> size_t;
    pub fn CGImageGetBitsPerPixel(image: CGImageRef)     -> size_t;
    pub fn CGImageGetBytesPerRow(image: CGImageRef)      -> size_t;
    
    pub fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;

    pub fn CGImageGetHeight(image: CGImageRef) -> size_t;
    pub fn CGImageGetWidth (image: CGImageRef) -> size_t;
    pub fn CGImageCreateCopyWithColorSpace(image: CGImageRef, space: CGColorSpaceRef) -> CGImageRef;
    pub fn CGImageRelease(image: CGImageRef);
}