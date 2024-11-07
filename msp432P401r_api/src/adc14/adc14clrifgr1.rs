#[doc = "Register `ADC14CLRIFGR1` reader"]
pub type R = crate::R<Adc14clrifgr1Spec>;
#[doc = "Register `ADC14CLRIFGR1` writer"]
pub type W = crate::W<Adc14clrifgr1Spec>;
#[doc = "clear ADC14INIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14inifgEnumWrite {
    #[doc = "0: no effect"]
    Clradc14inifg0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14inifg1 = 1,
}
impl From<Clradc14inifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14inifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14INIFG` writer - clear ADC14INIFG"]
pub type Clradc14inifgW<'a, REG> = crate::BitWriter<'a, REG, Clradc14inifgEnumWrite>;
impl<'a, REG> Clradc14inifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14inifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14inifgEnumWrite::Clradc14inifg0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14inifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14inifgEnumWrite::Clradc14inifg1)
    }
}
#[doc = "clear ADC14LOIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14loifgEnumWrite {
    #[doc = "0: no effect"]
    Clradc14loifg0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14loifg1 = 1,
}
impl From<Clradc14loifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14loifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14LOIFG` writer - clear ADC14LOIFG"]
pub type Clradc14loifgW<'a, REG> = crate::BitWriter<'a, REG, Clradc14loifgEnumWrite>;
impl<'a, REG> Clradc14loifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14loifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14loifgEnumWrite::Clradc14loifg0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14loifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14loifgEnumWrite::Clradc14loifg1)
    }
}
#[doc = "clear ADC14HIIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14hiifgEnumWrite {
    #[doc = "0: no effect"]
    Clradc14hiifg0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14hiifg1 = 1,
}
impl From<Clradc14hiifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14hiifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14HIIFG` writer - clear ADC14HIIFG"]
pub type Clradc14hiifgW<'a, REG> = crate::BitWriter<'a, REG, Clradc14hiifgEnumWrite>;
impl<'a, REG> Clradc14hiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14hiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14hiifgEnumWrite::Clradc14hiifg0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14hiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14hiifgEnumWrite::Clradc14hiifg1)
    }
}
#[doc = "clear ADC14OVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14ovifgEnumWrite {
    #[doc = "0: no effect"]
    Clradc14ovifg0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14ovifg1 = 1,
}
impl From<Clradc14ovifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14ovifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14OVIFG` writer - clear ADC14OVIFG"]
pub type Clradc14ovifgW<'a, REG> = crate::BitWriter<'a, REG, Clradc14ovifgEnumWrite>;
impl<'a, REG> Clradc14ovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ovifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ovifgEnumWrite::Clradc14ovifg0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ovifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14ovifgEnumWrite::Clradc14ovifg1)
    }
}
#[doc = "clear ADC14TOVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14tovifgEnumWrite {
    #[doc = "0: no effect"]
    Clradc14tovifg0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14tovifg1 = 1,
}
impl From<Clradc14tovifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14tovifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14TOVIFG` writer - clear ADC14TOVIFG"]
pub type Clradc14tovifgW<'a, REG> = crate::BitWriter<'a, REG, Clradc14tovifgEnumWrite>;
impl<'a, REG> Clradc14tovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14tovifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14tovifgEnumWrite::Clradc14tovifg0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14tovifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14tovifgEnumWrite::Clradc14tovifg1)
    }
}
#[doc = "clear ADC14RDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clradc14rdyifgEnumWrite {
    #[doc = "0: no effect"]
    Clradc14rdyifg0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    Clradc14rdyifg1 = 1,
}
impl From<Clradc14rdyifgEnumWrite> for bool {
    #[inline(always)]
    fn from(variant: Clradc14rdyifgEnumWrite) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14RDYIFG` writer - clear ADC14RDYIFG"]
pub type Clradc14rdyifgW<'a, REG> = crate::BitWriter<'a, REG, Clradc14rdyifgEnumWrite>;
impl<'a, REG> Clradc14rdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14rdyifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14rdyifgEnumWrite::Clradc14rdyifg0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14rdyifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clradc14rdyifgEnumWrite::Clradc14rdyifg1)
    }
}
impl W {
    #[doc = "Bit 1 - clear ADC14INIFG"]
    #[inline(always)]
    pub fn clradc14inifg(&mut self) -> Clradc14inifgW<Adc14clrifgr1Spec> {
        Clradc14inifgW::new(self, 1)
    }
    #[doc = "Bit 2 - clear ADC14LOIFG"]
    #[inline(always)]
    pub fn clradc14loifg(&mut self) -> Clradc14loifgW<Adc14clrifgr1Spec> {
        Clradc14loifgW::new(self, 2)
    }
    #[doc = "Bit 3 - clear ADC14HIIFG"]
    #[inline(always)]
    pub fn clradc14hiifg(&mut self) -> Clradc14hiifgW<Adc14clrifgr1Spec> {
        Clradc14hiifgW::new(self, 3)
    }
    #[doc = "Bit 4 - clear ADC14OVIFG"]
    #[inline(always)]
    pub fn clradc14ovifg(&mut self) -> Clradc14ovifgW<Adc14clrifgr1Spec> {
        Clradc14ovifgW::new(self, 4)
    }
    #[doc = "Bit 5 - clear ADC14TOVIFG"]
    #[inline(always)]
    pub fn clradc14tovifg(&mut self) -> Clradc14tovifgW<Adc14clrifgr1Spec> {
        Clradc14tovifgW::new(self, 5)
    }
    #[doc = "Bit 6 - clear ADC14RDYIFG"]
    #[inline(always)]
    pub fn clradc14rdyifg(&mut self) -> Clradc14rdyifgW<Adc14clrifgr1Spec> {
        Clradc14rdyifgW::new(self, 6)
    }
}
#[doc = "Clear Interrupt Flag 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc14clrifgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc14clrifgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc14clrifgr1Spec;
impl crate::RegisterSpec for Adc14clrifgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc14clrifgr1::R`](R) reader structure"]
impl crate::Readable for Adc14clrifgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc14clrifgr1::W`](W) writer structure"]
impl crate::Writable for Adc14clrifgr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC14CLRIFGR1 to value 0"]
impl crate::Resettable for Adc14clrifgr1Spec {
    const RESET_VALUE: u32 = 0;
}
