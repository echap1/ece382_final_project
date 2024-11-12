use core::intrinsics;

#[inline(always)]
pub fn sin(x: f32) -> f32 {
    unsafe { intrinsics::sinf32(x) }
}

#[inline(always)]
pub fn cos(x: f32) -> f32 {
    unsafe { intrinsics::cosf32(x) }
}

#[inline(always)]
pub fn sqrt(x: f32) -> f32 {
    unsafe { intrinsics::sqrtf32(x) }
}