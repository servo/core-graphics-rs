// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

pub use base::{CGError, boolean_t};
pub use core_foundation_sys::array::CFArray;
pub use core_foundation_sys::sync::CFShared;
pub use geometry::{CGRect, CGPoint, CGSize};

pub type CGDirectDisplayID = u32;
pub type CGWindowID = u32;

pub const kCGNullWindowID: CGWindowID = 0;

pub type CGWindowListOption = u32;

pub const kCGWindowListOptionAll: CGWindowListOption = 0;
pub const kCGWindowListOptionOnScreenOnly: CGWindowListOption = 1 << 0;
pub const kCGWindowListOptionOnScreenAboveWindow: CGWindowListOption = 1 << 1;
pub const kCGWindowListOptionOnScreenBelowWindow: CGWindowListOption = 1 << 2;
pub const kCGWindowListOptionIncludingWindow: CGWindowListOption = 1 << 3;
pub const kCGWindowListExcludeDesktopElements: CGWindowListOption = 1 << 4;

extern {
    pub fn CGMainDisplayID() -> CGDirectDisplayID;
    pub fn CGDisplayIsActive(display: CGDirectDisplayID) -> boolean_t;

    pub fn CGDisplayIsAlwaysInMirrorSet(
            display: CGDirectDisplayID)
            -> boolean_t;

    pub fn CGDisplayIsAsleep(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsBuiltin(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInHWMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsInMirrorSet(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsMain(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsOnline(display: CGDirectDisplayID) -> boolean_t;
    pub fn CGDisplayIsStereo(display: CGDirectDisplayID) -> boolean_t;

    pub fn CGDisplayMirrorsDisplay(
            display: CGDirectDisplayID)
            -> CGDirectDisplayID;

    pub fn CGDisplayPrimaryDisplay(
            display: CGDirectDisplayID)
            -> CGDirectDisplayID;

    pub fn CGDisplayRotation(display: CGDirectDisplayID) -> f64;
    pub fn CGDisplayScreenSize(display: CGDirectDisplayID) -> CGSize;
    pub fn CGDisplaySerialNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayUnitNumber(display: CGDirectDisplayID) -> u32;

    pub fn CGDisplayUsesOpenGLAcceleration(
            display: CGDirectDisplayID)
            -> boolean_t;

    pub fn CGDisplayVendorNumber(display: CGDirectDisplayID) -> u32;

    pub fn CGGetActiveDisplayList(
            max_displays: u32,
            active_displays: *mut CGDirectDisplayID,
            display_count: &mut u32)
            -> CGError;

    pub fn CGDisplayModelNumber(display: CGDirectDisplayID) -> u32;
    pub fn CGDisplayPixelsHigh(display: CGDirectDisplayID) -> usize;
    pub fn CGDisplayPixelsWide(display: CGDirectDisplayID) -> usize;
    pub fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
    pub fn CGDisplayHideCursor(display: CGDirectDisplayID) -> CGError;
    pub fn CGDisplayShowCursor(display: CGDirectDisplayID) -> CGError;

    pub fn CGDisplayMoveCursorToPoint(
            display: CGDirectDisplayID,
            point: CGPoint)
            -> CGError;

    pub fn CGWarpMouseCursorPosition(
            point: CGPoint)
            -> CGError;

    pub fn CGAssociateMouseAndMouseCursorPosition(connected: bool) -> CGError;

    pub fn CGWindowListCopyWindowInfo(
            option: CGWindowListOption,
            relativeToWindow: CGWindowID)
            -> *const CFShared<CFArray>;
}
