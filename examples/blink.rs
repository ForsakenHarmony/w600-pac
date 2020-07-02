#![no_std]
#![no_main]

use core::marker::PhantomData;
use cortex_m_rt::entry;
use panic_abort as _;
use w600_pac::{Peripherals, GPIOA};

fn init_ports(dp: &mut Peripherals) {
    dp.RCC.sw_clkg_en.write(|w| w.soft_gpio_gate_en().set_bit());
    dp.GPIOA
        .data_en
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 16) });
    dp.GPIOA
        .dir
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 16) });
}

// don't compile with optimization enabled!
fn delay(mut n: u32) {
    while n != 0 {
        n -= 1;
    }
}

fn blink_led(dp: &mut Peripherals) {
    let mut on = false;
    let off_bits: u32 = !(1 << 16);
    let on_bits: u32 = (1 << 16);
    loop {
        dp.GPIOA
            .data
            .write(|w| unsafe { w.bits(if on { on_bits } else { off_bits }) });
        delay(0x4ffff);
        on = !on;
    }
}

#[entry]
fn main() -> ! {
    let mut dp = crate::Peripherals::take().unwrap();
    init_ports(&mut dp);
    blink_led(&mut dp);
    loop {}
}
