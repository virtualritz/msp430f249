#[doc = "Register `UCA1CTL0` reader"]
pub struct R(crate::R<UCA1CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCA1CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCA1CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCA1CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCA1CTL0` writer"]
pub struct W(crate::W<UCA1CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCA1CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UCA1CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCA1CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCSYNC` reader - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_R = crate::BitReader;
#[doc = "Field `UCSYNC` writer - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
pub type UCSYNC_W<'a, const O: u8> = crate::BitWriter<'a, UCA1CTL0_SPEC, O>;
#[doc = "Field `UCMODE` reader - Async. Mode: USCI Mode 1"]
pub type UCMODE_R = crate::FieldReader<UCMODE_A>;
#[doc = "Async. Mode: USCI Mode 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UCMODE_A {
    #[doc = "0: Sync. Mode: USCI Mode: 0"]
    UCMODE_0 = 0,
    #[doc = "1: Sync. Mode: USCI Mode: 1"]
    UCMODE_1 = 1,
    #[doc = "2: Sync. Mode: USCI Mode: 2"]
    UCMODE_2 = 2,
    #[doc = "3: Sync. Mode: USCI Mode: 3"]
    UCMODE_3 = 3,
}
impl From<UCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: UCMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UCMODE_A {
    type Ux = u8;
}
impl UCMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCMODE_A {
        match self.bits {
            0 => UCMODE_A::UCMODE_0,
            1 => UCMODE_A::UCMODE_1,
            2 => UCMODE_A::UCMODE_2,
            3 => UCMODE_A::UCMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCMODE_0`"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == UCMODE_A::UCMODE_0
    }
    #[doc = "Checks if the value of the field is `UCMODE_1`"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == UCMODE_A::UCMODE_1
    }
    #[doc = "Checks if the value of the field is `UCMODE_2`"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == UCMODE_A::UCMODE_2
    }
    #[doc = "Checks if the value of the field is `UCMODE_3`"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == UCMODE_A::UCMODE_3
    }
}
#[doc = "Field `UCMODE` writer - Async. Mode: USCI Mode 1"]
pub type UCMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, UCA1CTL0_SPEC, 2, O, UCMODE_A>;
impl<'a, const O: u8> UCMODE_W<'a, O> {
    #[doc = "Sync. Mode: USCI Mode: 0"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_0)
    }
    #[doc = "Sync. Mode: USCI Mode: 1"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_1)
    }
    #[doc = "Sync. Mode: USCI Mode: 2"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_2)
    }
    #[doc = "Sync. Mode: USCI Mode: 3"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut W {
        self.variant(UCMODE_A::UCMODE_3)
    }
}
#[doc = "Field `UCSPB` reader - Async. Mode: Stop Bits 0:one / 1: two"]
pub type UCSPB_R = crate::BitReader;
#[doc = "Field `UCSPB` writer - Async. Mode: Stop Bits 0:one / 1: two"]
pub type UCSPB_W<'a, const O: u8> = crate::BitWriter<'a, UCA1CTL0_SPEC, O>;
#[doc = "Field `UC7BIT` reader - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type UC7BIT_R = crate::BitReader;
#[doc = "Field `UC7BIT` writer - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
pub type UC7BIT_W<'a, const O: u8> = crate::BitWriter<'a, UCA1CTL0_SPEC, O>;
#[doc = "Field `UCMSB` reader - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub type UCMSB_R = crate::BitReader;
#[doc = "Field `UCMSB` writer - Async. Mode: MSB first 0:LSB / 1:MSB"]
pub type UCMSB_W<'a, const O: u8> = crate::BitWriter<'a, UCA1CTL0_SPEC, O>;
#[doc = "Field `UCPAR` reader - Async. Mode: Parity 0:odd / 1:even"]
pub type UCPAR_R = crate::BitReader;
#[doc = "Field `UCPAR` writer - Async. Mode: Parity 0:odd / 1:even"]
pub type UCPAR_W<'a, const O: u8> = crate::BitWriter<'a, UCA1CTL0_SPEC, O>;
#[doc = "Field `UCPEN` reader - Async. Mode: Parity enable"]
pub type UCPEN_R = crate::BitReader;
#[doc = "Field `UCPEN` writer - Async. Mode: Parity enable"]
pub type UCPEN_W<'a, const O: u8> = crate::BitWriter<'a, UCA1CTL0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    pub fn ucsync(&self) -> UCSYNC_R {
        UCSYNC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    pub fn ucmode(&self) -> UCMODE_R {
        UCMODE_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    pub fn ucspb(&self) -> UCSPB_R {
        UCSPB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    pub fn uc7bit(&self) -> UC7BIT_R {
        UC7BIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UCMSB_R {
        UCMSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    pub fn ucpar(&self) -> UCPAR_R {
        UCPAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UCPEN_R {
        UCPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sync-Mode 0:UART-Mode / 1:SPI-Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ucsync(&mut self) -> UCSYNC_W<0> {
        UCSYNC_W::new(self)
    }
    #[doc = "Bits 1:2 - Async. Mode: USCI Mode 1"]
    #[inline(always)]
    #[must_use]
    pub fn ucmode(&mut self) -> UCMODE_W<1> {
        UCMODE_W::new(self)
    }
    #[doc = "Bit 3 - Async. Mode: Stop Bits 0:one / 1: two"]
    #[inline(always)]
    #[must_use]
    pub fn ucspb(&mut self) -> UCSPB_W<3> {
        UCSPB_W::new(self)
    }
    #[doc = "Bit 4 - Async. Mode: Data Bits 0:8-bits / 1:7-bits"]
    #[inline(always)]
    #[must_use]
    pub fn uc7bit(&mut self) -> UC7BIT_W<4> {
        UC7BIT_W::new(self)
    }
    #[doc = "Bit 5 - Async. Mode: MSB first 0:LSB / 1:MSB"]
    #[inline(always)]
    #[must_use]
    pub fn ucmsb(&mut self) -> UCMSB_W<5> {
        UCMSB_W::new(self)
    }
    #[doc = "Bit 6 - Async. Mode: Parity 0:odd / 1:even"]
    #[inline(always)]
    #[must_use]
    pub fn ucpar(&mut self) -> UCPAR_W<6> {
        UCPAR_W::new(self)
    }
    #[doc = "Bit 7 - Async. Mode: Parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucpen(&mut self) -> UCPEN_W<7> {
        UCPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USCI A1 Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uca1ctl0](index.html) module"]
pub struct UCA1CTL0_SPEC;
impl crate::RegisterSpec for UCA1CTL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uca1ctl0::R](R) reader structure"]
impl crate::Readable for UCA1CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uca1ctl0::W](W) writer structure"]
impl crate::Writable for UCA1CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UCA1CTL0 to value 0"]
impl crate::Resettable for UCA1CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
