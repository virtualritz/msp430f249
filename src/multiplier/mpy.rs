#[doc = "Register `MPY` reader"]
pub struct R(crate::R<MPY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPY` writer"]
pub struct W(crate::W<MPY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPY_SPEC>;
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
impl From<crate::W<MPY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPY_SPEC>) -> Self {
        W(writer)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MPY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multiply Unsigned/Operand 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpy](index.html) module"]
pub struct MPY_SPEC;
impl crate::RegisterSpec for MPY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mpy::R](R) reader structure"]
impl crate::Readable for MPY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpy::W](W) writer structure"]
impl crate::Writable for MPY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPY to value 0"]
impl crate::Resettable for MPY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
