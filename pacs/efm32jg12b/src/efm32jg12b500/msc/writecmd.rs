#[doc = "Register `WRITECMD` writer"]
pub struct W(crate::W<WRITECMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECMD_SPEC>;
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
impl From<crate::W<WRITECMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LADDRIM` writer - Load MSC_ADDRB Into ADDR"]
pub type LADDRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub type ERASEPAGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub type WRITEEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `WRITEONCE` writer - Word Write-Once Trigger"]
pub type WRITEONCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `WRITETRIG` writer - Word Write Sequence Trigger"]
pub type WRITETRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `ERASEABORT` writer - Abort Erase Sequence"]
pub type ERASEABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `ERASEMAIN0` writer - Mass Erase Region 0"]
pub type ERASEMAIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `ERASEMAIN1` writer - Mass Erase Region 1"]
pub type ERASEMAIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
#[doc = "Field `CLEARWDATA` writer - Clear WDATA State"]
pub type CLEARWDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRITECMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Load MSC_ADDRB Into ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn laddrim(&mut self) -> LADDRIM_W<0> {
        LADDRIM_W::new(self)
    }
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    #[must_use]
    pub fn erasepage(&mut self) -> ERASEPAGE_W<1> {
        ERASEPAGE_W::new(self)
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn writeend(&mut self) -> WRITEEND_W<2> {
        WRITEEND_W::new(self)
    }
    #[doc = "Bit 3 - Word Write-Once Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writeonce(&mut self) -> WRITEONCE_W<3> {
        WRITEONCE_W::new(self)
    }
    #[doc = "Bit 4 - Word Write Sequence Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writetrig(&mut self) -> WRITETRIG_W<4> {
        WRITETRIG_W::new(self)
    }
    #[doc = "Bit 5 - Abort Erase Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eraseabort(&mut self) -> ERASEABORT_W<5> {
        ERASEABORT_W::new(self)
    }
    #[doc = "Bit 8 - Mass Erase Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn erasemain0(&mut self) -> ERASEMAIN0_W<8> {
        ERASEMAIN0_W::new(self)
    }
    #[doc = "Bit 9 - Mass Erase Region 1"]
    #[inline(always)]
    #[must_use]
    pub fn erasemain1(&mut self) -> ERASEMAIN1_W<9> {
        ERASEMAIN1_W::new(self)
    }
    #[doc = "Bit 12 - Clear WDATA State"]
    #[inline(always)]
    #[must_use]
    pub fn clearwdata(&mut self) -> CLEARWDATA_W<12> {
        CLEARWDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writecmd](index.html) module"]
pub struct WRITECMD_SPEC;
impl crate::RegisterSpec for WRITECMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [writecmd::W](W) writer structure"]
impl crate::Writable for WRITECMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WRITECMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
