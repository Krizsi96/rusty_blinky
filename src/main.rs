#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm::nop;
use panic_halt as _;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::ToggleableOutputSwitch;

#[entry]
fn main() -> ! {
    // Taking ownership of the PAC's peripheral singleton
    let device_peripherals = pac::Peripherals::take().unwrap();

    // Enable the clock for GPIOE peripheral on the AHB bus
    let mut reset_and_clock_control = device_peripherals.RCC.constrain();

    // Configure the mode of the IO pin as output
    let mut gpioe = device_peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    // Toggle the LED
    loop {
        leds.ld10.toggle().unwrap();
        for _ in 0..500_000 {
            nop();
        }
    }
}