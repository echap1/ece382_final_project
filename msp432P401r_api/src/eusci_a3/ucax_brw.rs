#[doc = "Register `UCAxBRW` reader"]
pub type R = crate::R<UcaxBrwSpec>;
#[doc = "Register `UCAxBRW` writer"]
pub type W = crate::W<UcaxBrwSpec>;
#[doc = "Field `UCBR` reader - Clock prescaler setting of the Baud rate generator"]
pub type UcbrR = crate::FieldReader<u16>;
#[doc = "Field `UCBR` writer - Clock prescaler setting of the Baud rate generator"]
pub type UcbrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock prescaler setting of the Baud rate generator"]
    #[inline(always)]
    pub fn ucbr(&self) -> UcbrR {
        UcbrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock prescaler setting of the Baud rate generator"]
    #[inline(always)]
    pub fn ucbr(&mut self) -> UcbrW<UcaxBrwSpec> {
        UcbrW::new(self, 0)
    }
}
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucax_brw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucax_brw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcaxBrwSpec;
impl crate::RegisterSpec for UcaxBrwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucax_brw::R`](R) reader structure"]
impl crate::Readable for UcaxBrwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucax_brw::W`](W) writer structure"]
impl crate::Writable for UcaxBrwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCAxBRW to value 0"]
impl crate::Resettable for UcaxBrwSpec {
    const RESET_VALUE: u16 = 0;
}
