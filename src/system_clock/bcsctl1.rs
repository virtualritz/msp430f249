#[doc = "Register `BCSCTL1` reader"]
pub struct R(crate::R<BCSCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCSCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCSCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCSCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCSCTL1` writer"]
pub struct W(crate::W<BCSCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCSCTL1_SPEC>;
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
impl From<crate::W<BCSCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCSCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSEL0` reader - Range Select Bit 0"]
pub type RSEL0_R = crate::BitReader;
#[doc = "Field `RSEL0` writer - Range Select Bit 0"]
pub type RSEL0_W<'a, const O: u8> = crate::BitWriter<'a, BCSCTL1_SPEC, O>;
#[doc = "Field `RSEL1` reader - Range Select Bit 1"]
pub type RSEL1_R = crate::BitReader;
#[doc = "Field `RSEL1` writer - Range Select Bit 1"]
pub type RSEL1_W<'a, const O: u8> = crate::BitWriter<'a, BCSCTL1_SPEC, O>;
#[doc = "Field `RSEL2` reader - Range Select Bit 2"]
pub type RSEL2_R = crate::BitReader;
#[doc = "Field `RSEL2` writer - Range Select Bit 2"]
pub type RSEL2_W<'a, const O: u8> = crate::BitWriter<'a, BCSCTL1_SPEC, O>;
#[doc = "Field `RSEL3` reader - Range Select Bit 3"]
pub type RSEL3_R = crate::BitReader;
#[doc = "Field `RSEL3` writer - Range Select Bit 3"]
pub type RSEL3_W<'a, const O: u8> = crate::BitWriter<'a, BCSCTL1_SPEC, O>;
#[doc = "Field `DIVA` reader - ACLK Divider 0"]
pub type DIVA_R = crate::FieldReader<DIVA_A>;
#[doc = "ACLK Divider 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: ACLK Divider 0: /1"]
    DIVA_0 = 0,
    #[doc = "1: ACLK Divider 1: /2"]
    DIVA_1 = 1,
    #[doc = "2: ACLK Divider 2: /4"]
    DIVA_2 = 2,
    #[doc = "3: ACLK Divider 3: /8"]
    DIVA_3 = 3,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVA_A {
    type Ux = u8;
}
impl DIVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVA_A {
        match self.bits {
            0 => DIVA_A::DIVA_0,
            1 => DIVA_A::DIVA_1,
            2 => DIVA_A::DIVA_2,
            3 => DIVA_A::DIVA_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVA_0`"]
    #[inline(always)]
    pub fn is_diva_0(&self) -> bool {
        *self == DIVA_A::DIVA_0
    }
    #[doc = "Checks if the value of the field is `DIVA_1`"]
    #[inline(always)]
    pub fn is_diva_1(&self) -> bool {
        *self == DIVA_A::DIVA_1
    }
    #[doc = "Checks if the value of the field is `DIVA_2`"]
    #[inline(always)]
    pub fn is_diva_2(&self) -> bool {
        *self == DIVA_A::DIVA_2
    }
    #[doc = "Checks if the value of the field is `DIVA_3`"]
    #[inline(always)]
    pub fn is_diva_3(&self) -> bool {
        *self == DIVA_A::DIVA_3
    }
}
#[doc = "Field `DIVA` writer - ACLK Divider 0"]
pub type DIVA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, BCSCTL1_SPEC, 2, O, DIVA_A>;
impl<'a, const O: u8> DIVA_W<'a, O> {
    #[doc = "ACLK Divider 0: /1"]
    #[inline(always)]
    pub fn diva_0(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_0)
    }
    #[doc = "ACLK Divider 1: /2"]
    #[inline(always)]
    pub fn diva_1(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_1)
    }
    #[doc = "ACLK Divider 2: /4"]
    #[inline(always)]
    pub fn diva_2(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_2)
    }
    #[doc = "ACLK Divider 3: /8"]
    #[inline(always)]
    pub fn diva_3(self) -> &'a mut W {
        self.variant(DIVA_A::DIVA_3)
    }
}
#[doc = "Field `XTS` reader - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XTS_R = crate::BitReader;
#[doc = "Field `XTS` writer - LFXTCLK 0:Low Freq. / 1: High Freq."]
pub type XTS_W<'a, const O: u8> = crate::BitWriter<'a, BCSCTL1_SPEC, O>;
#[doc = "Field `XT2OFF` reader - Enable XT2CLK"]
pub type XT2OFF_R = crate::BitReader;
#[doc = "Field `XT2OFF` writer - Enable XT2CLK"]
pub type XT2OFF_W<'a, const O: u8> = crate::BitWriter<'a, BCSCTL1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Range Select Bit 0"]
    #[inline(always)]
    pub fn rsel0(&self) -> RSEL0_R {
        RSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Range Select Bit 1"]
    #[inline(always)]
    pub fn rsel1(&self) -> RSEL1_R {
        RSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Range Select Bit 2"]
    #[inline(always)]
    pub fn rsel2(&self) -> RSEL2_R {
        RSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Range Select Bit 3"]
    #[inline(always)]
    pub fn rsel3(&self) -> RSEL3_R {
        RSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    pub fn xts(&self) -> XTS_R {
        XTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    pub fn xt2off(&self) -> XT2OFF_R {
        XT2OFF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Range Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rsel0(&mut self) -> RSEL0_W<0> {
        RSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Range Select Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rsel1(&mut self) -> RSEL1_W<1> {
        RSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Range Select Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rsel2(&mut self) -> RSEL2_W<2> {
        RSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Range Select Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rsel3(&mut self) -> RSEL3_W<3> {
        RSEL3_W::new(self)
    }
    #[doc = "Bits 4:5 - ACLK Divider 0"]
    #[inline(always)]
    #[must_use]
    pub fn diva(&mut self) -> DIVA_W<4> {
        DIVA_W::new(self)
    }
    #[doc = "Bit 6 - LFXTCLK 0:Low Freq. / 1: High Freq."]
    #[inline(always)]
    #[must_use]
    pub fn xts(&mut self) -> XTS_W<6> {
        XTS_W::new(self)
    }
    #[doc = "Bit 7 - Enable XT2CLK"]
    #[inline(always)]
    #[must_use]
    pub fn xt2off(&mut self) -> XT2OFF_W<7> {
        XT2OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Basic Clock System Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcsctl1](index.html) module"]
pub struct BCSCTL1_SPEC;
impl crate::RegisterSpec for BCSCTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bcsctl1::R](R) reader structure"]
impl crate::Readable for BCSCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcsctl1::W](W) writer structure"]
impl crate::Writable for BCSCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCSCTL1 to value 0"]
impl crate::Resettable for BCSCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
