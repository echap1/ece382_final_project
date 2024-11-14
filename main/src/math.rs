use core::intrinsics;

#[allow(unused_imports)]
use micromath::F32Ext;

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
pub fn sqrt(x: f32) -> f32 {
    unsafe { intrinsics::sqrtf32(x) }
}
