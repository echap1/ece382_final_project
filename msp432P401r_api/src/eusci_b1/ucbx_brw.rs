#[doc = "Register `UCBxBRW` reader"]
pub type R = crate::R<UcbxBrwSpec>;
#[doc = "Register `UCBxBRW` writer"]
pub type W = crate::W<UcbxBrwSpec>;
#[doc = "Field `UCBR` reader - Bit clock prescaler"]
pub type UcbrR = crate::FieldReader<u16>;
#[doc = "Field `UCBR` writer - Bit clock prescaler"]
pub type UcbrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Bit clock prescaler"]
    #[inline(always)]
    pub fn ucbr(&self) -> UcbrR {
        UcbrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit clock prescaler"]
    #[inline(always)]
    pub fn ucbr(&mut self) -> UcbrW<UcbxBrwSpec> {
        UcbrW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_brw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_brw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxBrwSpec;
impl crate::RegisterSpec for UcbxBrwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_brw::R`](R) reader structure"]
impl crate::Readable for UcbxBrwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_brw::W`](W) writer structure"]
impl crate::Writable for UcbxBrwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxBRW to value 0"]
impl crate::Resettable for UcbxBrwSpec {
    const RESET_VALUE: u16 = 0;
}
