#[doc = "Register `AESADIN` writer"]
pub type W = crate::W<AesadinSpec>;
#[doc = "Field `AESDIN0x` writer - AES data in byte n when AESADIN is written as half-word"]
pub type Aesdin0xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESDIN1x` writer - AES data in byte n+1 when AESADIN is written as half-word"]
pub type Aesdin1xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin0x(&mut self) -> Aesdin0xW<AesadinSpec> {
        Aesdin0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESADIN is written as half-word"]
    #[inline(always)]
    pub fn aesdin1x(&mut self) -> Aesdin1xW<AesadinSpec> {
        Aesdin1xW::new(self, 8)
    }
}
#[doc = "AES Accelerator Data In Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadinSpec;
impl crate::RegisterSpec for AesadinSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`aesadin::W`](W) writer structure"]
impl crate::Writable for AesadinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AESADIN to value 0"]
impl crate::Resettable for AesadinSpec {
    const RESET_VALUE: u16 = 0;
}
