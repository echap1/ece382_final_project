#[doc = "Register `RSTCTL_SOFTRESET_STAT` reader"]
pub type R = crate::R<RstctlSoftresetStatSpec>;
#[doc = "Field `SRC0` reader - If 1, indicates that SRC0 was the source of the Soft Reset"]
pub type Src0R = crate::BitReader;
#[doc = "Field `SRC1` reader - If 1, indicates that SRC1 was the source of the Soft Reset"]
pub type Src1R = crate::BitReader;
#[doc = "Field `SRC2` reader - If 1, indicates that SRC2 was the source of the Soft Reset"]
pub type Src2R = crate::BitReader;
#[doc = "Field `SRC3` reader - If 1, indicates that SRC3 was the source of the Soft Reset"]
pub type Src3R = crate::BitReader;
#[doc = "Field `SRC4` reader - If 1, indicates that SRC4 was the source of the Soft Reset"]
pub type Src4R = crate::BitReader;
#[doc = "Field `SRC5` reader - If 1, indicates that SRC5 was the source of the Soft Reset"]
pub type Src5R = crate::BitReader;
#[doc = "Field `SRC6` reader - If 1, indicates that SRC6 was the source of the Soft Reset"]
pub type Src6R = crate::BitReader;
#[doc = "Field `SRC7` reader - If 1, indicates that SRC7 was the source of the Soft Reset"]
pub type Src7R = crate::BitReader;
#[doc = "Field `SRC8` reader - If 1, indicates that SRC8 was the source of the Soft Reset"]
pub type Src8R = crate::BitReader;
#[doc = "Field `SRC9` reader - If 1, indicates that SRC9 was the source of the Soft Reset"]
pub type Src9R = crate::BitReader;
#[doc = "Field `SRC10` reader - If 1, indicates that SRC10 was the source of the Soft Reset"]
pub type Src10R = crate::BitReader;
#[doc = "Field `SRC11` reader - If 1, indicates that SRC11 was the source of the Soft Reset"]
pub type Src11R = crate::BitReader;
#[doc = "Field `SRC12` reader - If 1, indicates that SRC12 was the source of the Soft Reset"]
pub type Src12R = crate::BitReader;
#[doc = "Field `SRC13` reader - If 1, indicates that SRC13 was the source of the Soft Reset"]
pub type Src13R = crate::BitReader;
#[doc = "Field `SRC14` reader - If 1, indicates that SRC14 was the source of the Soft Reset"]
pub type Src14R = crate::BitReader;
#[doc = "Field `SRC15` reader - If 1, indicates that SRC15 was the source of the Soft Reset"]
pub type Src15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If 1, indicates that SRC0 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src0(&self) -> Src0R {
        Src0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, indicates that SRC1 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src1(&self) -> Src1R {
        Src1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If 1, indicates that SRC2 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src2(&self) -> Src2R {
        Src2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, indicates that SRC3 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src3(&self) -> Src3R {
        Src3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If 1, indicates that SRC4 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src4(&self) -> Src4R {
        Src4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If 1, indicates that SRC5 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src5(&self) -> Src5R {
        Src5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If 1, indicates that SRC6 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src6(&self) -> Src6R {
        Src6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If 1, indicates that SRC7 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src7(&self) -> Src7R {
        Src7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If 1, indicates that SRC8 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src8(&self) -> Src8R {
        Src8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If 1, indicates that SRC9 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src9(&self) -> Src9R {
        Src9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If 1, indicates that SRC10 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src10(&self) -> Src10R {
        Src10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If 1, indicates that SRC11 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src11(&self) -> Src11R {
        Src11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If 1, indicates that SRC12 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src12(&self) -> Src12R {
        Src12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - If 1, indicates that SRC13 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src13(&self) -> Src13R {
        Src13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If 1, indicates that SRC14 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src14(&self) -> Src14R {
        Src14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If 1, indicates that SRC15 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src15(&self) -> Src15R {
        Src15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Soft Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_softreset_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlSoftresetStatSpec;
impl crate::RegisterSpec for RstctlSoftresetStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_softreset_stat::R`](R) reader structure"]
impl crate::Readable for RstctlSoftresetStatSpec {}
#[doc = "`reset()` method sets RSTCTL_SOFTRESET_STAT to value 0"]
impl crate::Resettable for RstctlSoftresetStatSpec {
    const RESET_VALUE: u32 = 0;
}
