#[doc = "Register `UCBxADDMASK` reader"]
pub type R = crate::R<UcbxAddmaskSpec>;
#[doc = "Register `UCBxADDMASK` writer"]
pub type W = crate::W<UcbxAddmaskSpec>;
#[doc = "Field `ADDMASK` reader - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type AddmaskR = crate::FieldReader<u16>;
#[doc = "Field `ADDMASK` writer - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type AddmaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&self) -> AddmaskR {
        AddmaskR::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&mut self) -> AddmaskW<UcbxAddmaskSpec> {
        AddmaskW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucbx_addmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucbx_addmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcbxAddmaskSpec;
impl crate::RegisterSpec for UcbxAddmaskSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucbx_addmask::R`](R) reader structure"]
impl crate::Readable for UcbxAddmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ucbx_addmask::W`](W) writer structure"]
impl crate::Writable for UcbxAddmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets UCBxADDMASK to value 0x03ff"]
impl crate::Resettable for UcbxAddmaskSpec {
    const RESET_VALUE: u16 = 0x03ff;
}
