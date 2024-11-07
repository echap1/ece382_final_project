#[doc = "Register `AESAKEY` writer"]
pub type W = crate::W<AesakeySpec>;
#[doc = "Field `AESKEY0x` writer - AES key byte n when AESAKEY is written as half-word"]
pub type Aeskey0xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESKEY1x` writer - AES key byte n+1 when AESAKEY is written as half-word"]
pub type Aeskey1xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - AES key byte n when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey0x(&mut self) -> Aeskey0xW<AesakeySpec> {
        Aeskey0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES key byte n+1 when AESAKEY is written as half-word"]
    #[inline(always)]
    pub fn aeskey1x(&mut self) -> Aeskey1xW<AesakeySpec> {
        Aeskey1xW::new(self, 8)
    }
}
#[doc = "AES Accelerator Key Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesakey::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesakeySpec;
impl crate::RegisterSpec for AesakeySpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`aesakey::W`](W) writer structure"]
impl crate::Writable for AesakeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AESAKEY to value 0"]
impl crate::Resettable for AesakeySpec {
    const RESET_VALUE: u16 = 0;
}
