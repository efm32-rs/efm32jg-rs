#[doc = "Register `DBGCLKSEL` reader"]
pub struct R(crate::R<DBGCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGCLKSEL` writer"]
pub struct W(crate::W<DBGCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGCLKSEL_SPEC>;
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
impl From<crate::W<DBGCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG` reader - Debug Trace Clock"]
pub type DBG_R = crate::BitReader<DBG_A>;
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_A {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    AUXHFRCO = 0,
    #[doc = "1: HFCLK is the debug trace clock"]
    HFCLK = 1,
}
impl From<DBG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_A {
        match self.bits {
            false => DBG_A::AUXHFRCO,
            true => DBG_A::HFCLK,
        }
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG_A::HFCLK
    }
}
#[doc = "Field `DBG` writer - Debug Trace Clock"]
pub type DBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGCLKSEL_SPEC, DBG_A, O>;
impl<'a, const O: u8> DBG_W<'a, O> {
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DBG_A::AUXHFRCO)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DBG_A::HFCLK)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DBG_W<0> {
        DBG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Trace Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgclksel](index.html) module"]
pub struct DBGCLKSEL_SPEC;
impl crate::RegisterSpec for DBGCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgclksel::R](R) reader structure"]
impl crate::Readable for DBGCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgclksel::W](W) writer structure"]
impl crate::Writable for DBGCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGCLKSEL to value 0"]
impl crate::Resettable for DBGCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
