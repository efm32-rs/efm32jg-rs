#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WARMUPMODE` reader - Warm-up Mode"]
pub type WARMUPMODE_R = crate::FieldReader<u8, WARMUPMODE_A>;
#[doc = "Warm-up Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WARMUPMODE_A {
    #[doc = "0: ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    NORMAL = 0,
    #[doc = "1: ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSTANDBY = 1,
    #[doc = "2: ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSLOWACC = 2,
    #[doc = "3: ADC is kept on after conversions, allowing for continuous conversion."]
    KEEPADCWARM = 3,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        variant as _
    }
}
impl WARMUPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMUPMODE_A {
        match self.bits {
            0 => WARMUPMODE_A::NORMAL,
            1 => WARMUPMODE_A::KEEPINSTANDBY,
            2 => WARMUPMODE_A::KEEPINSLOWACC,
            3 => WARMUPMODE_A::KEEPADCWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPINSTANDBY`"]
    #[inline(always)]
    pub fn is_keepinstandby(&self) -> bool {
        *self == WARMUPMODE_A::KEEPINSTANDBY
    }
    #[doc = "Checks if the value of the field is `KEEPINSLOWACC`"]
    #[inline(always)]
    pub fn is_keepinslowacc(&self) -> bool {
        *self == WARMUPMODE_A::KEEPINSLOWACC
    }
    #[doc = "Checks if the value of the field is `KEEPADCWARM`"]
    #[inline(always)]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPADCWARM
    }
}
#[doc = "Field `WARMUPMODE` writer - Warm-up Mode"]
pub type WARMUPMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, WARMUPMODE_A, 2, O>;
impl<'a, const O: u8> WARMUPMODE_W<'a, O> {
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn keepinstandby(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPINSTANDBY)
    }
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn keepinslowacc(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPINSLOWACC)
    }
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    #[inline(always)]
    pub fn keepadcwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPADCWARM)
    }
}
#[doc = "Field `SINGLEDMAWU` reader - SINGLEFIFO DMA Wakeup"]
pub type SINGLEDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEDMAWU` writer - SINGLEFIFO DMA Wakeup"]
pub type SINGLEDMAWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SCANDMAWU` reader - SCANFIFO DMA Wakeup"]
pub type SCANDMAWU_R = crate::BitReader<bool>;
#[doc = "Field `SCANDMAWU` writer - SCANFIFO DMA Wakeup"]
pub type SCANDMAWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TAILGATE` reader - Conversion Tailgating"]
pub type TAILGATE_R = crate::BitReader<bool>;
#[doc = "Field `TAILGATE` writer - Conversion Tailgating"]
pub type TAILGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ASYNCCLKEN` reader - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
pub type ASYNCCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `ASYNCCLKEN` writer - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
pub type ASYNCCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ADCCLKMODE` reader - ADC Clock Mode"]
pub type ADCCLKMODE_R = crate::BitReader<bool>;
#[doc = "Field `ADCCLKMODE` writer - ADC Clock Mode"]
pub type ADCCLKMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PRESC` reader - Prescalar Setting for ADC Sample and Conversion Clock"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "Prescalar Setting for ADC Sample and Conversion Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Field `PRESC` writer - Prescalar Setting for ADC Sample and Conversion Clock"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PRESC_A, 7, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
}
#[doc = "Field `TIMEBASE` reader - 1us Time Base"]
pub type TIMEBASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEBASE` writer - 1us Time Base"]
pub type TIMEBASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `OVSRSEL` reader - Oversample Rate Select"]
pub type OVSRSEL_R = crate::FieldReader<u8, OVSRSEL_A>;
#[doc = "Oversample Rate Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSRSEL_A {
    #[doc = "0: 2 samples for each conversion result"]
    X2 = 0,
    #[doc = "1: 4 samples for each conversion result"]
    X4 = 1,
    #[doc = "2: 8 samples for each conversion result"]
    X8 = 2,
    #[doc = "3: 16 samples for each conversion result"]
    X16 = 3,
    #[doc = "4: 32 samples for each conversion result"]
    X32 = 4,
    #[doc = "5: 64 samples for each conversion result"]
    X64 = 5,
    #[doc = "6: 128 samples for each conversion result"]
    X128 = 6,
    #[doc = "7: 256 samples for each conversion result"]
    X256 = 7,
    #[doc = "8: 512 samples for each conversion result"]
    X512 = 8,
    #[doc = "9: 1024 samples for each conversion result"]
    X1024 = 9,
    #[doc = "10: 2048 samples for each conversion result"]
    X2048 = 10,
    #[doc = "11: 4096 samples for each conversion result"]
    X4096 = 11,
}
impl From<OVSRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSRSEL_A) -> Self {
        variant as _
    }
}
impl OVSRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVSRSEL_A> {
        match self.bits {
            0 => Some(OVSRSEL_A::X2),
            1 => Some(OVSRSEL_A::X4),
            2 => Some(OVSRSEL_A::X8),
            3 => Some(OVSRSEL_A::X16),
            4 => Some(OVSRSEL_A::X32),
            5 => Some(OVSRSEL_A::X64),
            6 => Some(OVSRSEL_A::X128),
            7 => Some(OVSRSEL_A::X256),
            8 => Some(OVSRSEL_A::X512),
            9 => Some(OVSRSEL_A::X1024),
            10 => Some(OVSRSEL_A::X2048),
            11 => Some(OVSRSEL_A::X4096),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSEL_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSEL_A::X4
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSEL_A::X8
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSEL_A::X16
    }
    #[doc = "Checks if the value of the field is `X32`"]
    #[inline(always)]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSEL_A::X32
    }
    #[doc = "Checks if the value of the field is `X64`"]
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSEL_A::X64
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSEL_A::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSEL_A::X256
    }
    #[doc = "Checks if the value of the field is `X512`"]
    #[inline(always)]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSEL_A::X512
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSEL_A::X1024
    }
    #[doc = "Checks if the value of the field is `X2048`"]
    #[inline(always)]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSEL_A::X2048
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSEL_A::X4096
    }
}
#[doc = "Field `OVSRSEL` writer - Oversample Rate Select"]
pub type OVSRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, OVSRSEL_A, 4, O>;
impl<'a, const O: u8> OVSRSEL_W<'a, O> {
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn x32(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn x64(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn x512(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn x2048(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X4096)
    }
}
#[doc = "Field `CHCONMODE` reader - Channel Connect"]
pub type CHCONMODE_R = crate::BitReader<bool>;
#[doc = "Field `CHCONMODE` writer - Channel Connect"]
pub type CHCONMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn singledmawu(&self) -> SINGLEDMAWU_R {
        SINGLEDMAWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn scandmawu(&self) -> SCANDMAWU_R {
        SCANDMAWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&self) -> TAILGATE_R {
        TAILGATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline(always)]
    pub fn asyncclken(&self) -> ASYNCCLKEN_R {
        ASYNCCLKEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline(always)]
    pub fn adcclkmode(&self) -> ADCCLKMODE_R {
        ADCCLKMODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&self) -> OVSRSEL_R {
        OVSRSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline(always)]
    pub fn chconmode(&self) -> CHCONMODE_R {
        CHCONMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    #[must_use]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W<0> {
        WARMUPMODE_W::new(self)
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn singledmawu(&mut self) -> SINGLEDMAWU_W<2> {
        SINGLEDMAWU_W::new(self)
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn scandmawu(&mut self) -> SCANDMAWU_W<3> {
        SCANDMAWU_W::new(self)
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline(always)]
    #[must_use]
    pub fn tailgate(&mut self) -> TAILGATE_W<4> {
        TAILGATE_W::new(self)
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline(always)]
    #[must_use]
    pub fn asyncclken(&mut self) -> ASYNCCLKEN_W<6> {
        ASYNCCLKEN_W::new(self)
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adcclkmode(&mut self) -> ADCCLKMODE_W<7> {
        ADCCLKMODE_W::new(self)
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<8> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline(always)]
    #[must_use]
    pub fn timebase(&mut self) -> TIMEBASE_W<16> {
        TIMEBASE_W::new(self)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    #[must_use]
    pub fn ovsrsel(&mut self) -> OVSRSEL_W<24> {
        OVSRSEL_W::new(self)
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline(always)]
    #[must_use]
    pub fn chconmode(&mut self) -> CHCONMODE_W<29> {
        CHCONMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x001f_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x001f_0000;
}
