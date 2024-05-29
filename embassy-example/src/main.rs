#![no_std]
#![no_main]

use cortex_m_semihosting::{debug,hprintln};
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt as rt;
//use stm32f429::*;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use rt::entry;
use rt::exception;

use rt::ExceptionFrame;


#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let gpiob = p.GPIOB.split();
    let mut led = gpiob.pb14.into_push_pull_output();

    let gpioc = p.GPIOC.split();
    let button = gpioc.pc13.into_pull_down_input();

    let mut off_state = false;
    hprintln!("Enter loop");
    loop {
        if button.is_high(){
            hprintln!("Button pressed");
            off_state = true;
        } else {
            off_state = false;
        }
        if off_state {
            led.set_low();
        } else {
                
            for _ in 0..10_000 {
                led.set_high();
            }
            for _ in 0..10_000 {
                led.set_low();
            }
        }
    }
}


#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}