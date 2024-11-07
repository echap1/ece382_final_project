#[doc = "Register `ADC14CTL1` reader"]
pub type R = crate::R<Adc14ctl1Spec>;
#[doc = "Register `ADC14CTL1` writer"]
pub type W = crate::W<Adc14ctl1Spec>;
#[doc = "ADC14 power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14pwrmd {
    #[doc = "0: Regular power mode for use with any resolution setting. Sample rate can be up to 1 Msps."]
    Adc14pwrmd0 = 0,
    #[doc = "2: Low-power mode for 12-bit, 10-bit, and 8-bit resolution settings. Sample rate must not exceed 200 ksps."]
    Adc14pwrmd2 = 2,
}
impl From<Adc14pwrmd> for u8 {
    #[inline(always)]
    fn from(variant: Adc14pwrmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14pwrmd {
    type Ux = u8;
}
impl crate::IsEnum for Adc14pwrmd {}
#[doc = "Field `ADC14PWRMD` reader - ADC14 power modes"]
pub type Adc14pwrmdR = crate::FieldReader<Adc14pwrmd>;
impl Adc14pwrmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc14pwrmd> {
        match self.bits {
            0 => Some(Adc14pwrmd::Adc14pwrmd0),
            2 => Some(Adc14pwrmd::Adc14pwrmd2),
            _ => None,
        }
    }
    #[doc = "Regular power mode for use with any resolution setting. Sample rate can be up to 1 Msps."]
    #[inline(always)]
    pub fn is_adc14pwrmd_0(&self) -> bool {
        *self == Adc14pwrmd::Adc14pwrmd0
    }
    #[doc = "Low-power mode for 12-bit, 10-bit, and 8-bit resolution settings. Sample rate must not exceed 200 ksps."]
    #[inline(always)]
    pub fn is_adc14pwrmd_2(&self) -> bool {
        *self == Adc14pwrmd::Adc14pwrmd2
    }
}
#[doc = "Field `ADC14PWRMD` writer - ADC14 power modes"]
pub type Adc14pwrmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc14pwrmd>;
impl<'a, REG> Adc14pwrmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Regular power mode for use with any resolution setting. Sample rate can be up to 1 Msps."]
    #[inline(always)]
    pub fn adc14pwrmd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14pwrmd::Adc14pwrmd0)
    }
    #[doc = "Low-power mode for 12-bit, 10-bit, and 8-bit resolution settings. Sample rate must not exceed 200 ksps."]
    #[inline(always)]
    pub fn adc14pwrmd_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14pwrmd::Adc14pwrmd2)
    }
}
#[doc = "ADC14 reference buffer burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14refburst {
    #[doc = "0: ADC reference buffer on continuously"]
    Adc14refburst0 = 0,
    #[doc = "1: ADC reference buffer on only during sample-and-conversion"]
    Adc14refburst1 = 1,
}
impl From<Adc14refburst> for bool {
    #[inline(always)]
    fn from(variant: Adc14refburst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14REFBURST` reader - ADC14 reference buffer burst"]
pub type Adc14refburstR = crate::BitReader<Adc14refburst>;
impl Adc14refburstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14refburst {
        match self.bits {
            false => Adc14refburst::Adc14refburst0,
            true => Adc14refburst::Adc14refburst1,
        }
    }
    #[doc = "ADC reference buffer on continuously"]
    #[inline(always)]
    pub fn is_adc14refburst_0(&self) -> bool {
        *self == Adc14refburst::Adc14refburst0
    }
    #[doc = "ADC reference buffer on only during sample-and-conversion"]
    #[inline(always)]
    pub fn is_adc14refburst_1(&self) -> bool {
        *self == Adc14refburst::Adc14refburst1
    }
}
#[doc = "Field `ADC14REFBURST` writer - ADC14 reference buffer burst"]
pub type Adc14refburstW<'a, REG> = crate::BitWriter<'a, REG, Adc14refburst>;
impl<'a, REG> Adc14refburstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC reference buffer on continuously"]
    #[inline(always)]
    pub fn adc14refburst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14refburst::Adc14refburst0)
    }
    #[doc = "ADC reference buffer on only during sample-and-conversion"]
    #[inline(always)]
    pub fn adc14refburst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14refburst::Adc14refburst1)
    }
}
#[doc = "ADC14 data read-back format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14df {
    #[doc = "0: Binary unsigned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 0000h, and the analog input voltage + V(REF) results in 3FFFh"]
    Adc14df0 = 0,
    #[doc = "1: Signed binary (2s complement), left aligned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 8000h, and the analog input voltage + V(REF) results in 7FFCh"]
    Adc14df1 = 1,
}
impl From<Adc14df> for bool {
    #[inline(always)]
    fn from(variant: Adc14df) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14DF` reader - ADC14 data read-back format"]
pub type Adc14dfR = crate::BitReader<Adc14df>;
impl Adc14dfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14df {
        match self.bits {
            false => Adc14df::Adc14df0,
            true => Adc14df::Adc14df1,
        }
    }
    #[doc = "Binary unsigned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 0000h, and the analog input voltage + V(REF) results in 3FFFh"]
    #[inline(always)]
    pub fn is_adc14df_0(&self) -> bool {
        *self == Adc14df::Adc14df0
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 8000h, and the analog input voltage + V(REF) results in 7FFCh"]
    #[inline(always)]
    pub fn is_adc14df_1(&self) -> bool {
        *self == Adc14df::Adc14df1
    }
}
#[doc = "Field `ADC14DF` writer - ADC14 data read-back format"]
pub type Adc14dfW<'a, REG> = crate::BitWriter<'a, REG, Adc14df>;
impl<'a, REG> Adc14dfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Binary unsigned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 0000h, and the analog input voltage + V(REF) results in 3FFFh"]
    #[inline(always)]
    pub fn adc14df_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14df::Adc14df0)
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 8000h, and the analog input voltage + V(REF) results in 7FFCh"]
    #[inline(always)]
    pub fn adc14df_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14df::Adc14df1)
    }
}
#[doc = "ADC14 resolution\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc14res {
    #[doc = "0: 8 bit (9 clock cycle conversion time)"]
    Adc14res0 = 0,
    #[doc = "1: 10 bit (11 clock cycle conversion time)"]
    Adc14res1 = 1,
    #[doc = "2: 12 bit (14 clock cycle conversion time)"]
    Adc14res2 = 2,
    #[doc = "3: 14 bit (16 clock cycle conversion time)"]
    Adc14res3 = 3,
}
impl From<Adc14res> for u8 {
    #[inline(always)]
    fn from(variant: Adc14res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc14res {
    type Ux = u8;
}
impl crate::IsEnum for Adc14res {}
#[doc = "Field `ADC14RES` reader - ADC14 resolution"]
pub type Adc14resR = crate::FieldReader<Adc14res>;
impl Adc14resR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14res {
        match self.bits {
            0 => Adc14res::Adc14res0,
            1 => Adc14res::Adc14res1,
            2 => Adc14res::Adc14res2,
            3 => Adc14res::Adc14res3,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit (9 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_adc14res_0(&self) -> bool {
        *self == Adc14res::Adc14res0
    }
    #[doc = "10 bit (11 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_adc14res_1(&self) -> bool {
        *self == Adc14res::Adc14res1
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_adc14res_2(&self) -> bool {
        *self == Adc14res::Adc14res2
    }
    #[doc = "14 bit (16 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_adc14res_3(&self) -> bool {
        *self == Adc14res::Adc14res3
    }
}
#[doc = "Field `ADC14RES` writer - ADC14 resolution"]
pub type Adc14resW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adc14res, crate::Safe>;
impl<'a, REG> Adc14resW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit (9 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14res::Adc14res0)
    }
    #[doc = "10 bit (11 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14res::Adc14res1)
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14res::Adc14res2)
    }
    #[doc = "14 bit (16 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14res::Adc14res3)
    }
}
#[doc = "Field `ADC14CSTARTADD` reader - ADC14 conversion start address"]
pub type Adc14cstartaddR = crate::FieldReader;
#[doc = "Field `ADC14CSTARTADD` writer - ADC14 conversion start address"]
pub type Adc14cstartaddW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Controls 1/2 AVCC ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14batmap {
    #[doc = "0: ADC internal 1/2 x AVCC channel is not selected for ADC"]
    Adc14batmap0 = 0,
    #[doc = "1: ADC internal 1/2 x AVCC channel is selected for ADC input channel MAX"]
    Adc14batmap1 = 1,
}
impl From<Adc14batmap> for bool {
    #[inline(always)]
    fn from(variant: Adc14batmap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14BATMAP` reader - Controls 1/2 AVCC ADC input channel selection"]
pub type Adc14batmapR = crate::BitReader<Adc14batmap>;
impl Adc14batmapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14batmap {
        match self.bits {
            false => Adc14batmap::Adc14batmap0,
            true => Adc14batmap::Adc14batmap1,
        }
    }
    #[doc = "ADC internal 1/2 x AVCC channel is not selected for ADC"]
    #[inline(always)]
    pub fn is_adc14batmap_0(&self) -> bool {
        *self == Adc14batmap::Adc14batmap0
    }
    #[doc = "ADC internal 1/2 x AVCC channel is selected for ADC input channel MAX"]
    #[inline(always)]
    pub fn is_adc14batmap_1(&self) -> bool {
        *self == Adc14batmap::Adc14batmap1
    }
}
#[doc = "Field `ADC14BATMAP` writer - Controls 1/2 AVCC ADC input channel selection"]
pub type Adc14batmapW<'a, REG> = crate::BitWriter<'a, REG, Adc14batmap>;
impl<'a, REG> Adc14batmapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC internal 1/2 x AVCC channel is not selected for ADC"]
    #[inline(always)]
    pub fn adc14batmap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14batmap::Adc14batmap0)
    }
    #[doc = "ADC internal 1/2 x AVCC channel is selected for ADC input channel MAX"]
    #[inline(always)]
    pub fn adc14batmap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14batmap::Adc14batmap1)
    }
}
#[doc = "Controls temperature sensor ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14tcmap {
    #[doc = "0: ADC internal temperature sensor channel is not selected for ADC"]
    Adc14tcmap0 = 0,
    #[doc = "1: ADC internal temperature sensor channel is selected for ADC input channel MAX-1"]
    Adc14tcmap1 = 1,
}
impl From<Adc14tcmap> for bool {
    #[inline(always)]
    fn from(variant: Adc14tcmap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14TCMAP` reader - Controls temperature sensor ADC input channel selection"]
pub type Adc14tcmapR = crate::BitReader<Adc14tcmap>;
impl Adc14tcmapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14tcmap {
        match self.bits {
            false => Adc14tcmap::Adc14tcmap0,
            true => Adc14tcmap::Adc14tcmap1,
        }
    }
    #[doc = "ADC internal temperature sensor channel is not selected for ADC"]
    #[inline(always)]
    pub fn is_adc14tcmap_0(&self) -> bool {
        *self == Adc14tcmap::Adc14tcmap0
    }
    #[doc = "ADC internal temperature sensor channel is selected for ADC input channel MAX-1"]
    #[inline(always)]
    pub fn is_adc14tcmap_1(&self) -> bool {
        *self == Adc14tcmap::Adc14tcmap1
    }
}
#[doc = "Field `ADC14TCMAP` writer - Controls temperature sensor ADC input channel selection"]
pub type Adc14tcmapW<'a, REG> = crate::BitWriter<'a, REG, Adc14tcmap>;
impl<'a, REG> Adc14tcmapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC internal temperature sensor channel is not selected for ADC"]
    #[inline(always)]
    pub fn adc14tcmap_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14tcmap::Adc14tcmap0)
    }
    #[doc = "ADC internal temperature sensor channel is selected for ADC input channel MAX-1"]
    #[inline(always)]
    pub fn adc14tcmap_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14tcmap::Adc14tcmap1)
    }
}
#[doc = "Controls internal channel 0 selection to ADC input channel MAX-2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ch0map {
    #[doc = "0: ADC input channel internal 0 is not selected"]
    Adc14ch0map0 = 0,
    #[doc = "1: ADC input channel internal 0 is selected for ADC input channel MAX-2"]
    Adc14ch0map1 = 1,
}
impl From<Adc14ch0map> for bool {
    #[inline(always)]
    fn from(variant: Adc14ch0map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH0MAP` reader - Controls internal channel 0 selection to ADC input channel MAX-2"]
pub type Adc14ch0mapR = crate::BitReader<Adc14ch0map>;
impl Adc14ch0mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ch0map {
        match self.bits {
            false => Adc14ch0map::Adc14ch0map0,
            true => Adc14ch0map::Adc14ch0map1,
        }
    }
    #[doc = "ADC input channel internal 0 is not selected"]
    #[inline(always)]
    pub fn is_adc14ch0map_0(&self) -> bool {
        *self == Adc14ch0map::Adc14ch0map0
    }
    #[doc = "ADC input channel internal 0 is selected for ADC input channel MAX-2"]
    #[inline(always)]
    pub fn is_adc14ch0map_1(&self) -> bool {
        *self == Adc14ch0map::Adc14ch0map1
    }
}
#[doc = "Field `ADC14CH0MAP` writer - Controls internal channel 0 selection to ADC input channel MAX-2"]
pub type Adc14ch0mapW<'a, REG> = crate::BitWriter<'a, REG, Adc14ch0map>;
impl<'a, REG> Adc14ch0mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input channel internal 0 is not selected"]
    #[inline(always)]
    pub fn adc14ch0map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch0map::Adc14ch0map0)
    }
    #[doc = "ADC input channel internal 0 is selected for ADC input channel MAX-2"]
    #[inline(always)]
    pub fn adc14ch0map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch0map::Adc14ch0map1)
    }
}
#[doc = "Controls internal channel 1 selection to ADC input channel MAX-3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ch1map {
    #[doc = "0: ADC input channel internal 1 is not selected"]
    Adc14ch1map0 = 0,
    #[doc = "1: ADC input channel internal 1 is selected for ADC input channel MAX-3"]
    Adc14ch1map1 = 1,
}
impl From<Adc14ch1map> for bool {
    #[inline(always)]
    fn from(variant: Adc14ch1map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH1MAP` reader - Controls internal channel 1 selection to ADC input channel MAX-3"]
pub type Adc14ch1mapR = crate::BitReader<Adc14ch1map>;
impl Adc14ch1mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ch1map {
        match self.bits {
            false => Adc14ch1map::Adc14ch1map0,
            true => Adc14ch1map::Adc14ch1map1,
        }
    }
    #[doc = "ADC input channel internal 1 is not selected"]
    #[inline(always)]
    pub fn is_adc14ch1map_0(&self) -> bool {
        *self == Adc14ch1map::Adc14ch1map0
    }
    #[doc = "ADC input channel internal 1 is selected for ADC input channel MAX-3"]
    #[inline(always)]
    pub fn is_adc14ch1map_1(&self) -> bool {
        *self == Adc14ch1map::Adc14ch1map1
    }
}
#[doc = "Field `ADC14CH1MAP` writer - Controls internal channel 1 selection to ADC input channel MAX-3"]
pub type Adc14ch1mapW<'a, REG> = crate::BitWriter<'a, REG, Adc14ch1map>;
impl<'a, REG> Adc14ch1mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input channel internal 1 is not selected"]
    #[inline(always)]
    pub fn adc14ch1map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch1map::Adc14ch1map0)
    }
    #[doc = "ADC input channel internal 1 is selected for ADC input channel MAX-3"]
    #[inline(always)]
    pub fn adc14ch1map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch1map::Adc14ch1map1)
    }
}
#[doc = "Controls internal channel 2 selection to ADC input channel MAX-4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ch2map {
    #[doc = "0: ADC input channel internal 2 is not selected"]
    Adc14ch2map0 = 0,
    #[doc = "1: ADC input channel internal 2 is selected for ADC input channel MAX-4"]
    Adc14ch2map1 = 1,
}
impl From<Adc14ch2map> for bool {
    #[inline(always)]
    fn from(variant: Adc14ch2map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH2MAP` reader - Controls internal channel 2 selection to ADC input channel MAX-4"]
pub type Adc14ch2mapR = crate::BitReader<Adc14ch2map>;
impl Adc14ch2mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ch2map {
        match self.bits {
            false => Adc14ch2map::Adc14ch2map0,
            true => Adc14ch2map::Adc14ch2map1,
        }
    }
    #[doc = "ADC input channel internal 2 is not selected"]
    #[inline(always)]
    pub fn is_adc14ch2map_0(&self) -> bool {
        *self == Adc14ch2map::Adc14ch2map0
    }
    #[doc = "ADC input channel internal 2 is selected for ADC input channel MAX-4"]
    #[inline(always)]
    pub fn is_adc14ch2map_1(&self) -> bool {
        *self == Adc14ch2map::Adc14ch2map1
    }
}
#[doc = "Field `ADC14CH2MAP` writer - Controls internal channel 2 selection to ADC input channel MAX-4"]
pub type Adc14ch2mapW<'a, REG> = crate::BitWriter<'a, REG, Adc14ch2map>;
impl<'a, REG> Adc14ch2mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input channel internal 2 is not selected"]
    #[inline(always)]
    pub fn adc14ch2map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch2map::Adc14ch2map0)
    }
    #[doc = "ADC input channel internal 2 is selected for ADC input channel MAX-4"]
    #[inline(always)]
    pub fn adc14ch2map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch2map::Adc14ch2map1)
    }
}
#[doc = "Controls internal channel 3 selection to ADC input channel MAX-5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc14ch3map {
    #[doc = "0: ADC input channel internal 3 is not selected"]
    Adc14ch3map0 = 0,
    #[doc = "1: ADC input channel internal 3 is selected for ADC input channel MAX-5"]
    Adc14ch3map1 = 1,
}
impl From<Adc14ch3map> for bool {
    #[inline(always)]
    fn from(variant: Adc14ch3map) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH3MAP` reader - Controls internal channel 3 selection to ADC input channel MAX-5"]
pub type Adc14ch3mapR = crate::BitReader<Adc14ch3map>;
impl Adc14ch3mapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc14ch3map {
        match self.bits {
            false => Adc14ch3map::Adc14ch3map0,
            true => Adc14ch3map::Adc14ch3map1,
        }
    }
    #[doc = "ADC input channel internal 3 is not selected"]
    #[inline(always)]
    pub fn is_adc14ch3map_0(&self) -> bool {
        *self == Adc14ch3map::Adc14ch3map0
    }
    #[doc = "ADC input channel internal 3 is selected for ADC input channel MAX-5"]
    #[inline(always)]
    pub fn is_adc14ch3map_1(&self) -> bool {
        *self == Adc14ch3map::Adc14ch3map1
    }
}
#[doc = "Field `ADC14CH3MAP` writer - Controls internal channel 3 selection to ADC input channel MAX-5"]
pub type Adc14ch3mapW<'a, REG> = crate::BitWriter<'a, REG, Adc14ch3map>;
impl<'a, REG> Adc14ch3mapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC input channel internal 3 is not selected"]
    #[inline(always)]
    pub fn adc14ch3map_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch3map::Adc14ch3map0)
    }
    #[doc = "ADC input channel internal 3 is selected for ADC input channel MAX-5"]
    #[inline(always)]
    pub fn adc14ch3map_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc14ch3map::Adc14ch3map1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC14 power modes"]
    #[inline(always)]
    pub fn adc14pwrmd(&self) -> Adc14pwrmdR {
        Adc14pwrmdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADC14 reference buffer burst"]
    #[inline(always)]
    pub fn adc14refburst(&self) -> Adc14refburstR {
        Adc14refburstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC14 data read-back format"]
    #[inline(always)]
    pub fn adc14df(&self) -> Adc14dfR {
        Adc14dfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ADC14 resolution"]
    #[inline(always)]
    pub fn adc14res(&self) -> Adc14resR {
        Adc14resR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:20 - ADC14 conversion start address"]
    #[inline(always)]
    pub fn adc14cstartadd(&self) -> Adc14cstartaddR {
        Adc14cstartaddR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Controls 1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc14batmap(&self) -> Adc14batmapR {
        Adc14batmapR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Controls temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc14tcmap(&self) -> Adc14tcmapR {
        Adc14tcmapR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Controls internal channel 0 selection to ADC input channel MAX-2"]
    #[inline(always)]
    pub fn adc14ch0map(&self) -> Adc14ch0mapR {
        Adc14ch0mapR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Controls internal channel 1 selection to ADC input channel MAX-3"]
    #[inline(always)]
    pub fn adc14ch1map(&self) -> Adc14ch1mapR {
        Adc14ch1mapR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Controls internal channel 2 selection to ADC input channel MAX-4"]
    #[inline(always)]
    pub fn adc14ch2map(&self) -> Adc14ch2mapR {
        Adc14ch2mapR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Controls internal channel 3 selection to ADC input channel MAX-5"]
    #[inline(always)]
    pub fn adc14ch3map(&self) -> Adc14ch3mapR {
        Adc14ch3mapR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC14 power modes"]
    #[inline(always)]
    pub fn adc14pwrmd(&mut self) -> Adc14pwrmdW<Adc14ctl1Spec> {
        Adc14pwrmdW::new(self, 0)
    }
    #[doc = "Bit 2 - ADC14 reference buffer burst"]
    #[inline(always)]
    pub fn adc14refburst(&mut self) -> Adc14refburstW<Adc14ctl1Spec> {
        Adc14refburstW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC14 data read-back format"]
    #[inline(always)]
    pub fn adc14df(&mut self) -> Adc14dfW<Adc14ctl1Spec> {
        Adc14dfW::new(self, 3)
    }
    #[doc = "Bits 4:5 - ADC14 resolution"]
    #[inline(always)]
    pub fn adc14res(&mut self) -> Adc14resW<Adc14ctl1Spec> {
        Adc14resW::new(self, 4)
    }
    #[doc = "Bits 16:20 - ADC14 conversion start address"]
    #[inline(always)]
    pub fn adc14cstartadd(&mut self) -> Adc14cstartaddW<Adc14ctl1Spec> {
        Adc14cstartaddW::new(self, 16)
    }
    #[doc = "Bit 22 - Controls 1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc14batmap(&mut self) -> Adc14batmapW<Adc14ctl1Spec> {
        Adc14batmapW::new(self, 22)
    }
    #[doc = "Bit 23 - Controls temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc14tcmap(&mut self) -> Adc14tcmapW<Adc14ctl1Spec> {
        Adc14tcmapW::new(self, 23)
    }
    #[doc = "Bit 24 - Controls internal channel 0 selection to ADC input channel MAX-2"]
    #[inline(always)]
    pub fn adc14ch0map(&mut self) -> Adc14ch0mapW<Adc14ctl1Spec> {
        Adc14ch0mapW::new(self, 24)
    }
    #[doc = "Bit 25 - Controls internal channel 1 selection to ADC input channel MAX-3"]
    #[inline(always)]
    pub fn adc14ch1map(&mut self) -> Adc14ch1mapW<Adc14ctl1Spec> {
        Adc14ch1mapW::new(self, 25)
    }
    #[doc = "Bit 26 - Controls internal channel 2 selection to ADC input channel MAX-4"]
    #[inline(always)]
    pub fn adc14ch2map(&mut self) -> Adc14ch2mapW<Adc14ctl1Spec> {
        Adc14ch2mapW::new(self, 26)
    }
    #[doc = "Bit 27 - Controls internal channel 3 selection to ADC input channel MAX-5"]
    #[inline(always)]
    pub fn adc14ch3map(&mut self) -> Adc14ch3mapW<Adc14ctl1Spec> {
        Adc14ch3mapW::new(self, 27)
    }
}
#[doc = "Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14ctl1Spec;
impl crate::RegisterSpec for Adc14ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14ctl1::R`](R) reader structure"]
impl crate::Readable for Adc14ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14ctl1::W`](W) writer structure"]
impl crate::Writable for Adc14ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14CTL1 to value 0x30"]
impl crate::Resettable for Adc14ctl1Spec {
    const RESET_VALUE: u32 = 0x30;
}
