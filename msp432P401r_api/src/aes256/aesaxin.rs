#[doc = "Register `AESAXIN` writer"]
pub type W = crate::W<AesaxinSpec>;
#[doc = "Field `AESXIN0x` writer - AES data in byte n when AESAXIN is written as half-word"]
pub type Aesxin0xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESXIN1x` writer - AES data in byte n+1 when AESAXIN is written as half-word"]
pub type Aesxin1xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin0x(&mut self) -> Aesxin0xW<AesaxinSpec> {
        Aesxin0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin1x(&mut self) -> Aesxin1xW<AesaxinSpec> {
        Aesxin1xW::new(self, 8)
    }
}
#[doc = "AES Accelerator XORed Data In Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesaxin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesaxinSpec;
impl crate::RegisterSpec for AesaxinSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`aesaxin::W`](W) writer structure"]
impl crate::Writable for AesaxinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AESAXIN to value 0"]
impl crate::Resettable for AesaxinSpec {
    const RESET_VALUE: u16 = 0;
}
