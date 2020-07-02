#[doc = "Reader of register PULL_EN"]
pub type R = crate::R<u32, super::PULL_EN>;
#[doc = "Writer for register PULL_EN"]
pub type W = crate::W<u32, super::PULL_EN>;
#[doc = "Register PULL_EN `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::PULL_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
impl R {}
impl W {}
