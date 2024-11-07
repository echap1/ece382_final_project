#[doc = "Register `UCBxTBCNT` reader"]
pub type R = crate::R<UcbxTbcntSpec>;
#[doc = "Register `UCBxTBCNT` writer"]
pub type W = crate::W<UcbxTbcntSpec>;
#[doc = "Field `UCTBCNT` reader - Byte counter threshold value"]
pub type UctbcntR = crate::FieldReader;
#[doc = "Field `UCTBCNT` writer - Byte counter threshold value"]
pub type UctbcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&self) -> UctbcntR {
        UctbcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&mut self) -> UctbcntW<UcbxTbcntSpec> {
        UctbcntW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_tbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_tbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxTbcntSpec;
impl crate::RegisterSpec for UcbxTbcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_tbcnt::R`](R) reader structure"]
impl crate::Readable for UcbxTbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_tbcnt::W`](W) writer structure"]
impl crate::Writable for UcbxTbcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxTBCNT to value 0"]
impl crate::Resettable for UcbxTbcntSpec {
    const RESET_VALUE: u16 = 0;
}
