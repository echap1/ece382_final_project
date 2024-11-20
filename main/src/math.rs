use core::f32::consts::PI;
use core::intrinsics;

#[allow(unused_imports)]
use micromath::F32Ext;

const TWO_PI: f32 = 6.283185307179586_f32;

#[inline(always)]
pub fn atan2(y: f32, x: f32) -> f32 {
    y.atan2(x)
}
    
#[inline(always)]
pub fn sin(x: f32) -> f32 {
    x.sin()
}

#[inline(always)]
pub fn cos(x: f32) -> f32 {
    x.cos()
}

#[inline(always)]
pub fn sin_cos(x: f32) -> (f32, f32) {
    x.sin_cos()
}

#[inline(always)]
pub fn abs(x: f32) -> f32 {
    x.abs()
}

#[inline(always)]
pub fn sqrt(x: f32) -> f32 {
    unsafe { intrinsics::sqrtf32(x) }
}

#[inline(always)]
pub fn normalize_angle_rad(mut x: f32) -> f32 {
    while x < -PI { x += TWO_PI }
    while x > PI { x -= TWO_PI }
    x
}

#[inline(always)]
pub fn clamp(x: f32, mag: f32) -> f32 {
    if x > mag { mag }
    else if x < -mag { -mag }
    else { x }
}