#[doc = "Register `PMAPKEYID` reader"]
pub type R = crate::R<PmapkeyidSpec>;
#[doc = "Register `PMAPKEYID` writer"]
pub type W = crate::W<PmapkeyidSpec>;
#[doc = "Field `PMAPKEY` reader - Port mapping controller write access key"]
pub type PmapkeyR = crate::FieldReader<u16>;
#[doc = "Field `PMAPKEY` writer - Port mapping controller write access key"]
pub type PmapkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port mapping controller write access key"]
    #[inline(always)]
    pub fn pmapkey(&self) -> PmapkeyR {
        PmapkeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port mapping controller write access key"]
    #[inline(always)]
    pub fn pmapkey(&mut self) -> PmapkeyW<PmapkeyidSpec> {
        PmapkeyW::new(self, 0)
    }
}
#[doc = "Port Mapping Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmapkeyid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmapkeyid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmapkeyidSpec;
impl crate::RegisterSpec for PmapkeyidSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pmapkeyid::R`](R) reader structure"]
impl crate::Readable for PmapkeyidSpec {}
#[doc = "`write(|w| ..)` method takes [`pmapkeyid::W`](W) writer structure"]
impl crate::Writable for PmapkeyidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMAPKEYID to value 0x96a5"]
impl crate::Resettable for PmapkeyidSpec {
    const RESET_VALUE: u16 = 0x96a5;
}
