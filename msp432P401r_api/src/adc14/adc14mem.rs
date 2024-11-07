#[doc = "Register `ADC14MEM[%s]` reader"]
pub type R = crate::R<Adc14memSpec>;
#[doc = "Register `ADC14MEM[%s]` writer"]
pub type W = crate::W<Adc14memSpec>;
#[doc = "Field `Conversion_Results` reader - Conversion Result"]
pub type ConversionResultsR = crate::FieldReader<u16>;
#[doc = "Field `Conversion_Results` writer - Conversion Result"]
pub type ConversionResultsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Conversion Result"]
    #[inline(always)]
    pub fn conversion_results(&self) -> ConversionResultsR {
        ConversionResultsR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Conversion Result"]
    #[inline(always)]
    pub fn conversion_results(&mut self) -> ConversionResultsW<Adc14memSpec> {
        ConversionResultsW::new(self, 0)
    }
}
#[doc = "Conversion Memory Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14memSpec;
impl crate::RegisterSpec for Adc14memSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14mem::R`](R) reader structure"]
impl crate::Readable for Adc14memSpec {}
#[doc = "`write(|w| ..)` method takes [`adc14mem::W`](W) writer structure"]
impl crate::Writable for Adc14memSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14MEM[%s]
to value 0"]
impl crate::Resettable for Adc14memSpec {
    const RESET_VALUE: u32 = 0;
}
