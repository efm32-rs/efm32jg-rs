#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CD` reader - CH0CD Interrupt Enable"]
pub type CH0CD_R = crate::BitReader<bool>;
#[doc = "Field `CH0CD` writer - CH0CD Interrupt Enable"]
pub type CH0CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1CD` reader - CH1CD Interrupt Enable"]
pub type CH1CD_R = crate::BitReader<bool>;
#[doc = "Field `CH1CD` writer - CH1CD Interrupt Enable"]
pub type CH1CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH0OF` reader - CH0OF Interrupt Enable"]
pub type CH0OF_R = crate::BitReader<bool>;
#[doc = "Field `CH0OF` writer - CH0OF Interrupt Enable"]
pub type CH0OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1OF` reader - CH1OF Interrupt Enable"]
pub type CH1OF_R = crate::BitReader<bool>;
#[doc = "Field `CH1OF` writer - CH1OF Interrupt Enable"]
pub type CH1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH0UF` reader - CH0UF Interrupt Enable"]
pub type CH0UF_R = crate::BitReader<bool>;
#[doc = "Field `CH0UF` writer - CH0UF Interrupt Enable"]
pub type CH0UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1UF` reader - CH1UF Interrupt Enable"]
pub type CH1UF_R = crate::BitReader<bool>;
#[doc = "Field `CH1UF` writer - CH1UF Interrupt Enable"]
pub type CH1UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH0BL` reader - CH0BL Interrupt Enable"]
pub type CH0BL_R = crate::BitReader<bool>;
#[doc = "Field `CH0BL` writer - CH0BL Interrupt Enable"]
pub type CH0BL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1BL` reader - CH1BL Interrupt Enable"]
pub type CH1BL_R = crate::BitReader<bool>;
#[doc = "Field `CH1BL` writer - CH1BL Interrupt Enable"]
pub type CH1BL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM23ERR` reader - EM23ERR Interrupt Enable"]
pub type EM23ERR_R = crate::BitReader<bool>;
#[doc = "Field `EM23ERR` writer - EM23ERR Interrupt Enable"]
pub type EM23ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA0APORTCONFLICT` reader - OPA0APORTCONFLICT Interrupt Enable"]
pub type OPA0APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA0APORTCONFLICT` writer - OPA0APORTCONFLICT Interrupt Enable"]
pub type OPA0APORTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA1APORTCONFLICT` reader - OPA1APORTCONFLICT Interrupt Enable"]
pub type OPA1APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA1APORTCONFLICT` writer - OPA1APORTCONFLICT Interrupt Enable"]
pub type OPA1APORTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA2APORTCONFLICT` reader - OPA2APORTCONFLICT Interrupt Enable"]
pub type OPA2APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA2APORTCONFLICT` writer - OPA2APORTCONFLICT Interrupt Enable"]
pub type OPA2APORTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA0PRSTIMEDERR` reader - OPA0PRSTIMEDERR Interrupt Enable"]
pub type OPA0PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA0PRSTIMEDERR` writer - OPA0PRSTIMEDERR Interrupt Enable"]
pub type OPA0PRSTIMEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA1PRSTIMEDERR` reader - OPA1PRSTIMEDERR Interrupt Enable"]
pub type OPA1PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA1PRSTIMEDERR` writer - OPA1PRSTIMEDERR Interrupt Enable"]
pub type OPA1PRSTIMEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA2PRSTIMEDERR` reader - OPA2PRSTIMEDERR Interrupt Enable"]
pub type OPA2PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA2PRSTIMEDERR` writer - OPA2PRSTIMEDERR Interrupt Enable"]
pub type OPA2PRSTIMEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA0OUTVALID` reader - OPA0OUTVALID Interrupt Enable"]
pub type OPA0OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA0OUTVALID` writer - OPA0OUTVALID Interrupt Enable"]
pub type OPA0OUTVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA1OUTVALID` reader - OPA1OUTVALID Interrupt Enable"]
pub type OPA1OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA1OUTVALID` writer - OPA1OUTVALID Interrupt Enable"]
pub type OPA1OUTVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `OPA2OUTVALID` reader - OPA2OUTVALID Interrupt Enable"]
pub type OPA2OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA2OUTVALID` writer - OPA2OUTVALID Interrupt Enable"]
pub type OPA2OUTVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch0cd(&self) -> CH0CD_R {
        CH0CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch1cd(&self) -> CH1CD_R {
        CH1CD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch0bl(&self) -> CH0BL_R {
        CH0BL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch1bl(&self) -> CH1BL_R {
        CH1BL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> OPA0APORTCONFLICT_R {
        OPA0APORTCONFLICT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> OPA1APORTCONFLICT_R {
        OPA1APORTCONFLICT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> OPA2APORTCONFLICT_R {
        OPA2APORTCONFLICT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa0prstimederr(&self) -> OPA0PRSTIMEDERR_R {
        OPA0PRSTIMEDERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa1prstimederr(&self) -> OPA1PRSTIMEDERR_R {
        OPA1PRSTIMEDERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa2prstimederr(&self) -> OPA2PRSTIMEDERR_R {
        OPA2PRSTIMEDERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> OPA0OUTVALID_R {
        OPA0OUTVALID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> OPA1OUTVALID_R {
        OPA1OUTVALID_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> OPA2OUTVALID_R {
        OPA2OUTVALID_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cd(&mut self) -> CH0CD_W<0> {
        CH0CD_W::new(self)
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cd(&mut self) -> CH1CD_W<1> {
        CH1CD_W::new(self)
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<2> {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<3> {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0uf(&mut self) -> CH0UF_W<4> {
        CH0UF_W::new(self)
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1uf(&mut self) -> CH1UF_W<5> {
        CH1UF_W::new(self)
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0bl(&mut self) -> CH0BL_W<6> {
        CH0BL_W::new(self)
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1bl(&mut self) -> CH1BL_W<7> {
        CH1BL_W::new(self)
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23err(&mut self) -> EM23ERR_W<15> {
        EM23ERR_W::new(self)
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0aportconflict(&mut self) -> OPA0APORTCONFLICT_W<16> {
        OPA0APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1aportconflict(&mut self) -> OPA1APORTCONFLICT_W<17> {
        OPA1APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2aportconflict(&mut self) -> OPA2APORTCONFLICT_W<18> {
        OPA2APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0prstimederr(&mut self) -> OPA0PRSTIMEDERR_W<20> {
        OPA0PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1prstimederr(&mut self) -> OPA1PRSTIMEDERR_W<21> {
        OPA1PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2prstimederr(&mut self) -> OPA2PRSTIMEDERR_W<22> {
        OPA2PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0outvalid(&mut self) -> OPA0OUTVALID_W<28> {
        OPA0OUTVALID_W::new(self)
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1outvalid(&mut self) -> OPA1OUTVALID_W<29> {
        OPA1OUTVALID_W::new(self)
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2outvalid(&mut self) -> OPA2OUTVALID_W<30> {
        OPA2OUTVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
