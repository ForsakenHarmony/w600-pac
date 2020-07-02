#[doc = "Reader of register SW_RST_CTRL"]
pub type R = crate::R<u32, super::SW_RST_CTRL>;
#[doc = "Writer for register SW_RST_CTRL"]
pub type W = crate::W<u32, super::SW_RST_CTRL>;
#[doc = "Register SW_RST_CTRL `reset()`'s with value 0x01ff_ffff"]
impl crate::ResetValue for super::SW_RST_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01ff_ffff
    }
}
impl R {}
impl W {}
