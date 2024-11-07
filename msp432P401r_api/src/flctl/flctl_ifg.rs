#[doc = "Register `FLCTL_IFG` reader"]
pub type R = crate::R<FlctlIfgSpec>;
#[doc = "Field `RDBRST` reader - If set to 1, indicates that the Read Burst/Compare operation is complete"]
pub type RdbrstR = crate::BitReader;
#[doc = "Field `AVPRE` reader - If set to 1, indicates that the pre-program verify operation has detected an error"]
pub type AvpreR = crate::BitReader;
#[doc = "Field `AVPST` reader - If set to 1, indicates that the post-program verify operation has failed comparison"]
pub type AvpstR = crate::BitReader;
#[doc = "Field `PRG` reader - If set to 1, indicates that a word Program operation is complete"]
pub type PrgR = crate::BitReader;
#[doc = "Field `PRGB` reader - If set to 1, indicates that the configured Burst Program operation is complete"]
pub type PrgbR = crate::BitReader;
#[doc = "Field `ERASE` reader - If set to 1, indicates that the Erase operation is complete"]
pub type EraseR = crate::BitReader;
#[doc = "Field `BMRK` reader - If set to 1, indicates that a Benchmark Compare match occurred"]
pub type BmrkR = crate::BitReader;
#[doc = "Field `PRG_ERR` reader - If set to 1, indicates a word composition error in full word write mode (possible data loss due to writes crossing over to a new 128bit boundary before full word has been composed)"]
pub type PrgErrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If set to 1, indicates that the Read Burst/Compare operation is complete"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RdbrstR {
        RdbrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to 1, indicates that the pre-program verify operation has detected an error"]
    #[inline(always)]
    pub fn avpre(&self) -> AvpreR {
        AvpreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to 1, indicates that the post-program verify operation has failed comparison"]
    #[inline(always)]
    pub fn avpst(&self) -> AvpstR {
        AvpstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set to 1, indicates that a word Program operation is complete"]
    #[inline(always)]
    pub fn prg(&self) -> PrgR {
        PrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If set to 1, indicates that the configured Burst Program operation is complete"]
    #[inline(always)]
    pub fn prgb(&self) -> PrgbR {
        PrgbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If set to 1, indicates that the Erase operation is complete"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - If set to 1, indicates that a Benchmark Compare match occurred"]
    #[inline(always)]
    pub fn bmrk(&self) -> BmrkR {
        BmrkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If set to 1, indicates a word composition error in full word write mode (possible data loss due to writes crossing over to a new 128bit boundary before full word has been composed)"]
    #[inline(always)]
    pub fn prg_err(&self) -> PrgErrR {
        PrgErrR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flctl_ifg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlctlIfgSpec;
impl crate::RegisterSpec for FlctlIfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flctl_ifg::R`](R) reader structure"]
impl crate::Readable for FlctlIfgSpec {}
#[doc = "`reset()` method sets FLCTL_IFG to value 0"]
impl crate::Resettable for FlctlIfgSpec {
    const RESET_VALUE: u32 = 0;
}
