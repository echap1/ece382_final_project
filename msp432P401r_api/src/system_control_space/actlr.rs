#[doc = "Register `ACTLR` reader"]
pub type R = crate::R<ActlrSpec>;
#[doc = "Register `ACTLR` writer"]
pub type W = crate::W<ActlrSpec>;
#[doc = "Field `DISMCYCINT` reader - Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
pub type DismcycintR = crate::BitReader;
#[doc = "Field `DISMCYCINT` writer - Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
pub type DismcycintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISDEFWBUF` reader - Disables write buffer us during default memorty map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
pub type DisdefwbufR = crate::BitReader;
#[doc = "Field `DISDEFWBUF` writer - Disables write buffer us during default memorty map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
pub type DisdefwbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFOLD` reader - Disables IT folding."]
pub type DisfoldR = crate::BitReader;
#[doc = "Field `DISFOLD` writer - Disables IT folding."]
pub type DisfoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISFPCA` reader - Disable automatic update of CONTROL.FPCA"]
pub type DisfpcaR = crate::BitReader;
#[doc = "Field `DISFPCA` writer - Disable automatic update of CONTROL.FPCA"]
pub type DisfpcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISOOFP` reader - Disables floating point instructions completing out of order with respect to integer instructions."]
pub type DisoofpR = crate::BitReader;
#[doc = "Field `DISOOFP` writer - Disables floating point instructions completing out of order with respect to integer instructions."]
pub type DisoofpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[inline(always)]
    pub fn dismcycint(&self) -> DismcycintR {
        DismcycintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disables write buffer us during default memorty map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DisdefwbufR {
        DisdefwbufR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disables IT folding."]
    #[inline(always)]
    pub fn disfold(&self) -> DisfoldR {
        DisfoldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&self) -> DisfpcaR {
        DisfpcaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disables floating point instructions completing out of order with respect to integer instructions."]
    #[inline(always)]
    pub fn disoofp(&self) -> DisoofpR {
        DisoofpR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[inline(always)]
    pub fn dismcycint(&mut self) -> DismcycintW<ActlrSpec> {
        DismcycintW::new(self, 0)
    }
    #[doc = "Bit 1 - Disables write buffer us during default memorty map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[inline(always)]
    pub fn disdefwbuf(&mut self) -> DisdefwbufW<ActlrSpec> {
        DisdefwbufW::new(self, 1)
    }
    #[doc = "Bit 2 - Disables IT folding."]
    #[inline(always)]
    pub fn disfold(&mut self) -> DisfoldW<ActlrSpec> {
        DisfoldW::new(self, 2)
    }
    #[doc = "Bit 8 - Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&mut self) -> DisfpcaW<ActlrSpec> {
        DisfpcaW::new(self, 8)
    }
    #[doc = "Bit 9 - Disables floating point instructions completing out of order with respect to integer instructions."]
    #[inline(always)]
    pub fn disoofp(&mut self) -> DisoofpW<ActlrSpec> {
        DisoofpW::new(self, 9)
    }
}
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`actlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActlrSpec;
impl crate::RegisterSpec for ActlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actlr::R`](R) reader structure"]
impl crate::Readable for ActlrSpec {}
#[doc = "`write(|w| ..)` method takes [`actlr::W`](W) writer structure"]
impl crate::Writable for ActlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ActlrSpec {
    const RESET_VALUE: u32 = 0;
}
