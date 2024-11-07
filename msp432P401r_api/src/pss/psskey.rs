#[doc = "Register `PSSKEY` reader"]
pub type R = crate::R<PsskeySpec>;
#[doc = "Register `PSSKEY` writer"]
pub type W = crate::W<PsskeySpec>;
#[doc = "Field `PSSKEY` reader - PSS control key"]
pub type PsskeyR = crate::FieldReader<u16>;
#[doc = "Field `PSSKEY` writer - PSS control key"]
pub type PsskeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PSS control key"]
    #[inline(always)]
    pub fn psskey(&self) -> PsskeyR {
        PsskeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSS control key"]
    #[inline(always)]
    pub fn psskey(&mut self) -> PsskeyW<PsskeySpec> {
        PsskeyW::new(self, 0)
    }
}
#[doc = "Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psskey::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psskey::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsskeySpec;
impl crate::RegisterSpec for PsskeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psskey::R`](R) reader structure"]
impl crate::Readable for PsskeySpec {}
#[doc = "`write(|w| ..)` method takes [`psskey::W`](W) writer structure"]
impl crate::Writable for PsskeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSSKEY to value 0xa596"]
impl crate::Resettable for PsskeySpec {
    const RESET_VALUE: u32 = 0xa596;
}
