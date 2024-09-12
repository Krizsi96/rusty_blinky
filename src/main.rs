#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, prelude::*};

#[entry]
fn main() -> ! {
    // Taking ownership of the PAC's peripheral singleton
    let p = hal::pac::Peripherals::take().unwrap();

    // Enable the clock for GPIOE peripheral on the AHB bus
    let mut rcc = p.RCC.constrain();
    let mut gpioe = p.GPIOE.split(&mut rcc.ahb);

    // Configure the mode of the IO pin as output
    let mut led = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Toggle the LED via setting the output data register
    loop {
        led.toggle().unwrap();
        for _ in 0..500_000 {
            nop();
        }
    }
}