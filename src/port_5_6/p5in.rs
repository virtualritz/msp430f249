#[doc = "Register `P5IN` reader"]
pub struct R(crate::R<P5IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P5IN` writer"]
pub struct W(crate::W<P5IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P5IN_SPEC>;
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
impl From<crate::W<P5IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P5IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0` reader - P0"]
pub type P0_R = crate::BitReader;
#[doc = "Field `P0` writer - P0"]
pub type P0_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
#[doc = "Field `P1` reader - P1"]
pub type P1_R = crate::BitReader;
#[doc = "Field `P1` writer - P1"]
pub type P1_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
#[doc = "Field `P2` reader - P2"]
pub type P2_R = crate::BitReader;
#[doc = "Field `P2` writer - P2"]
pub type P2_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
#[doc = "Field `P3` reader - P3"]
pub type P3_R = crate::BitReader;
#[doc = "Field `P3` writer - P3"]
pub type P3_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
#[doc = "Field `P4` reader - P4"]
pub type P4_R = crate::BitReader;
#[doc = "Field `P4` writer - P4"]
pub type P4_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
#[doc = "Field `P5` reader - P5"]
pub type P5_R = crate::BitReader;
#[doc = "Field `P5` writer - P5"]
pub type P5_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
#[doc = "Field `P6` reader - P6"]
pub type P6_R = crate::BitReader;
#[doc = "Field `P6` writer - P6"]
pub type P6_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
#[doc = "Field `P7` reader - P7"]
pub type P7_R = crate::BitReader;
#[doc = "Field `P7` writer - P7"]
pub type P7_W<'a, const O: u8> = crate::BitWriter<'a, P5IN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P0"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - P1"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - P2"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - P3"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - P4"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - P5"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - P6"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - P7"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<7> {
        P7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Port 5 Input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5in](index.html) module"]
pub struct P5IN_SPEC;
impl crate::RegisterSpec for P5IN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p5in::R](R) reader structure"]
impl crate::Readable for P5IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p5in::W](W) writer structure"]
impl crate::Writable for P5IN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets P5IN to value 0"]
impl crate::Resettable for P5IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
