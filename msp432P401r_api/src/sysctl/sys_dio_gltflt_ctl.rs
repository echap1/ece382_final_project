#[doc = "Register `SYS_DIO_GLTFLT_CTL` reader"]
pub type R = crate::R<SysDioGltfltCtlSpec>;
#[doc = "Register `SYS_DIO_GLTFLT_CTL` writer"]
pub type W = crate::W<SysDioGltfltCtlSpec>;
#[doc = "Glitch filter enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltchEn {
    #[doc = "0: Disables glitch filter on the digital I/Os"]
    GltchEn0 = 0,
    #[doc = "1: Enables glitch filter on the digital I/Os"]
    GltchEn1 = 1,
}
impl From<GltchEn> for bool {
    #[inline(always)]
    fn from(variant: GltchEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLTCH_EN` reader - Glitch filter enable"]
pub type GltchEnR = crate::BitReader<GltchEn>;
impl GltchEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GltchEn {
        match self.bits {
            false => GltchEn::GltchEn0,
            true => GltchEn::GltchEn1,
        }
    }
    #[doc = "Disables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn is_gltch_en_0(&self) -> bool {
        *self == GltchEn::GltchEn0
    }
    #[doc = "Enables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn is_gltch_en_1(&self) -> bool {
        *self == GltchEn::GltchEn1
    }
}
#[doc = "Field `GLTCH_EN` writer - Glitch filter enable"]
pub type GltchEnW<'a, REG> = crate::BitWriter<'a, REG, GltchEn>;
impl<'a, REG> GltchEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn gltch_en_0(self) -> &'a mut crate::W<REG> {
        self.variant(GltchEn::GltchEn0)
    }
    #[doc = "Enables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn gltch_en_1(self) -> &'a mut crate::W<REG> {
        self.variant(GltchEn::GltchEn1)
    }
}
impl R {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gltch_en(&self) -> GltchEnR {
        GltchEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gltch_en(&mut self) -> GltchEnW<SysDioGltfltCtlSpec> {
        GltchEnW::new(self, 0)
    }
}
#[doc = "Digital I/O Glitch Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_dio_gltflt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_dio_gltflt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysDioGltfltCtlSpec;
impl crate::RegisterSpec for SysDioGltfltCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_dio_gltflt_ctl::R`](R) reader structure"]
impl crate::Readable for SysDioGltfltCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_dio_gltflt_ctl::W`](W) writer structure"]
impl crate::Writable for SysDioGltfltCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_DIO_GLTFLT_CTL to value 0x01"]
impl crate::Resettable for SysDioGltfltCtlSpec {
    const RESET_VALUE: u32 = 0x01;
}
