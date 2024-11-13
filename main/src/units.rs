#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Time(f32);

impl Time {
    #[inline(always)] pub const fn from_s(t: f32) -> Self { Self(t) }
    #[inline(always)] pub fn from_ms(t: f32) -> Self { Self(t * 0.001) }

    #[inline(always)] pub const fn as_s(&self) -> f32 { self.0 }
    #[inline(always)] pub fn as_s_mut(&mut self) -> &mut f32 { &mut self.0 }
    #[inline(always)] pub fn as_ms(&self) -> f32 { self.0 * 1_000.0 }
    #[inline(always)] pub fn as_us(&self) -> f32 { self.0 * 1_000_000.0 }
}


#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Length(f32);

impl Length {
    #[inline(always)] pub const fn from_m(t: f32) -> Self { Self(t) }
    #[inline(always)] pub fn from_cm(t: f32) -> Self { Self(t * 0.01) }
    #[inline(always)] pub fn from_mm(t: f32) -> Self { Self(t * 0.001) }

    #[inline(always)] pub const fn as_m(&self) -> f32 { self.0 }
    #[inline(always)] pub fn as_m_mut(&mut self) -> &mut f32 { &mut self.0 }
    #[inline(always)] pub fn as_mm(&self) -> f32 { self.0 * 1000.0 }
}


#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Velocity(f32);

impl Velocity {
    #[inline(always)] pub const fn from_m_per_sec(t: f32) -> Self { Self(t) }
    #[inline(always)] pub fn from_mm_per_sec(t: f32) -> Self { Self(t * 0.001) }

    #[inline(always)] pub const fn as_m_per_sec(&self) -> f32 { self.0 }
    #[inline(always)] pub fn as_m_per_sec_mut(&mut self) -> &mut f32 { &mut self.0 }
    #[inline(always)] pub fn as_mm_per_sec(&self) -> f32 { self.0 * 1000.0 }
}


#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Angle(f32);

impl Angle {
    #[inline(always)] pub const fn from_rad(t: f32) -> Self { Self(t) }
    #[inline(always)] pub fn from_deg(t: f32) -> Self { Self(t * 0.017453) }

    #[inline(always)] pub const fn as_rad(&self) -> f32 { self.0 }
    #[inline(always)] pub fn as_rad_mut(&mut self) -> &mut f32 { &mut self.0 }
    #[inline(always)] pub fn as_deg(&self) -> f32 { self.0 * 57.296 }
}


#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct AngularVelocity(f32);

impl AngularVelocity {
    #[inline(always)] pub const fn from_rad_per_sec(t: f32) -> Self { Self(t) }
    #[inline(always)] pub fn from_deg_per_s(t: f32) -> Self { Self(t * 0.017453) }

    #[inline(always)] pub const fn as_rad_per_s(&self) -> f32 { self.0 }
    #[inline(always)] pub fn as_rad_per_s_mut(&mut self) -> &mut f32 { &mut self.0 }
    #[inline(always)] pub fn as_deg_per_s(&self) -> f32 { self.0 * 57.296 }
}
