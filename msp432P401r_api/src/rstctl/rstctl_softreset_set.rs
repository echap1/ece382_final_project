#[doc = "Register `RSTCTL_SOFTRESET_SET` reader"]
pub type R = crate::R<RstctlSoftresetSetSpec>;
#[doc = "Register `RSTCTL_SOFTRESET_SET` writer"]
pub type W = crate::W<RstctlSoftresetSetSpec>;
#[doc = "Field `SRC0` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC1` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC2` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC3` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC4` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC5` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC6` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC7` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC8` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC9` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC10` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC11` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC12` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC13` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC14` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC15` writer - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
pub type Src15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src0(&mut self) -> Src0W<RstctlSoftresetSetSpec> {
        Src0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src1(&mut self) -> Src1W<RstctlSoftresetSetSpec> {
        Src1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src2(&mut self) -> Src2W<RstctlSoftresetSetSpec> {
        Src2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src3(&mut self) -> Src3W<RstctlSoftresetSetSpec> {
        Src3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src4(&mut self) -> Src4W<RstctlSoftresetSetSpec> {
        Src4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src5(&mut self) -> Src5W<RstctlSoftresetSetSpec> {
        Src5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src6(&mut self) -> Src6W<RstctlSoftresetSetSpec> {
        Src6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src7(&mut self) -> Src7W<RstctlSoftresetSetSpec> {
        Src7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src8(&mut self) -> Src8W<RstctlSoftresetSetSpec> {
        Src8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src9(&mut self) -> Src9W<RstctlSoftresetSetSpec> {
        Src9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src10(&mut self) -> Src10W<RstctlSoftresetSetSpec> {
        Src10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src11(&mut self) -> Src11W<RstctlSoftresetSetSpec> {
        Src11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src12(&mut self) -> Src12W<RstctlSoftresetSetSpec> {
        Src12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src13(&mut self) -> Src13W<RstctlSoftresetSetSpec> {
        Src13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src14(&mut self) -> Src14W<RstctlSoftresetSetSpec> {
        Src14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write 1 sets the corresponding bit in the RSTCTL_SOFTRESET_STAT (and initiates a Soft Reset)"]
    #[inline(always)]
    pub fn src15(&mut self) -> Src15W<RstctlSoftresetSetSpec> {
        Src15W::new(self, 15)
    }
}
#[doc = "Soft Reset Status Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctl_softreset_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl_softreset_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlSoftresetSetSpec;
impl crate::RegisterSpec for RstctlSoftresetSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl_softreset_set::R`](R) reader structure"]
impl crate::Readable for RstctlSoftresetSetSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl_softreset_set::W`](W) writer structure"]
impl crate::Writable for RstctlSoftresetSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL_SOFTRESET_SET to value 0"]
impl crate::Resettable for RstctlSoftresetSetSpec {
    const RESET_VALUE: u32 = 0;
}
