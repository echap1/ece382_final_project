#[doc = "Register `DMA_INT0_SRCFLG` reader"]
pub type R = crate::R<DmaInt0SrcflgSpec>;
#[doc = "Field `CH0` reader - Channel 0 was the source of DMA_INT0"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - Channel 1 was the source of DMA_INT0"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - Channel 2 was the source of DMA_INT0"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - Channel 3 was the source of DMA_INT0"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH4` reader - Channel 4 was the source of DMA_INT0"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH5` reader - Channel 5 was the source of DMA_INT0"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH6` reader - Channel 6 was the source of DMA_INT0"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH7` reader - Channel 7 was the source of DMA_INT0"]
pub type Ch7R = crate::BitReader;
#[doc = "Field `CH8` reader - Channel 8 was the source of DMA_INT0"]
pub type Ch8R = crate::BitReader;
#[doc = "Field `CH9` reader - Channel 9 was the source of DMA_INT0"]
pub type Ch9R = crate::BitReader;
#[doc = "Field `CH10` reader - Channel 10 was the source of DMA_INT0"]
pub type Ch10R = crate::BitReader;
#[doc = "Field `CH11` reader - Channel 11 was the source of DMA_INT0"]
pub type Ch11R = crate::BitReader;
#[doc = "Field `CH12` reader - Channel 12 was the source of DMA_INT0"]
pub type Ch12R = crate::BitReader;
#[doc = "Field `CH13` reader - Channel 13 was the source of DMA_INT0"]
pub type Ch13R = crate::BitReader;
#[doc = "Field `CH14` reader - Channel 14 was the source of DMA_INT0"]
pub type Ch14R = crate::BitReader;
#[doc = "Field `CH15` reader - Channel 15 was the source of DMA_INT0"]
pub type Ch15R = crate::BitReader;
#[doc = "Field `CH16` reader - Channel 16 was the source of DMA_INT0"]
pub type Ch16R = crate::BitReader;
#[doc = "Field `CH17` reader - Channel 17 was the source of DMA_INT0"]
pub type Ch17R = crate::BitReader;
#[doc = "Field `CH18` reader - Channel 18 was the source of DMA_INT0"]
pub type Ch18R = crate::BitReader;
#[doc = "Field `CH19` reader - Channel 19 was the source of DMA_INT0"]
pub type Ch19R = crate::BitReader;
#[doc = "Field `CH20` reader - Channel 20 was the source of DMA_INT0"]
pub type Ch20R = crate::BitReader;
#[doc = "Field `CH21` reader - Channel 21 was the source of DMA_INT0"]
pub type Ch21R = crate::BitReader;
#[doc = "Field `CH22` reader - Channel 22 was the source of DMA_INT0"]
pub type Ch22R = crate::BitReader;
#[doc = "Field `CH23` reader - Channel 23 was the source of DMA_INT0"]
pub type Ch23R = crate::BitReader;
#[doc = "Field `CH24` reader - Channel 24 was the source of DMA_INT0"]
pub type Ch24R = crate::BitReader;
#[doc = "Field `CH25` reader - Channel 25 was the source of DMA_INT0"]
pub type Ch25R = crate::BitReader;
#[doc = "Field `CH26` reader - Channel 26 was the source of DMA_INT0"]
pub type Ch26R = crate::BitReader;
#[doc = "Field `CH27` reader - Channel 27 was the source of DMA_INT0"]
pub type Ch27R = crate::BitReader;
#[doc = "Field `CH28` reader - Channel 28 was the source of DMA_INT0"]
pub type Ch28R = crate::BitReader;
#[doc = "Field `CH29` reader - Channel 29 was the source of DMA_INT0"]
pub type Ch29R = crate::BitReader;
#[doc = "Field `CH30` reader - Channel 30 was the source of DMA_INT0"]
pub type Ch30R = crate::BitReader;
#[doc = "Field `CH31` reader - Channel 31 was the source of DMA_INT0"]
pub type Ch31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 0 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch16(&self) -> Ch16R {
        Ch16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch17(&self) -> Ch17R {
        Ch17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch18(&self) -> Ch18R {
        Ch18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch19(&self) -> Ch19R {
        Ch19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch20(&self) -> Ch20R {
        Ch20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch21(&self) -> Ch21R {
        Ch21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch22(&self) -> Ch22R {
        Ch22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch23(&self) -> Ch23R {
        Ch23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 24 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch24(&self) -> Ch24R {
        Ch24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 25 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch25(&self) -> Ch25R {
        Ch25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 26 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch26(&self) -> Ch26R {
        Ch26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 27 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch27(&self) -> Ch27R {
        Ch27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel 28 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch28(&self) -> Ch28R {
        Ch28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel 29 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch29(&self) -> Ch29R {
        Ch29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel 30 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch30(&self) -> Ch30R {
        Ch30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel 31 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch31(&self) -> Ch31R {
        Ch31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt 0 Source Channel Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int0_srcflg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaInt0SrcflgSpec;
impl crate::RegisterSpec for DmaInt0SrcflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int0_srcflg::R`](R) reader structure"]
impl crate::Readable for DmaInt0SrcflgSpec {}
#[doc = "`reset()` method sets DMA_INT0_SRCFLG to value 0"]
impl crate::Resettable for DmaInt0SrcflgSpec {
    const RESET_VALUE: u32 = 0;
}
