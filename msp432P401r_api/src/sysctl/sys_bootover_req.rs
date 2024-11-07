#[doc = "Register `SYS_BOOTOVER_REQ[%s]` reader"]
pub type R = crate::R<SysBootoverReqSpec>;
#[doc = "Register `SYS_BOOTOVER_REQ[%s]` writer"]
pub type W = crate::W<SysBootoverReqSpec>;
#[doc = "Field `REGVAL` reader - Value set by debugger, read and clear by the CPU"]
pub type RegvalR = crate::FieldReader<u32>;
#[doc = "Field `REGVAL` writer - Value set by debugger, read and clear by the CPU"]
pub type RegvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value set by debugger, read and clear by the CPU"]
    #[inline(always)]
    pub fn regval(&self) -> RegvalR {
        RegvalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value set by debugger, read and clear by the CPU"]
    #[inline(always)]
    pub fn regval(&mut self) -> RegvalW<SysBootoverReqSpec> {
        RegvalW::new(self, 0)
    }
}
#[doc = "Boot Override Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_bootover_req::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_bootover_req::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysBootoverReqSpec;
impl crate::RegisterSpec for SysBootoverReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_bootover_req::R`](R) reader structure"]
impl crate::Readable for SysBootoverReqSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_bootover_req::W`](W) writer structure"]
impl crate::Writable for SysBootoverReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_BOOTOVER_REQ[%s]
to value 0"]
impl crate::Resettable for SysBootoverReqSpec {
    const RESET_VALUE: u32 = 0;
}
