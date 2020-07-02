#[doc = "Reader of register DATA_EN"]
pub type R = crate::R<u32, super::DATA_EN>;
#[doc = "Writer for register DATA_EN"]
pub type W = crate::W<u32, super::DATA_EN>;
#[doc = "Register DATA_EN `reset()`'s with value 0x7fff_ffff"]
impl crate::ResetValue for super::DATA_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff_ffff
    }
}
impl R {}
impl W {}
