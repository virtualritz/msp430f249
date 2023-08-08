#[doc = "Register `OP2` reader"]
pub struct R(crate::R<OP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OP2` writer"]
pub struct W(crate::W<OP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OP2_SPEC>;
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
impl From<crate::W<OP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OP2_SPEC>) -> Self {
        W(writer)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<OP2_SPEC> {
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
#[doc = "Operand 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [op2](index.html) module"]
pub struct OP2_SPEC;
impl crate::RegisterSpec for OP2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [op2::R](R) reader structure"]
impl crate::Readable for OP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [op2::W](W) writer structure"]
impl crate::Writable for OP2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OP2 to value 0"]
impl crate::Resettable for OP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
