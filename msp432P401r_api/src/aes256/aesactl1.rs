#[doc = "Register `AESACTL1` reader"]
pub type R = crate::R<Aesactl1Spec>;
#[doc = "Register `AESACTL1` writer"]
pub type W = crate::W<Aesactl1Spec>;
#[doc = "Field `AESBLKCNTx` reader - Cipher Block Counter"]
pub type AesblkcntxR = crate::FieldReader;
#[doc = "Field `AESBLKCNTx` writer - Cipher Block Counter"]
pub type AesblkcntxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcntx(&self) -> AesblkcntxR {
        AesblkcntxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcntx(&mut self) -> AesblkcntxW<Aesactl1Spec> {
        AesblkcntxW::new(self, 0)
    }
}
#[doc = "AES Accelerator Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesactl1Spec;
impl crate::RegisterSpec for Aesactl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesactl1::R`](R) reader structure"]
impl crate::Readable for Aesactl1Spec {}
#[doc = "`write(|w| ..)` method takes [`aesactl1::W`](W) writer structure"]
impl crate::Writable for Aesactl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AESACTL1 to value 0"]
impl crate::Resettable for Aesactl1Spec {
    const RESET_VALUE: u16 = 0;
}
