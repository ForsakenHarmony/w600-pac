#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Register"]
    pub data: DATA,
    #[doc = "0x04 - Data enable register"]
    pub data_en: DATA_EN,
    #[doc = "0x08 - Direction control register"]
    pub dir: DIR,
    #[doc = "0x0c - Up and down control register"]
    pub pull_en: PULL_EN,
    #[doc = "0x10 - Multiplex select register"]
    pub af_sel: AF_SEL,
    #[doc = "0x14 - Multiplex select register 1"]
    pub sf_s1: SF_S1,
    #[doc = "0x18 - Multiplex select register 0"]
    pub af_s0: AF_S0,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Interrupt trigger mode configuration register"]
    pub is: IS,
    #[doc = "0x24 - Interrupt edge trigger mode configuration register"]
    pub ibe: IBE,
    #[doc = "0x28 - Interrupt upper and lower edge trigger configuration register"]
    pub iev: IEV,
    #[doc = "0x2c - Interrupt enable configuration register"]
    pub ie: IE,
    #[doc = "0x30 - Bare interrupt status register"]
    pub ris: RIS,
    _reserved12: [u8; 4usize],
    #[doc = "0x38 - Interrupt clear control register"]
    pub ic: IC,
    _reserved13: [u8; 784usize],
    #[doc = "0x34c - Interrupt status register after masking"]
    pub mis: MIS,
}
#[doc = "Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Data Register"]
pub mod data;
#[doc = "Data enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_en](data_en) module"]
pub type DATA_EN = crate::Reg<u32, _DATA_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_EN;
#[doc = "`read()` method returns [data_en::R](data_en::R) reader structure"]
impl crate::Readable for DATA_EN {}
#[doc = "`write(|w| ..)` method takes [data_en::W](data_en::W) writer structure"]
impl crate::Writable for DATA_EN {}
#[doc = "Data enable register"]
pub mod data_en;
#[doc = "Direction control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "Direction control register"]
pub mod dir;
#[doc = "Up and down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pull_en](pull_en) module"]
pub type PULL_EN = crate::Reg<u32, _PULL_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PULL_EN;
#[doc = "`read()` method returns [pull_en::R](pull_en::R) reader structure"]
impl crate::Readable for PULL_EN {}
#[doc = "`write(|w| ..)` method takes [pull_en::W](pull_en::W) writer structure"]
impl crate::Writable for PULL_EN {}
#[doc = "Up and down control register"]
pub mod pull_en;
#[doc = "Multiplex select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af_sel](af_sel) module"]
pub type AF_SEL = crate::Reg<u32, _AF_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AF_SEL;
#[doc = "`read()` method returns [af_sel::R](af_sel::R) reader structure"]
impl crate::Readable for AF_SEL {}
#[doc = "`write(|w| ..)` method takes [af_sel::W](af_sel::W) writer structure"]
impl crate::Writable for AF_SEL {}
#[doc = "Multiplex select register"]
pub mod af_sel;
#[doc = "Multiplex select register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_s1](sf_s1) module"]
pub type SF_S1 = crate::Reg<u32, _SF_S1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SF_S1;
#[doc = "`read()` method returns [sf_s1::R](sf_s1::R) reader structure"]
impl crate::Readable for SF_S1 {}
#[doc = "`write(|w| ..)` method takes [sf_s1::W](sf_s1::W) writer structure"]
impl crate::Writable for SF_S1 {}
#[doc = "Multiplex select register 1"]
pub mod sf_s1;
#[doc = "Multiplex select register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af_s0](af_s0) module"]
pub type AF_S0 = crate::Reg<u32, _AF_S0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AF_S0;
#[doc = "`read()` method returns [af_s0::R](af_s0::R) reader structure"]
impl crate::Readable for AF_S0 {}
#[doc = "`write(|w| ..)` method takes [af_s0::W](af_s0::W) writer structure"]
impl crate::Writable for AF_S0 {}
#[doc = "Multiplex select register 0"]
pub mod af_s0;
#[doc = "Interrupt trigger mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is](is) module"]
pub type IS = crate::Reg<u32, _IS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IS;
#[doc = "`read()` method returns [is::R](is::R) reader structure"]
impl crate::Readable for IS {}
#[doc = "`write(|w| ..)` method takes [is::W](is::W) writer structure"]
impl crate::Writable for IS {}
#[doc = "Interrupt trigger mode configuration register"]
pub mod is;
#[doc = "Interrupt edge trigger mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibe](ibe) module"]
pub type IBE = crate::Reg<u32, _IBE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IBE;
#[doc = "`read()` method returns [ibe::R](ibe::R) reader structure"]
impl crate::Readable for IBE {}
#[doc = "`write(|w| ..)` method takes [ibe::W](ibe::W) writer structure"]
impl crate::Writable for IBE {}
#[doc = "Interrupt edge trigger mode configuration register"]
pub mod ibe;
#[doc = "Interrupt upper and lower edge trigger configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iev](iev) module"]
pub type IEV = crate::Reg<u32, _IEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEV;
#[doc = "`read()` method returns [iev::R](iev::R) reader structure"]
impl crate::Readable for IEV {}
#[doc = "`write(|w| ..)` method takes [iev::W](iev::W) writer structure"]
impl crate::Writable for IEV {}
#[doc = "Interrupt upper and lower edge trigger configuration register"]
pub mod iev;
#[doc = "Interrupt enable configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u32, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "Interrupt enable configuration register"]
pub mod ie;
#[doc = "Bare interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Bare interrupt status register"]
pub mod ris;
#[doc = "Interrupt status register after masking\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "Interrupt status register after masking"]
pub mod mis;
#[doc = "Interrupt clear control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic](ic) module"]
pub type IC = crate::Reg<u32, _IC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC;
#[doc = "`write(|w| ..)` method takes [ic::W](ic::W) writer structure"]
impl crate::Writable for IC {}
#[doc = "Interrupt clear control register"]
pub mod ic;
