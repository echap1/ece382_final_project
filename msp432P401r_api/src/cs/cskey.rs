#[doc = "Register `CSKEY` reader"]
pub type R = crate::R<CskeySpec>;
#[doc = "Register `CSKEY` writer"]
pub type W = crate::W<CskeySpec>;
#[doc = "Field `CSKEY` reader - Write xxxx_695Ah to unlock"]
pub type CskeyR = crate::FieldReader<u16>;
#[doc = "Field `CSKEY` writer - Write xxxx_695Ah to unlock"]
pub type CskeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write xxxx_695Ah to unlock"]
    #[inline(always)]
    pub fn cskey(&self) -> CskeyR {
        CskeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write xxxx_695Ah to unlock"]
    #[inline(always)]
    pub fn cskey(&mut self) -> CskeyW<CskeySpec> {
        CskeyW::new(self, 0)
    }
}
#[doc = "Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cskey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cskey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CskeySpec;
impl crate::RegisterSpec for CskeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cskey::R`](R) reader structure"]
impl crate::Readable for CskeySpec {}
#[doc = "`write(|w| ..)` method takes [`cskey::W`](W) writer structure"]
impl crate::Writable for CskeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSKEY to value 0xa596"]
impl crate::Resettable for CskeySpec {
    const RESET_VALUE: u32 = 0xa596;
}
