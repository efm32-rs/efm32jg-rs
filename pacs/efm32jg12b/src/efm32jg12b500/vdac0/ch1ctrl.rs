#[doc = "Register `CH1CTRL` reader"]
pub struct R(crate::R<CH1CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1CTRL` writer"]
pub struct W(crate::W<CH1CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1CTRL_SPEC>;
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
impl From<crate::W<CH1CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONVMODE` reader - Conversion Mode"]
pub type CONVMODE_R = crate::BitReader<bool>;
#[doc = "Field `CONVMODE` writer - Conversion Mode"]
pub type CONVMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1CTRL_SPEC, bool, O>;
#[doc = "Field `TRIGMODE` reader - Channel 1 Trigger Mode"]
pub type TRIGMODE_R = crate::FieldReader<u8, TRIGMODE_A>;
#[doc = "Channel 1 Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGMODE_A {
    #[doc = "0: Channel 1 is triggered by CH1DATA or COMBDATA write"]
    SW = 0,
    #[doc = "1: Channel 1 is triggered by PRS input"]
    PRS = 1,
    #[doc = "2: Channel 1 is triggered by Refresh timer"]
    REFRESH = 2,
    #[doc = "3: Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    SWPRS = 3,
    #[doc = "4: Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    SWREFRESH = 4,
    #[doc = "5: Channel 1 is triggered by LESENSE"]
    LESENSE = 5,
}
impl From<TRIGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGMODE_A) -> Self {
        variant as _
    }
}
impl TRIGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGMODE_A> {
        match self.bits {
            0 => Some(TRIGMODE_A::SW),
            1 => Some(TRIGMODE_A::PRS),
            2 => Some(TRIGMODE_A::REFRESH),
            3 => Some(TRIGMODE_A::SWPRS),
            4 => Some(TRIGMODE_A::SWREFRESH),
            5 => Some(TRIGMODE_A::LESENSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGMODE_A::SW
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == TRIGMODE_A::PRS
    }
    #[doc = "Checks if the value of the field is `REFRESH`"]
    #[inline(always)]
    pub fn is_refresh(&self) -> bool {
        *self == TRIGMODE_A::REFRESH
    }
    #[doc = "Checks if the value of the field is `SWPRS`"]
    #[inline(always)]
    pub fn is_swprs(&self) -> bool {
        *self == TRIGMODE_A::SWPRS
    }
    #[doc = "Checks if the value of the field is `SWREFRESH`"]
    #[inline(always)]
    pub fn is_swrefresh(&self) -> bool {
        *self == TRIGMODE_A::SWREFRESH
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == TRIGMODE_A::LESENSE
    }
}
#[doc = "Field `TRIGMODE` writer - Channel 1 Trigger Mode"]
pub type TRIGMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH1CTRL_SPEC, u8, TRIGMODE_A, 3, O>;
impl<'a, const O: u8> TRIGMODE_W<'a, O> {
    #[doc = "Channel 1 is triggered by CH1DATA or COMBDATA write"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SW)
    }
    #[doc = "Channel 1 is triggered by PRS input"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(TRIGMODE_A::PRS)
    }
    #[doc = "Channel 1 is triggered by Refresh timer"]
    #[inline(always)]
    pub fn refresh(self) -> &'a mut W {
        self.variant(TRIGMODE_A::REFRESH)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or PRS input"]
    #[inline(always)]
    pub fn swprs(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SWPRS)
    }
    #[doc = "Channel 1 is triggered by CH1DATA/COMBDATA write or Refresh timer"]
    #[inline(always)]
    pub fn swrefresh(self) -> &'a mut W {
        self.variant(TRIGMODE_A::SWREFRESH)
    }
    #[doc = "Channel 1 is triggered by LESENSE"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut W {
        self.variant(TRIGMODE_A::LESENSE)
    }
}
#[doc = "Field `PRSASYNC` reader - Channel 1 PRS Asynchronous Enable"]
pub type PRSASYNC_R = crate::BitReader<bool>;
#[doc = "Field `PRSASYNC` writer - Channel 1 PRS Asynchronous Enable"]
pub type PRSASYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH1CTRL_SPEC, bool, O>;
#[doc = "Field `PRSSEL` reader - Channel 1 PRS Trigger Select"]
pub type PRSSEL_R = crate::FieldReader<u8, PRSSEL_A>;
#[doc = "Channel 1 PRS Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRSSEL_A {
    #[doc = "0: PRS ch 0 triggers a conversion."]
    PRSCH0 = 0,
    #[doc = "1: PRS ch 1 triggers a conversion."]
    PRSCH1 = 1,
    #[doc = "2: PRS ch 2 triggers a conversion."]
    PRSCH2 = 2,
    #[doc = "3: PRS ch 3 triggers a conversion."]
    PRSCH3 = 3,
    #[doc = "4: PRS ch 4 triggers a conversion."]
    PRSCH4 = 4,
    #[doc = "5: PRS ch 5 triggers a conversion."]
    PRSCH5 = 5,
    #[doc = "6: PRS ch 6 triggers a conversion."]
    PRSCH6 = 6,
    #[doc = "7: PRS ch 7 triggers a conversion."]
    PRSCH7 = 7,
    #[doc = "8: PRS ch 8 triggers a conversion."]
    PRSCH8 = 8,
    #[doc = "9: PRS ch 9 triggers a conversion."]
    PRSCH9 = 9,
    #[doc = "10: PRS ch 10 triggers a conversion."]
    PRSCH10 = 10,
    #[doc = "11: PRS ch 11 triggers a conversion."]
    PRSCH11 = 11,
}
impl From<PRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PRSSEL_A) -> Self {
        variant as _
    }
}
impl PRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRSSEL_A> {
        match self.bits {
            0 => Some(PRSSEL_A::PRSCH0),
            1 => Some(PRSSEL_A::PRSCH1),
            2 => Some(PRSSEL_A::PRSCH2),
            3 => Some(PRSSEL_A::PRSCH3),
            4 => Some(PRSSEL_A::PRSCH4),
            5 => Some(PRSSEL_A::PRSCH5),
            6 => Some(PRSSEL_A::PRSCH6),
            7 => Some(PRSSEL_A::PRSCH7),
            8 => Some(PRSSEL_A::PRSCH8),
            9 => Some(PRSSEL_A::PRSCH9),
            10 => Some(PRSSEL_A::PRSCH10),
            11 => Some(PRSSEL_A::PRSCH11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == PRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == PRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == PRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == PRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == PRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == PRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == PRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == PRSSEL_A::PRSCH7
    }
    #[doc = "Checks if the value of the field is `PRSCH8`"]
    #[inline(always)]
    pub fn is_prsch8(&self) -> bool {
        *self == PRSSEL_A::PRSCH8
    }
    #[doc = "Checks if the value of the field is `PRSCH9`"]
    #[inline(always)]
    pub fn is_prsch9(&self) -> bool {
        *self == PRSSEL_A::PRSCH9
    }
    #[doc = "Checks if the value of the field is `PRSCH10`"]
    #[inline(always)]
    pub fn is_prsch10(&self) -> bool {
        *self == PRSSEL_A::PRSCH10
    }
    #[doc = "Checks if the value of the field is `PRSCH11`"]
    #[inline(always)]
    pub fn is_prsch11(&self) -> bool {
        *self == PRSSEL_A::PRSCH11
    }
}
#[doc = "Field `PRSSEL` writer - Channel 1 PRS Trigger Select"]
pub type PRSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1CTRL_SPEC, u8, PRSSEL_A, 4, O>;
impl<'a, const O: u8> PRSSEL_W<'a, O> {
    #[doc = "PRS ch 0 triggers a conversion."]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH0)
    }
    #[doc = "PRS ch 1 triggers a conversion."]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH1)
    }
    #[doc = "PRS ch 2 triggers a conversion."]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH2)
    }
    #[doc = "PRS ch 3 triggers a conversion."]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH3)
    }
    #[doc = "PRS ch 4 triggers a conversion."]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH4)
    }
    #[doc = "PRS ch 5 triggers a conversion."]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH5)
    }
    #[doc = "PRS ch 6 triggers a conversion."]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH6)
    }
    #[doc = "PRS ch 7 triggers a conversion."]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH7)
    }
    #[doc = "PRS ch 8 triggers a conversion."]
    #[inline(always)]
    pub fn prsch8(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH8)
    }
    #[doc = "PRS ch 9 triggers a conversion."]
    #[inline(always)]
    pub fn prsch9(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH9)
    }
    #[doc = "PRS ch 10 triggers a conversion."]
    #[inline(always)]
    pub fn prsch10(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH10)
    }
    #[doc = "PRS ch 11 triggers a conversion."]
    #[inline(always)]
    pub fn prsch11(self) -> &'a mut W {
        self.variant(PRSSEL_A::PRSCH11)
    }
}
impl R {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&self) -> CONVMODE_R {
        CONVMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&self) -> TRIGMODE_R {
        TRIGMODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    pub fn prsasync(&self) -> PRSASYNC_R {
        PRSASYNC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    pub fn prssel(&self) -> PRSSEL_R {
        PRSSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Conversion Mode"]
    #[inline(always)]
    #[must_use]
    pub fn convmode(&mut self) -> CONVMODE_W<0> {
        CONVMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trigmode(&mut self) -> TRIGMODE_W<4> {
        TRIGMODE_W::new(self)
    }
    #[doc = "Bit 8 - Channel 1 PRS Asynchronous Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsasync(&mut self) -> PRSASYNC_W<8> {
        PRSASYNC_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 1 PRS Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PRSSEL_W<12> {
        PRSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ctrl](index.html) module"]
pub struct CH1CTRL_SPEC;
impl crate::RegisterSpec for CH1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1ctrl::R](R) reader structure"]
impl crate::Readable for CH1CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1ctrl::W](W) writer structure"]
impl crate::Writable for CH1CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1CTRL to value 0"]
impl crate::Resettable for CH1CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
