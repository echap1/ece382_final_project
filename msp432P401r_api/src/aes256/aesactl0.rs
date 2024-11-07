#[doc = "Register `AESACTL0` reader"]
pub type R = crate::R<Aesactl0Spec>;
#[doc = "Register `AESACTL0` writer"]
pub type W = crate::W<Aesactl0Spec>;
#[doc = "AES operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesopx {
    #[doc = "0: Encryption"]
    Aesopx0 = 0,
    #[doc = "1: Decryption. The provided key is the same key used for encryption"]
    Aesopx1 = 1,
    #[doc = "2: Generate first round key required for decryption"]
    Aesopx2 = 2,
    #[doc = "3: Decryption. The provided key is the first round key required for decryption"]
    Aesopx3 = 3,
}
impl From<Aesopx> for u8 {
    #[inline(always)]
    fn from(variant: Aesopx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesopx {
    type Ux = u8;
}
impl crate::IsEnum for Aesopx {}
#[doc = "Field `AESOPx` reader - AES operation"]
pub type AesopxR = crate::FieldReader<Aesopx>;
impl AesopxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesopx {
        match self.bits {
            0 => Aesopx::Aesopx0,
            1 => Aesopx::Aesopx1,
            2 => Aesopx::Aesopx2,
            3 => Aesopx::Aesopx3,
            _ => unreachable!(),
        }
    }
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn is_aesopx_0(&self) -> bool {
        *self == Aesopx::Aesopx0
    }
    #[doc = "Decryption. The provided key is the same key used for encryption"]
    #[inline(always)]
    pub fn is_aesopx_1(&self) -> bool {
        *self == Aesopx::Aesopx1
    }
    #[doc = "Generate first round key required for decryption"]
    #[inline(always)]
    pub fn is_aesopx_2(&self) -> bool {
        *self == Aesopx::Aesopx2
    }
    #[doc = "Decryption. The provided key is the first round key required for decryption"]
    #[inline(always)]
    pub fn is_aesopx_3(&self) -> bool {
        *self == Aesopx::Aesopx3
    }
}
#[doc = "Field `AESOPx` writer - AES operation"]
pub type AesopxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aesopx, crate::Safe>;
impl<'a, REG> AesopxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn aesopx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesopx::Aesopx0)
    }
    #[doc = "Decryption. The provided key is the same key used for encryption"]
    #[inline(always)]
    pub fn aesopx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesopx::Aesopx1)
    }
    #[doc = "Generate first round key required for decryption"]
    #[inline(always)]
    pub fn aesopx_2(self) -> &'a mut crate::W<REG> {
        self.variant(Aesopx::Aesopx2)
    }
    #[doc = "Decryption. The provided key is the first round key required for decryption"]
    #[inline(always)]
    pub fn aesopx_3(self) -> &'a mut crate::W<REG> {
        self.variant(Aesopx::Aesopx3)
    }
}
#[doc = "AES key length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aesklx {
    #[doc = "0: AES128. The key size is 128 bit"]
    Aesklx0 = 0,
    #[doc = "1: AES192. The key size is 192 bit."]
    Aesklx1 = 1,
    #[doc = "2: AES256. The key size is 256 bit"]
    Aesklx2 = 2,
}
impl From<Aesklx> for u8 {
    #[inline(always)]
    fn from(variant: Aesklx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aesklx {
    type Ux = u8;
}
impl crate::IsEnum for Aesklx {}
#[doc = "Field `AESKLx` reader - AES key length"]
pub type AesklxR = crate::FieldReader<Aesklx>;
impl AesklxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aesklx> {
        match self.bits {
            0 => Some(Aesklx::Aesklx0),
            1 => Some(Aesklx::Aesklx1),
            2 => Some(Aesklx::Aesklx2),
            _ => None,
        }
    }
    #[doc = "AES128. The key size is 128 bit"]
    #[inline(always)]
    pub fn is_aesklx_0(&self) -> bool {
        *self == Aesklx::Aesklx0
    }
    #[doc = "AES192. The key size is 192 bit."]
    #[inline(always)]
    pub fn is_aesklx_1(&self) -> bool {
        *self == Aesklx::Aesklx1
    }
    #[doc = "AES256. The key size is 256 bit"]
    #[inline(always)]
    pub fn is_aesklx_2(&self) -> bool {
        *self == Aesklx::Aesklx2
    }
}
#[doc = "Field `AESKLx` writer - AES key length"]
pub type AesklxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aesklx>;
impl<'a, REG> AesklxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES128. The key size is 128 bit"]
    #[inline(always)]
    pub fn aesklx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesklx::Aesklx0)
    }
    #[doc = "AES192. The key size is 192 bit."]
    #[inline(always)]
    pub fn aesklx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesklx::Aesklx1)
    }
    #[doc = "AES256. The key size is 256 bit"]
    #[inline(always)]
    pub fn aesklx_2(self) -> &'a mut crate::W<REG> {
        self.variant(Aesklx::Aesklx2)
    }
}
#[doc = "AES cipher mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aescmx {
    #[doc = "0: ECB"]
    Aescmx0 = 0,
    #[doc = "1: CBC"]
    Aescmx1 = 1,
    #[doc = "2: OFB"]
    Aescmx2 = 2,
    #[doc = "3: CFB"]
    Aescmx3 = 3,
}
impl From<Aescmx> for u8 {
    #[inline(always)]
    fn from(variant: Aescmx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aescmx {
    type Ux = u8;
}
impl crate::IsEnum for Aescmx {}
#[doc = "Field `AESCMx` reader - AES cipher mode select"]
pub type AescmxR = crate::FieldReader<Aescmx>;
impl AescmxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aescmx {
        match self.bits {
            0 => Aescmx::Aescmx0,
            1 => Aescmx::Aescmx1,
            2 => Aescmx::Aescmx2,
            3 => Aescmx::Aescmx3,
            _ => unreachable!(),
        }
    }
    #[doc = "ECB"]
    #[inline(always)]
    pub fn is_aescmx_0(&self) -> bool {
        *self == Aescmx::Aescmx0
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn is_aescmx_1(&self) -> bool {
        *self == Aescmx::Aescmx1
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn is_aescmx_2(&self) -> bool {
        *self == Aescmx::Aescmx2
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn is_aescmx_3(&self) -> bool {
        *self == Aescmx::Aescmx3
    }
}
#[doc = "Field `AESCMx` writer - AES cipher mode select"]
pub type AescmxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aescmx, crate::Safe>;
impl<'a, REG> AescmxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB"]
    #[inline(always)]
    pub fn aescmx_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmx::Aescmx0)
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn aescmx_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmx::Aescmx1)
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn aescmx_2(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmx::Aescmx2)
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn aescmx_3(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmx::Aescmx3)
    }
}
#[doc = "AES software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesswrst {
    #[doc = "0: No reset"]
    Aesswrst0 = 0,
    #[doc = "1: Reset AES accelerator module"]
    Aesswrst1 = 1,
}
impl From<Aesswrst> for bool {
    #[inline(always)]
    fn from(variant: Aesswrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESSWRST` reader - AES software reset"]
pub type AesswrstR = crate::BitReader<Aesswrst>;
impl AesswrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesswrst {
        match self.bits {
            false => Aesswrst::Aesswrst0,
            true => Aesswrst::Aesswrst1,
        }
    }
    #[doc = "No reset"]
    #[inline(always)]
    pub fn is_aesswrst_0(&self) -> bool {
        *self == Aesswrst::Aesswrst0
    }
    #[doc = "Reset AES accelerator module"]
    #[inline(always)]
    pub fn is_aesswrst_1(&self) -> bool {
        *self == Aesswrst::Aesswrst1
    }
}
#[doc = "Field `AESSWRST` writer - AES software reset"]
pub type AesswrstW<'a, REG> = crate::BitWriter<'a, REG, Aesswrst>;
impl<'a, REG> AesswrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset"]
    #[inline(always)]
    pub fn aesswrst_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesswrst::Aesswrst0)
    }
    #[doc = "Reset AES accelerator module"]
    #[inline(always)]
    pub fn aesswrst_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesswrst::Aesswrst1)
    }
}
#[doc = "AES ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesrdyifg {
    #[doc = "0: No interrupt pending"]
    Aesrdyifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Aesrdyifg1 = 1,
}
impl From<Aesrdyifg> for bool {
    #[inline(always)]
    fn from(variant: Aesrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRDYIFG` reader - AES ready interrupt flag"]
pub type AesrdyifgR = crate::BitReader<Aesrdyifg>;
impl AesrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesrdyifg {
        match self.bits {
            false => Aesrdyifg::Aesrdyifg0,
            true => Aesrdyifg::Aesrdyifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_aesrdyifg_0(&self) -> bool {
        *self == Aesrdyifg::Aesrdyifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_aesrdyifg_1(&self) -> bool {
        *self == Aesrdyifg::Aesrdyifg1
    }
}
#[doc = "Field `AESRDYIFG` writer - AES ready interrupt flag"]
pub type AesrdyifgW<'a, REG> = crate::BitWriter<'a, REG, Aesrdyifg>;
impl<'a, REG> AesrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyifg::Aesrdyifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyifg::Aesrdyifg1)
    }
}
#[doc = "AES error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aeserrfg {
    #[doc = "0: No error"]
    Aeserrfg0 = 0,
    #[doc = "1: Error occurred"]
    Aeserrfg1 = 1,
}
impl From<Aeserrfg> for bool {
    #[inline(always)]
    fn from(variant: Aeserrfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESERRFG` reader - AES error flag"]
pub type AeserrfgR = crate::BitReader<Aeserrfg>;
impl AeserrfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aeserrfg {
        match self.bits {
            false => Aeserrfg::Aeserrfg0,
            true => Aeserrfg::Aeserrfg1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_aeserrfg_0(&self) -> bool {
        *self == Aeserrfg::Aeserrfg0
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn is_aeserrfg_1(&self) -> bool {
        *self == Aeserrfg::Aeserrfg1
    }
}
#[doc = "Field `AESERRFG` writer - AES error flag"]
pub type AeserrfgW<'a, REG> = crate::BitWriter<'a, REG, Aeserrfg>;
impl<'a, REG> AeserrfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn aeserrfg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aeserrfg::Aeserrfg0)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn aeserrfg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aeserrfg::Aeserrfg1)
    }
}
#[doc = "AES ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aesrdyie {
    #[doc = "0: Interrupt disabled"]
    Aesrdyie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Aesrdyie1 = 1,
}
impl From<Aesrdyie> for bool {
    #[inline(always)]
    fn from(variant: Aesrdyie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESRDYIE` reader - AES ready interrupt enable"]
pub type AesrdyieR = crate::BitReader<Aesrdyie>;
impl AesrdyieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aesrdyie {
        match self.bits {
            false => Aesrdyie::Aesrdyie0,
            true => Aesrdyie::Aesrdyie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_aesrdyie_0(&self) -> bool {
        *self == Aesrdyie::Aesrdyie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_aesrdyie_1(&self) -> bool {
        *self == Aesrdyie::Aesrdyie1
    }
}
#[doc = "Field `AESRDYIE` writer - AES ready interrupt enable"]
pub type AesrdyieW<'a, REG> = crate::BitWriter<'a, REG, Aesrdyie>;
impl<'a, REG> AesrdyieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn aesrdyie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyie::Aesrdyie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn aesrdyie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aesrdyie::Aesrdyie1)
    }
}
#[doc = "AES cipher mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aescmen {
    #[doc = "0: No DMA triggers are generated"]
    Aescmen0 = 0,
    #[doc = "1: DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    Aescmen1 = 1,
}
impl From<Aescmen> for bool {
    #[inline(always)]
    fn from(variant: Aescmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESCMEN` reader - AES cipher mode enable"]
pub type AescmenR = crate::BitReader<Aescmen>;
impl AescmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aescmen {
        match self.bits {
            false => Aescmen::Aescmen0,
            true => Aescmen::Aescmen1,
        }
    }
    #[doc = "No DMA triggers are generated"]
    #[inline(always)]
    pub fn is_aescmen_0(&self) -> bool {
        *self == Aescmen::Aescmen0
    }
    #[doc = "DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    #[inline(always)]
    pub fn is_aescmen_1(&self) -> bool {
        *self == Aescmen::Aescmen1
    }
}
#[doc = "Field `AESCMEN` writer - AES cipher mode enable"]
pub type AescmenW<'a, REG> = crate::BitWriter<'a, REG, Aescmen>;
impl<'a, REG> AescmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA triggers are generated"]
    #[inline(always)]
    pub fn aescmen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmen::Aescmen0)
    }
    #[doc = "DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    #[inline(always)]
    pub fn aescmen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Aescmen::Aescmen1)
    }
}
impl R {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesopx(&self) -> AesopxR {
        AesopxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aesklx(&self) -> AesklxR {
        AesklxR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescmx(&self) -> AescmxR {
        AescmxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&self) -> AesswrstR {
        AesswrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&self) -> AesrdyifgR {
        AesrdyifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&self) -> AeserrfgR {
        AeserrfgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&self) -> AesrdyieR {
        AesrdyieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&self) -> AescmenR {
        AescmenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesopx(&mut self) -> AesopxW<Aesactl0Spec> {
        AesopxW::new(self, 0)
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aesklx(&mut self) -> AesklxW<Aesactl0Spec> {
        AesklxW::new(self, 2)
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescmx(&mut self) -> AescmxW<Aesactl0Spec> {
        AescmxW::new(self, 5)
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&mut self) -> AesswrstW<Aesactl0Spec> {
        AesswrstW::new(self, 7)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&mut self) -> AesrdyifgW<Aesactl0Spec> {
        AesrdyifgW::new(self, 8)
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&mut self) -> AeserrfgW<Aesactl0Spec> {
        AeserrfgW::new(self, 11)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&mut self) -> AesrdyieW<Aesactl0Spec> {
        AesrdyieW::new(self, 12)
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&mut self) -> AescmenW<Aesactl0Spec> {
        AescmenW::new(self, 15)
    }
}
#[doc = "AES Accelerator Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`aesactl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aesactl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Aesactl0Spec;
impl crate::RegisterSpec for Aesactl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`aesactl0::R`](R) reader structure"]
impl crate::Readable for Aesactl0Spec {}
#[doc = "`write(|w| ..)` method takes [`aesactl0::W`](W) writer structure"]
impl crate::Writable for Aesactl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets AESACTL0 to value 0"]
impl crate::Resettable for Aesactl0Spec {
    const RESET_VALUE: u16 = 0;
}
