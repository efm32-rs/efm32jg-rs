#[doc = "Register `READCTRL` reader"]
pub struct R(crate::R<READCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READCTRL` writer"]
pub struct W(crate::W<READCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READCTRL_SPEC>;
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
impl From<crate::W<READCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFCDIS` reader - Internal Flash Cache Disable"]
pub type IFCDIS_R = crate::BitReader<bool>;
#[doc = "Field `IFCDIS` writer - Internal Flash Cache Disable"]
pub type IFCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `AIDIS` reader - Automatic Invalidate Disable"]
pub type AIDIS_R = crate::BitReader<bool>;
#[doc = "Field `AIDIS` writer - Automatic Invalidate Disable"]
pub type AIDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `ICCDIS` reader - Interrupt Context Cache Disable"]
pub type ICCDIS_R = crate::BitReader<bool>;
#[doc = "Field `ICCDIS` writer - Interrupt Context Cache Disable"]
pub type ICCDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `PREFETCH` reader - Prefetch Mode"]
pub type PREFETCH_R = crate::BitReader<bool>;
#[doc = "Field `PREFETCH` writer - Prefetch Mode"]
pub type PREFETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `USEHPROT` reader - AHB_HPROT Mode"]
pub type USEHPROT_R = crate::BitReader<bool>;
#[doc = "Field `USEHPROT` writer - AHB_HPROT Mode"]
pub type USEHPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Read Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Read Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers"]
    WS0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    WS1 = 1,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::WS0),
            1 => Some(MODE_A::WS1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == MODE_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == MODE_A::WS1
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, READCTRL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(MODE_A::WS0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(MODE_A::WS1)
    }
}
#[doc = "Field `SCBTP` reader - Suppress Conditional Branch Target Perfetch"]
pub type SCBTP_R = crate::BitReader<bool>;
#[doc = "Field `SCBTP` writer - Suppress Conditional Branch Target Perfetch"]
pub type SCBTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, READCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IFCDIS_R {
        IFCDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AIDIS_R {
        AIDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&self) -> ICCDIS_R {
        ICCDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    pub fn prefetch(&self) -> PREFETCH_R {
        PREFETCH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    pub fn usehprot(&self) -> USEHPROT_R {
        USEHPROT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    pub fn scbtp(&self) -> SCBTP_R {
        SCBTP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ifcdis(&mut self) -> IFCDIS_W<3> {
        IFCDIS_W::new(self)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aidis(&mut self) -> AIDIS_W<4> {
        AIDIS_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iccdis(&mut self) -> ICCDIS_W<5> {
        ICCDIS_W::new(self)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prefetch(&mut self) -> PREFETCH_W<8> {
        PREFETCH_W::new(self)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usehprot(&mut self) -> USEHPROT_W<9> {
        USEHPROT_W::new(self)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<24> {
        MODE_W::new(self)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    #[must_use]
    pub fn scbtp(&mut self) -> SCBTP_W<28> {
        SCBTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readctrl](index.html) module"]
pub struct READCTRL_SPEC;
impl crate::RegisterSpec for READCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readctrl::R](R) reader structure"]
impl crate::Readable for READCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readctrl::W](W) writer structure"]
impl crate::Writable for READCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READCTRL to value 0x0100_0100"]
impl crate::Resettable for READCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0100;
}
