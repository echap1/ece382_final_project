#[doc = "Register `FLCTL_BMRK_CTLSTAT` reader"]
pub type R = crate::R<FlctlBmrkCtlstatSpec>;
#[doc = "Register `FLCTL_BMRK_CTLSTAT` writer"]
pub type W = crate::W<FlctlBmrkCtlstatSpec>;
#[doc = "Field `I_BMRK` reader - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
pub type IBmrkR = crate::BitReader;
#[doc = "Field `I_BMRK` writer - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
pub type IBmrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_BMRK` reader - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
pub type DBmrkR = crate::BitReader;
#[doc = "Field `D_BMRK` writer - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
pub type DBmrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP_EN` reader - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
pub type CmpEnR = crate::BitReader;
#[doc = "Field `CMP_EN` writer - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
pub type CmpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects which benchmark register should be compared against the threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpSel {
    #[doc = "0: Compares the Instruction Benchmark Register against the threshold value"]
    En1_0x0 = 0,
    #[doc = "1: Compares the Data Benchmark Register against the threshold value"]
    En2_0x1 = 1,
}
impl From<CmpSel> for bool {
    #[inline(always)]
    fn from(variant: CmpSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_SEL` reader - Selects which benchmark register should be compared against the threshold"]
pub type CmpSelR = crate::BitReader<CmpSel>;
impl CmpSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmpSel {
        match self.bits {
            false => CmpSel::En1_0x0,
            true => CmpSel::En2_0x1,
        }
    }
    #[doc = "Compares the Instruction Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn is_en_1_0x0(&self) -> bool {
        *self == CmpSel::En1_0x0
    }
    #[doc = "Compares the Data Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn is_en_2_0x1(&self) -> bool {
        *self == CmpSel::En2_0x1
    }
}
#[doc = "Field `CMP_SEL` writer - Selects which benchmark register should be compared against the threshold"]
pub type CmpSelW<'a, REG> = crate::BitWriter<'a, REG, CmpSel>;
impl<'a, REG> CmpSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compares the Instruction Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_1_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CmpSel::En1_0x0)
    }
    #[doc = "Compares the Data Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_2_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CmpSel::En2_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&self) -> IBmrkR {
        IBmrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&self) -> DBmrkR {
        DBmrkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&self) -> CmpEnR {
        CmpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&self) -> CmpSelR {
        CmpSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&mut self) -> IBmrkW<FlctlBmrkCtlstatSpec> {
        IBmrkW::new(self, 0)
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&mut self) -> DBmrkW<FlctlBmrkCtlstatSpec> {
        DBmrkW::new(self, 1)
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&mut self) -> CmpEnW<FlctlBmrkCtlstatSpec> {
        CmpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&mut self) -> CmpSelW<FlctlBmrkCtlstatSpec> {
        CmpSelW::new(self, 3)
    }
}
#[doc = "Benchmark Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_bmrk_ctlstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flctl_bmrk_ctlstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlBmrkCtlstatSpec;
impl crate::RegisterSpec for FlctlBmrkCtlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_bmrk_ctlstat::R`](R) reader structure"]
impl crate::Readable for FlctlBmrkCtlstatSpec {}
#[doc = "`write(|w| ..)` method takes [`flctl_bmrk_ctlstat::W`](W) writer structure"]
impl crate::Writable for FlctlBmrkCtlstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLCTL_BMRK_CTLSTAT to value 0"]
impl crate::Resettable for FlctlBmrkCtlstatSpec {
    const RESET_VALUE: u32 = 0;
}
