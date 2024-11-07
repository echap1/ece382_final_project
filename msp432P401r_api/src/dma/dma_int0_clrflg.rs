#[doc = "Register `DMA_INT0_CLRFLG` writer"]
pub type W = crate::W<DmaInt0ClrflgSpec>;
#[doc = "Field `CH0` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH16` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH17` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH18` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH19` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH20` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH21` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH22` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH23` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH24` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH25` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH26` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH27` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH28` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH29` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH30` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH31` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type Ch31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<DmaInt0ClrflgSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<DmaInt0ClrflgSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<DmaInt0ClrflgSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<DmaInt0ClrflgSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<DmaInt0ClrflgSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<DmaInt0ClrflgSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<DmaInt0ClrflgSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<DmaInt0ClrflgSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch8(&mut self) -> Ch8W<DmaInt0ClrflgSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch9(&mut self) -> Ch9W<DmaInt0ClrflgSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch10(&mut self) -> Ch10W<DmaInt0ClrflgSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch11(&mut self) -> Ch11W<DmaInt0ClrflgSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch12(&mut self) -> Ch12W<DmaInt0ClrflgSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch13(&mut self) -> Ch13W<DmaInt0ClrflgSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch14(&mut self) -> Ch14W<DmaInt0ClrflgSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch15(&mut self) -> Ch15W<DmaInt0ClrflgSpec> {
        Ch15W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch16(&mut self) -> Ch16W<DmaInt0ClrflgSpec> {
        Ch16W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch17(&mut self) -> Ch17W<DmaInt0ClrflgSpec> {
        Ch17W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch18(&mut self) -> Ch18W<DmaInt0ClrflgSpec> {
        Ch18W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch19(&mut self) -> Ch19W<DmaInt0ClrflgSpec> {
        Ch19W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch20(&mut self) -> Ch20W<DmaInt0ClrflgSpec> {
        Ch20W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch21(&mut self) -> Ch21W<DmaInt0ClrflgSpec> {
        Ch21W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch22(&mut self) -> Ch22W<DmaInt0ClrflgSpec> {
        Ch22W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch23(&mut self) -> Ch23W<DmaInt0ClrflgSpec> {
        Ch23W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch24(&mut self) -> Ch24W<DmaInt0ClrflgSpec> {
        Ch24W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch25(&mut self) -> Ch25W<DmaInt0ClrflgSpec> {
        Ch25W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch26(&mut self) -> Ch26W<DmaInt0ClrflgSpec> {
        Ch26W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch27(&mut self) -> Ch27W<DmaInt0ClrflgSpec> {
        Ch27W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch28(&mut self) -> Ch28W<DmaInt0ClrflgSpec> {
        Ch28W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch29(&mut self) -> Ch29W<DmaInt0ClrflgSpec> {
        Ch29W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch30(&mut self) -> Ch30W<DmaInt0ClrflgSpec> {
        Ch30W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch31(&mut self) -> Ch31W<DmaInt0ClrflgSpec> {
        Ch31W::new(self, 31)
    }
}
#[doc = "Interrupt 0 Source Channel Clear Flag Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int0_clrflg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaInt0ClrflgSpec;
impl crate::RegisterSpec for DmaInt0ClrflgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int0_clrflg::W`](W) writer structure"]
impl crate::Writable for DmaInt0ClrflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INT0_CLRFLG to value 0"]
impl crate::Resettable for DmaInt0ClrflgSpec {
    const RESET_VALUE: u32 = 0;
}
