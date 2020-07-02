#[doc = "Reader of register SW_CLK_MASK"]
pub type R = crate::R<u32, super::SW_CLK_MASK>;
#[doc = "Writer for register SW_CLK_MASK"]
pub type W = crate::W<u32, super::SW_CLK_MASK>;
#[doc = "Register SW_CLK_MASK `reset()`'s with value 0x7e"]
impl crate::ResetValue for super::SW_CLK_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7e
    }
}
#[doc = "Reader of field `soft_pmu_clk_gt_mask`"]
pub type SOFT_PMU_CLK_GT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_pmu_clk_gt_mask`"]
pub struct SOFT_PMU_CLK_GT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_PMU_CLK_GT_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `soft_sdioahb_clk_gt_mask`"]
pub type SOFT_SDIOAHB_CLK_GT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_sdioahb_clk_gt_mask`"]
pub struct SOFT_SDIOAHB_CLK_GT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_SDIOAHB_CLK_GT_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `soft_cpu_clk_gt_mask`"]
pub type SOFT_CPU_CLK_GT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_cpu_clk_gt_mask`"]
pub struct SOFT_CPU_CLK_GT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_CPU_CLK_GT_MASK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The clock output from the pll is followed by a gating unit, using this register configuration to indicate whether it is allowed to be shut down by the PMU."]
    #[inline(always)]
    pub fn soft_pmu_clk_gt_mask(&self) -> SOFT_PMU_CLK_GT_MASK_R {
        SOFT_PMU_CLK_GT_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the clock supplied to the sdio ahb clock domain can be turned off adaptively."]
    #[inline(always)]
    pub fn soft_sdioahb_clk_gt_mask(&self) -> SOFT_SDIOAHB_CLK_GT_MASK_R {
        SOFT_SDIOAHB_CLK_GT_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates whether the clock supplied to the CPU clock domain (including CPU, bus1, ROM and SRAM) can be turned off adaptively."]
    #[inline(always)]
    pub fn soft_cpu_clk_gt_mask(&self) -> SOFT_CPU_CLK_GT_MASK_R {
        SOFT_CPU_CLK_GT_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The clock output from the pll is followed by a gating unit, using this register configuration to indicate whether it is allowed to be shut down by the PMU."]
    #[inline(always)]
    pub fn soft_pmu_clk_gt_mask(&mut self) -> SOFT_PMU_CLK_GT_MASK_W {
        SOFT_PMU_CLK_GT_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Indicates whether the clock supplied to the sdio ahb clock domain can be turned off adaptively."]
    #[inline(always)]
    pub fn soft_sdioahb_clk_gt_mask(&mut self) -> SOFT_SDIOAHB_CLK_GT_MASK_W {
        SOFT_SDIOAHB_CLK_GT_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Indicates whether the clock supplied to the CPU clock domain (including CPU, bus1, ROM and SRAM) can be turned off adaptively."]
    #[inline(always)]
    pub fn soft_cpu_clk_gt_mask(&mut self) -> SOFT_CPU_CLK_GT_MASK_W {
        SOFT_CPU_CLK_GT_MASK_W { w: self }
    }
}
