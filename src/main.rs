#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Register addresses
    const RCC_AHBENR: *mut u32 = 0x4002_1014 as *mut u32;
    const GPIOE_MODER: *mut u32 = 0x4800_1000 as *mut u32;
    const GPIOE_ODR: *mut u32 = 0x4800_1014 as *mut u32;

    // Enable the clock for GPIOE peripheral on the AHB bus
    const GPIOEEN: u32 = 21;
    const ENABLE_GPIO_E_MASK: u32 = 1 << GPIOEEN;
    unsafe {
        write_volatile(RCC_AHBENR, read_volatile(RCC_AHBENR) | ENABLE_GPIO_E_MASK);
        hprintln!("updated RCC_AHBENR: {}", read_volatile(RCC_AHBENR));
    }

    // Configure the mode of the IO pin as output
    const GEN_OUTPUT_MODE: u32 = 1;
    let mut gpioe_mode_r: u32 = 0;

    unsafe {
        gpioe_mode_r = read_volatile(GPIOE_MODER);
    }
    // Clear MODER13 (bit 27 and 26)
    const MODE_R_13_CLEAR_MASK: u32 = !(0x3 << 26);
    gpioe_mode_r &= MODE_R_13_CLEAR_MASK;

    // Set MODER13 to general purpose output mode
    const MODE_R_13_MASK: u32 = GEN_OUTPUT_MODE << 26;
    gpioe_mode_r |= MODE_R_13_MASK;
    unsafe {
        write_volatile(GPIOE_MODER, gpioe_mode_r);
        hprintln!("updated GPIOE_MODER: {}", read_volatile(GPIOE_MODER));
    }

    // Toggle the LED via setting the output data register
    const PIN_POS: u32 = 13;
    let mut is_on: bool = true;
    loop {
        unsafe {
            write_volatile(GPIOE_ODR, (is_on as u32) << PIN_POS);
            hprintln!("updated GPIOE_ODR: {}", read_volatile(GPIOE_ODR));
        }
        for _ in 0..500 {
            nop();
        }
        is_on = !is_on;
    }
}
