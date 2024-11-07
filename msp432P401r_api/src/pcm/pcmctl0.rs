#[doc = "Register `PCMCTL0` reader"]
pub type R = crate::R<Pcmctl0Spec>;
#[doc = "Register `PCMCTL0` writer"]
pub type W = crate::W<Pcmctl0Spec>;
#[doc = "Active Mode Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amr {
    #[doc = "0: LDO based Active Mode at Core voltage setting 0."]
    Amr0 = 0,
    #[doc = "1: LDO based Active Mode at Core voltage setting 1."]
    Amr1 = 1,
    #[doc = "4: DC-DC based Active Mode at Core voltage setting 0."]
    Amr4 = 4,
    #[doc = "5: DC-DC based Active Mode at Core voltage setting 1."]
    Amr5 = 5,
    #[doc = "8: Low-Frequency Active Mode at Core voltage setting 0."]
    Amr8 = 8,
    #[doc = "9: Low-Frequency Active Mode at Core voltage setting 1."]
    Amr9 = 9,
}
impl From<Amr> for u8 {
    #[inline(always)]
    fn from(variant: Amr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Amr {
    type Ux = u8;
}
impl crate::IsEnum for Amr {}
#[doc = "Field `AMR` reader - Active Mode Request"]
pub type AmrR = crate::FieldReader<Amr>;
impl AmrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Amr> {
        match self.bits {
            0 => Some(Amr::Amr0),
            1 => Some(Amr::Amr1),
            4 => Some(Amr::Amr4),
            5 => Some(Amr::Amr5),
            8 => Some(Amr::Amr8),
            9 => Some(Amr::Amr9),
            _ => None,
        }
    }
    #[doc = "LDO based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_amr_0(&self) -> bool {
        *self == Amr::Amr0
    }
    #[doc = "LDO based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_amr_1(&self) -> bool {
        *self == Amr::Amr1
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_amr_4(&self) -> bool {
        *self == Amr::Amr4
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_amr_5(&self) -> bool {
        *self == Amr::Amr5
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_amr_8(&self) -> bool {
        *self == Amr::Amr8
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_amr_9(&self) -> bool {
        *self == Amr::Amr9
    }
}
#[doc = "Field `AMR` writer - Active Mode Request"]
pub type AmrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Amr>;
impl<'a, REG> AmrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LDO based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Amr::Amr0)
    }
    #[doc = "LDO based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Amr::Amr1)
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_4(self) -> &'a mut crate::W<REG> {
        self.variant(Amr::Amr4)
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_5(self) -> &'a mut crate::W<REG> {
        self.variant(Amr::Amr5)
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_8(self) -> &'a mut crate::W<REG> {
        self.variant(Amr::Amr8)
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_9(self) -> &'a mut crate::W<REG> {
        self.variant(Amr::Amr9)
    }
}
#[doc = "Low Power Mode Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpmr {
    #[doc = "0: LPM3. Core voltage setting is similar to the mode from which LPM3 is entered."]
    Lpmr0 = 0,
    #[doc = "10: LPM3.5. Core voltage setting 0."]
    Lpmr10 = 10,
    #[doc = "12: LPM4.5"]
    Lpmr12 = 12,
}
impl From<Lpmr> for u8 {
    #[inline(always)]
    fn from(variant: Lpmr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpmr {
    type Ux = u8;
}
impl crate::IsEnum for Lpmr {}
#[doc = "Field `LPMR` reader - Low Power Mode Request"]
pub type LpmrR = crate::FieldReader<Lpmr>;
impl LpmrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lpmr> {
        match self.bits {
            0 => Some(Lpmr::Lpmr0),
            10 => Some(Lpmr::Lpmr10),
            12 => Some(Lpmr::Lpmr12),
            _ => None,
        }
    }
    #[doc = "LPM3. Core voltage setting is similar to the mode from which LPM3 is entered."]
    #[inline(always)]
    pub fn is_lpmr_0(&self) -> bool {
        *self == Lpmr::Lpmr0
    }
    #[doc = "LPM3.5. Core voltage setting 0."]
    #[inline(always)]
    pub fn is_lpmr_10(&self) -> bool {
        *self == Lpmr::Lpmr10
    }
    #[doc = "LPM4.5"]
    #[inline(always)]
    pub fn is_lpmr_12(&self) -> bool {
        *self == Lpmr::Lpmr12
    }
}
#[doc = "Field `LPMR` writer - Low Power Mode Request"]
pub type LpmrW<'a, REG> = crate::FieldWriter<'a, REG, 4, Lpmr>;
impl<'a, REG> LpmrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPM3. Core voltage setting is similar to the mode from which LPM3 is entered."]
    #[inline(always)]
    pub fn lpmr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmr::Lpmr0)
    }
    #[doc = "LPM3.5. Core voltage setting 0."]
    #[inline(always)]
    pub fn lpmr_10(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmr::Lpmr10)
    }
    #[doc = "LPM4.5"]
    #[inline(always)]
    pub fn lpmr_12(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmr::Lpmr12)
    }
}
#[doc = "Current Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CpmEnumRead {
    #[doc = "0: LDO based Active Mode at Core voltage setting 0."]
    Cpm0 = 0,
    #[doc = "1: LDO based Active Mode at Core voltage setting 1."]
    Cpm1 = 1,
    #[doc = "4: DC-DC based Active Mode at Core voltage setting 0."]
    Cpm4 = 4,
    #[doc = "5: DC-DC based Active Mode at Core voltage setting 1."]
    Cpm5 = 5,
    #[doc = "8: Low-Frequency Active Mode at Core voltage setting 0."]
    Cpm8 = 8,
    #[doc = "9: Low-Frequency Active Mode at Core voltage setting 1."]
    Cpm9 = 9,
    #[doc = "16: LDO based LPM0 at Core voltage setting 0."]
    Cpm16 = 16,
    #[doc = "17: LDO based LPM0 at Core voltage setting 1."]
    Cpm17 = 17,
    #[doc = "20: DC-DC based LPM0 at Core voltage setting 0."]
    Cpm20 = 20,
    #[doc = "21: DC-DC based LPM0 at Core voltage setting 1."]
    Cpm21 = 21,
    #[doc = "24: Low-Frequency LPM0 at Core voltage setting 0."]
    Cpm24 = 24,
    #[doc = "25: Low-Frequency LPM0 at Core voltage setting 1."]
    Cpm25 = 25,
    #[doc = "32: LPM3"]
    Cpm32 = 32,
}
impl From<CpmEnumRead> for u8 {
    #[inline(always)]
    fn from(variant: CpmEnumRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CpmEnumRead {
    type Ux = u8;
}
impl crate::IsEnum for CpmEnumRead {}
#[doc = "Field `CPM` reader - Current Power Mode"]
pub type CpmR = crate::FieldReader<CpmEnumRead>;
impl CpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CpmEnumRead> {
        match self.bits {
            0 => Some(CpmEnumRead::Cpm0),
            1 => Some(CpmEnumRead::Cpm1),
            4 => Some(CpmEnumRead::Cpm4),
            5 => Some(CpmEnumRead::Cpm5),
            8 => Some(CpmEnumRead::Cpm8),
            9 => Some(CpmEnumRead::Cpm9),
            16 => Some(CpmEnumRead::Cpm16),
            17 => Some(CpmEnumRead::Cpm17),
            20 => Some(CpmEnumRead::Cpm20),
            21 => Some(CpmEnumRead::Cpm21),
            24 => Some(CpmEnumRead::Cpm24),
            25 => Some(CpmEnumRead::Cpm25),
            32 => Some(CpmEnumRead::Cpm32),
            _ => None,
        }
    }
    #[doc = "LDO based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_cpm_0(&self) -> bool {
        *self == CpmEnumRead::Cpm0
    }
    #[doc = "LDO based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_cpm_1(&self) -> bool {
        *self == CpmEnumRead::Cpm1
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_cpm_4(&self) -> bool {
        *self == CpmEnumRead::Cpm4
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_cpm_5(&self) -> bool {
        *self == CpmEnumRead::Cpm5
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_cpm_8(&self) -> bool {
        *self == CpmEnumRead::Cpm8
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_cpm_9(&self) -> bool {
        *self == CpmEnumRead::Cpm9
    }
    #[doc = "LDO based LPM0 at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_cpm_16(&self) -> bool {
        *self == CpmEnumRead::Cpm16
    }
    #[doc = "LDO based LPM0 at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_cpm_17(&self) -> bool {
        *self == CpmEnumRead::Cpm17
    }
    #[doc = "DC-DC based LPM0 at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_cpm_20(&self) -> bool {
        *self == CpmEnumRead::Cpm20
    }
    #[doc = "DC-DC based LPM0 at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_cpm_21(&self) -> bool {
        *self == CpmEnumRead::Cpm21
    }
    #[doc = "Low-Frequency LPM0 at Core voltage setting 0."]
    #[inline(always)]
    pub fn is_cpm_24(&self) -> bool {
        *self == CpmEnumRead::Cpm24
    }
    #[doc = "Low-Frequency LPM0 at Core voltage setting 1."]
    #[inline(always)]
    pub fn is_cpm_25(&self) -> bool {
        *self == CpmEnumRead::Cpm25
    }
    #[doc = "LPM3"]
    #[inline(always)]
    pub fn is_cpm_32(&self) -> bool {
        *self == CpmEnumRead::Cpm32
    }
}
#[doc = "Field `PCMKEY` reader - PCM key"]
pub type PcmkeyR = crate::FieldReader<u16>;
#[doc = "Field `PCMKEY` writer - PCM key"]
pub type PcmkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Active Mode Request"]
    #[inline(always)]
    pub fn amr(&self) -> AmrR {
        AmrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low Power Mode Request"]
    #[inline(always)]
    pub fn lpmr(&self) -> LpmrR {
        LpmrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Current Power Mode"]
    #[inline(always)]
    pub fn cpm(&self) -> CpmR {
        CpmR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&self) -> PcmkeyR {
        PcmkeyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active Mode Request"]
    #[inline(always)]
    pub fn amr(&mut self) -> AmrW<Pcmctl0Spec> {
        AmrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Low Power Mode Request"]
    #[inline(always)]
    pub fn lpmr(&mut self) -> LpmrW<Pcmctl0Spec> {
        LpmrW::new(self, 4)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&mut self) -> PcmkeyW<Pcmctl0Spec> {
        PcmkeyW::new(self, 16)
    }
}
#[doc = "Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcmctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcmctl0Spec;
impl crate::RegisterSpec for Pcmctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmctl0::R`](R) reader structure"]
impl crate::Readable for Pcmctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcmctl0::W`](W) writer structure"]
impl crate::Writable for Pcmctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCMCTL0 to value 0xa596_0000"]
impl crate::Resettable for Pcmctl0Spec {
    const RESET_VALUE: u32 = 0xa596_0000;
}
