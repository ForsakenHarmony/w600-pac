#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software clock gating enable register"]
    pub sw_clkg_en: SW_CLKG_EN,
    #[doc = "0x04 - Software clock mask register"]
    pub sw_clk_mask: SW_CLK_MASK,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Software reset control register"]
    pub sw_rst_ctrl: SW_RST_CTRL,
    #[doc = "0x10 - Clock divider configuration register"]
    pub sys_clk_div: SYS_CLK_DIV,
    #[doc = "0x14 - Debug control register"]
    pub debug_ctrl: DEBUG_CTRL,
    #[doc = "0x18 - I2S clock control register"]
    pub i2s_clk_ctrl: I2S_CLK_CTRL,
}
#[doc = "Software clock gating enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_clkg_en](sw_clkg_en) module"]
pub type SW_CLKG_EN = crate::Reg<u32, _SW_CLKG_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_CLKG_EN;
#[doc = "`read()` method returns [sw_clkg_en::R](sw_clkg_en::R) reader structure"]
impl crate::Readable for SW_CLKG_EN {}
#[doc = "`write(|w| ..)` method takes [sw_clkg_en::W](sw_clkg_en::W) writer structure"]
impl crate::Writable for SW_CLKG_EN {}
#[doc = "Software clock gating enable register"]
pub mod sw_clkg_en;
#[doc = "Software clock mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_clk_mask](sw_clk_mask) module"]
pub type SW_CLK_MASK = crate::Reg<u32, _SW_CLK_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_CLK_MASK;
#[doc = "`read()` method returns [sw_clk_mask::R](sw_clk_mask::R) reader structure"]
impl crate::Readable for SW_CLK_MASK {}
#[doc = "`write(|w| ..)` method takes [sw_clk_mask::W](sw_clk_mask::W) writer structure"]
impl crate::Writable for SW_CLK_MASK {}
#[doc = "Software clock mask register"]
pub mod sw_clk_mask;
#[doc = "Software reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sw_rst_ctrl](sw_rst_ctrl) module"]
pub type SW_RST_CTRL = crate::Reg<u32, _SW_RST_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SW_RST_CTRL;
#[doc = "`read()` method returns [sw_rst_ctrl::R](sw_rst_ctrl::R) reader structure"]
impl crate::Readable for SW_RST_CTRL {}
#[doc = "`write(|w| ..)` method takes [sw_rst_ctrl::W](sw_rst_ctrl::W) writer structure"]
impl crate::Writable for SW_RST_CTRL {}
#[doc = "Software reset control register"]
pub mod sw_rst_ctrl;
#[doc = "Clock divider configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_clk_div](sys_clk_div) module"]
pub type SYS_CLK_DIV = crate::Reg<u32, _SYS_CLK_DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_CLK_DIV;
#[doc = "`read()` method returns [sys_clk_div::R](sys_clk_div::R) reader structure"]
impl crate::Readable for SYS_CLK_DIV {}
#[doc = "`write(|w| ..)` method takes [sys_clk_div::W](sys_clk_div::W) writer structure"]
impl crate::Writable for SYS_CLK_DIV {}
#[doc = "Clock divider configuration register"]
pub mod sys_clk_div;
#[doc = "Debug control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_ctrl](debug_ctrl) module"]
pub type DEBUG_CTRL = crate::Reg<u32, _DEBUG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBUG_CTRL;
#[doc = "`read()` method returns [debug_ctrl::R](debug_ctrl::R) reader structure"]
impl crate::Readable for DEBUG_CTRL {}
#[doc = "`write(|w| ..)` method takes [debug_ctrl::W](debug_ctrl::W) writer structure"]
impl crate::Writable for DEBUG_CTRL {}
#[doc = "Debug control register"]
pub mod debug_ctrl;
#[doc = "I2S clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_clk_ctrl](i2s_clk_ctrl) module"]
pub type I2S_CLK_CTRL = crate::Reg<u32, _I2S_CLK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CLK_CTRL;
#[doc = "`read()` method returns [i2s_clk_ctrl::R](i2s_clk_ctrl::R) reader structure"]
impl crate::Readable for I2S_CLK_CTRL {}
#[doc = "`write(|w| ..)` method takes [i2s_clk_ctrl::W](i2s_clk_ctrl::W) writer structure"]
impl crate::Writable for I2S_CLK_CTRL {}
#[doc = "I2S clock control register"]
pub mod i2s_clk_ctrl;
