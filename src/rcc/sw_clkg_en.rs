#[doc = "Reader of register SW_CLKG_EN"]
pub type R = crate::R<u32, super::SW_CLKG_EN>;
#[doc = "Writer for register SW_CLKG_EN"]
pub type W = crate::W<u32, super::SW_CLKG_EN>;
#[doc = "Register SW_CLKG_EN `reset()`'s with value 0x7fff"]
impl crate::ResetValue for super::SW_CLKG_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff
    }
}
#[doc = "Reader of field `soft_7816_gate_en`"]
pub type SOFT_7816_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_7816_gate_en`"]
pub struct SOFT_7816_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_7816_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `soft_gpsec_gate_en`"]
pub type SOFT_GPSEC_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_gpsec_gate_en`"]
pub struct SOFT_GPSEC_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_GPSEC_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `soft_rsa_gate_en`"]
pub type SOFT_RSA_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_rsa_gate_en`"]
pub struct SOFT_RSA_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RSA_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `soft_i2s_gate_en`"]
pub type SOFT_I2S_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_i2s_gate_en`"]
pub struct SOFT_I2S_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_I2S_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `soft_lcd_gate_en`"]
pub type SOFT_LCD_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_lcd_gate_en`"]
pub struct SOFT_LCD_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_LCD_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `soft_pwm_gate_en`"]
pub type SOFT_PWM_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_pwm_gate_en`"]
pub struct SOFT_PWM_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_PWM_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `soft_sd_adc_gate_en`"]
pub type SOFT_SD_ADC_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_sd_adc_gate_en`"]
pub struct SOFT_SD_ADC_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_SD_ADC_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `soft_gpio_gate_en`"]
pub type SOFT_GPIO_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_gpio_gate_en`"]
pub struct SOFT_GPIO_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_GPIO_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `soft_timer_gate_en`"]
pub type SOFT_TIMER_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_timer_gate_en`"]
pub struct SOFT_TIMER_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_TIMER_GATE_EN_W<'a> {
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
#[doc = "Reader of field `soft_rf_cfg_gate_en`"]
pub type SOFT_RF_CFG_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_rf_cfg_gate_en`"]
pub struct SOFT_RF_CFG_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RF_CFG_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `soft_dma_gate_en`"]
pub type SOFT_DMA_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_dma_gate_en`"]
pub struct SOFT_DMA_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_DMA_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `soft_ls_spi_gate_en`"]
pub type SOFT_LS_SPI_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_ls_spi_gate_en`"]
pub struct SOFT_LS_SPI_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_LS_SPI_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `soft_uart1_gate_en`"]
pub type SOFT_UART1_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_uart1_gate_en`"]
pub struct SOFT_UART1_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_UART1_GATE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `soft_uart0_gate_en`"]
pub type SOFT_UART0_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_uart0_gate_en`"]
pub struct SOFT_UART0_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_UART0_GATE_EN_W<'a> {
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
#[doc = "Reader of field `soft_i2c_gate_en`"]
pub type SOFT_I2C_GATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_i2c_gate_en`"]
pub struct SOFT_I2C_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_I2C_GATE_EN_W<'a> {
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
impl R {
    #[doc = "Bit 14 - 7816/uart2 clock"]
    #[inline(always)]
    pub fn soft_7816_gate_en(&self) -> SOFT_7816_GATE_EN_R {
        SOFT_7816_GATE_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 7816/uart2"]
    #[inline(always)]
    pub fn soft_gpsec_gate_en(&self) -> SOFT_GPSEC_GATE_EN_R {
        SOFT_GPSEC_GATE_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 7816/uart2"]
    #[inline(always)]
    pub fn soft_rsa_gate_en(&self) -> SOFT_RSA_GATE_EN_R {
        SOFT_RSA_GATE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 7816/uart2"]
    #[inline(always)]
    pub fn soft_i2s_gate_en(&self) -> SOFT_I2S_GATE_EN_R {
        SOFT_I2S_GATE_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - lcd clock"]
    #[inline(always)]
    pub fn soft_lcd_gate_en(&self) -> SOFT_LCD_GATE_EN_R {
        SOFT_LCD_GATE_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - pwm clock"]
    #[inline(always)]
    pub fn soft_pwm_gate_en(&self) -> SOFT_PWM_GATE_EN_R {
        SOFT_PWM_GATE_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - sd_adc clock"]
    #[inline(always)]
    pub fn soft_sd_adc_gate_en(&self) -> SOFT_SD_ADC_GATE_EN_R {
        SOFT_SD_ADC_GATE_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - gpio clock"]
    #[inline(always)]
    pub fn soft_gpio_gate_en(&self) -> SOFT_GPIO_GATE_EN_R {
        SOFT_GPIO_GATE_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - timer clock"]
    #[inline(always)]
    pub fn soft_timer_gate_en(&self) -> SOFT_TIMER_GATE_EN_R {
        SOFT_TIMER_GATE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - rf_cfg clock: internal use, do not modify"]
    #[inline(always)]
    pub fn soft_rf_cfg_gate_en(&self) -> SOFT_RF_CFG_GATE_EN_R {
        SOFT_RF_CFG_GATE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - dma clock"]
    #[inline(always)]
    pub fn soft_dma_gate_en(&self) -> SOFT_DMA_GATE_EN_R {
        SOFT_DMA_GATE_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - low speed spi clock"]
    #[inline(always)]
    pub fn soft_ls_spi_gate_en(&self) -> SOFT_LS_SPI_GATE_EN_R {
        SOFT_LS_SPI_GATE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - uart1 clock"]
    #[inline(always)]
    pub fn soft_uart1_gate_en(&self) -> SOFT_UART1_GATE_EN_R {
        SOFT_UART1_GATE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - uart0 clock"]
    #[inline(always)]
    pub fn soft_uart0_gate_en(&self) -> SOFT_UART0_GATE_EN_R {
        SOFT_UART0_GATE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - i2c clock"]
    #[inline(always)]
    pub fn soft_i2c_gate_en(&self) -> SOFT_I2C_GATE_EN_R {
        SOFT_I2C_GATE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - 7816/uart2 clock"]
    #[inline(always)]
    pub fn soft_7816_gate_en(&mut self) -> SOFT_7816_GATE_EN_W {
        SOFT_7816_GATE_EN_W { w: self }
    }
    #[doc = "Bit 13 - 7816/uart2"]
    #[inline(always)]
    pub fn soft_gpsec_gate_en(&mut self) -> SOFT_GPSEC_GATE_EN_W {
        SOFT_GPSEC_GATE_EN_W { w: self }
    }
    #[doc = "Bit 12 - 7816/uart2"]
    #[inline(always)]
    pub fn soft_rsa_gate_en(&mut self) -> SOFT_RSA_GATE_EN_W {
        SOFT_RSA_GATE_EN_W { w: self }
    }
    #[doc = "Bit 11 - 7816/uart2"]
    #[inline(always)]
    pub fn soft_i2s_gate_en(&mut self) -> SOFT_I2S_GATE_EN_W {
        SOFT_I2S_GATE_EN_W { w: self }
    }
    #[doc = "Bit 10 - lcd clock"]
    #[inline(always)]
    pub fn soft_lcd_gate_en(&mut self) -> SOFT_LCD_GATE_EN_W {
        SOFT_LCD_GATE_EN_W { w: self }
    }
    #[doc = "Bit 9 - pwm clock"]
    #[inline(always)]
    pub fn soft_pwm_gate_en(&mut self) -> SOFT_PWM_GATE_EN_W {
        SOFT_PWM_GATE_EN_W { w: self }
    }
    #[doc = "Bit 8 - sd_adc clock"]
    #[inline(always)]
    pub fn soft_sd_adc_gate_en(&mut self) -> SOFT_SD_ADC_GATE_EN_W {
        SOFT_SD_ADC_GATE_EN_W { w: self }
    }
    #[doc = "Bit 7 - gpio clock"]
    #[inline(always)]
    pub fn soft_gpio_gate_en(&mut self) -> SOFT_GPIO_GATE_EN_W {
        SOFT_GPIO_GATE_EN_W { w: self }
    }
    #[doc = "Bit 6 - timer clock"]
    #[inline(always)]
    pub fn soft_timer_gate_en(&mut self) -> SOFT_TIMER_GATE_EN_W {
        SOFT_TIMER_GATE_EN_W { w: self }
    }
    #[doc = "Bit 5 - rf_cfg clock: internal use, do not modify"]
    #[inline(always)]
    pub fn soft_rf_cfg_gate_en(&mut self) -> SOFT_RF_CFG_GATE_EN_W {
        SOFT_RF_CFG_GATE_EN_W { w: self }
    }
    #[doc = "Bit 4 - dma clock"]
    #[inline(always)]
    pub fn soft_dma_gate_en(&mut self) -> SOFT_DMA_GATE_EN_W {
        SOFT_DMA_GATE_EN_W { w: self }
    }
    #[doc = "Bit 3 - low speed spi clock"]
    #[inline(always)]
    pub fn soft_ls_spi_gate_en(&mut self) -> SOFT_LS_SPI_GATE_EN_W {
        SOFT_LS_SPI_GATE_EN_W { w: self }
    }
    #[doc = "Bit 2 - uart1 clock"]
    #[inline(always)]
    pub fn soft_uart1_gate_en(&mut self) -> SOFT_UART1_GATE_EN_W {
        SOFT_UART1_GATE_EN_W { w: self }
    }
    #[doc = "Bit 1 - uart0 clock"]
    #[inline(always)]
    pub fn soft_uart0_gate_en(&mut self) -> SOFT_UART0_GATE_EN_W {
        SOFT_UART0_GATE_EN_W { w: self }
    }
    #[doc = "Bit 0 - i2c clock"]
    #[inline(always)]
    pub fn soft_i2c_gate_en(&mut self) -> SOFT_I2C_GATE_EN_W {
        SOFT_I2C_GATE_EN_W { w: self }
    }
}
