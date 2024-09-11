#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use stm32f3::stm32f303;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Taking ownership of the PAC's peripheral singleton
    let mut peripherals = stm32f303::Peripherals::take().unwrap();

    // Enable the clock for GPIOE peripheral on the AHB bus
    let rcc = &peripherals.RCC;
    rcc.ahbenr.modify(|_, w| w.iopeen().set_bit());

    // Configure the mode of the IO pin as output
    let gpioe = &peripherals.GPIOE;
    gpioe.moder.modify(|_, w| w.moder13().output());

    // Toggle the LED via setting the output data register
    let mut is_on: bool = true;
    loop {
        gpioe.odr.modify(|_, w| w.odr13().bit(is_on));
        for _ in 0..500_000 {
            nop();
        }
        is_on = !is_on;
    }
}