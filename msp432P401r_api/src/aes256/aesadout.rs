#[doc = "Register `AESADOUT` writer"]
pub type W = crate::W<AesadoutSpec>;
#[doc = "Field `AESDOUT0x` writer - AES data out byte n when AESADOUT is read as half-word"]
pub type Aesdout0xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AESDOUT1x` writer - AES data out byte n+1 when AESADOUT is read as half-word"]
pub type Aesdout1xW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - AES data out byte n when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout0x(&mut self) -> Aesdout0xW<AesadoutSpec> {
        Aesdout0xW::new(self, 0)
    }
    #[doc = "Bits 8:15 - AES data out byte n+1 when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout1x(&mut self) -> Aesdout1xW<AesadoutSpec> {
        Aesdout1xW::new(self, 8)
    }
}
#[doc = "AES Accelerator Data Out Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesadout::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesadoutSpec;
impl crate::RegisterSpec for AesadoutSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`aesadout::W`](W) writer structure"]
impl crate::Writable for AesadoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AESADOUT to value 0"]
impl crate::Resettable for AesadoutSpec {
    const RESET_VALUE: u16 = 0;
}
