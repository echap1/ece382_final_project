#[doc = "Register `AESAXDIN` writer"]
pub type W = crate::W<AesaxdinSpec>;
#[doc = "Field `AESXDIN0x` writer - AES data in byte n when AESAXDIN is written as half-word"]
pub type Aesxdin0xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESXDIN1x` writer - AES data in byte n+1 when AESAXDIN is written as half-word"]
pub type Aesxdin1xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin0x(&mut self) -> Aesxdin0xW<AesaxdinSpec> {
        Aesxdin0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXDIN is written as half-word"]
    #[inline(always)]
    pub fn aesxdin1x(&mut self) -> Aesxdin1xW<AesaxdinSpec> {
        Aesxdin1xW::new(self, 8)
    }
}
#[doc = "AES Accelerator XORed Data In Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxdin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesaxdinSpec;
impl crate::RegisterSpec for AesaxdinSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`aesaxdin::W`](W) writer structure"]
impl crate::Writable for AesaxdinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AESAXDIN to value 0"]
impl crate::Resettable for AesaxdinSpec {
    const RESET_VALUE: u16 = 0;
}
