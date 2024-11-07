#[doc = "Register `SYS_BOOTOVER_ACK` reader"]
pub type R = crate::R<SysBootoverAckSpec>;
#[doc = "Register `SYS_BOOTOVER_ACK` writer"]
pub type W = crate::W<SysBootoverAckSpec>;
#[doc = "Field `REGVAL` reader - Value set by CPU, read/clear by the debugger"]
pub type RegvalR = crate::FieldReader<u32>;
#[doc = "Field `REGVAL` writer - Value set by CPU, read/clear by the debugger"]
pub type RegvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value set by CPU, read/clear by the debugger"]
    #[inline(always)]
    pub fn regval(&self) -> RegvalR {
        RegvalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value set by CPU, read/clear by the debugger"]
    #[inline(always)]
    pub fn regval(&mut self) -> RegvalW<SysBootoverAckSpec> {
        RegvalW::new(self, 0)
    }
}
#[doc = "Boot Override Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_bootover_ack::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_bootover_ack::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysBootoverAckSpec;
impl crate::RegisterSpec for SysBootoverAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_bootover_ack::R`](R) reader structure"]
impl crate::Readable for SysBootoverAckSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_bootover_ack::W`](W) writer structure"]
impl crate::Writable for SysBootoverAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_BOOTOVER_ACK to value 0"]
impl crate::Resettable for SysBootoverAckSpec {
    const RESET_VALUE: u32 = 0;
}
