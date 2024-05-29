#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]

extern crate alloc;

use cortex_m_semihosting::hprintln;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt as rt;
//use stm32f429::*;
//use stm32f4xx_hal as hal;

//use crate::hal::{pac, prelude::*};
use rt::entry;
use rt::exception;

use rt::ExceptionFrame;

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    //panic!("allocation error: {:?}", layout)
    hprintln!("allocation error: {:?}", layout);
    asm::bkpt();
    loop {}
}

use alloc::vec;

//extern crate embedded_hal;
#[entry]
fn main() -> ! {


    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut test_vec = vec![1u128, 2, 3, 4, 5];
    test_vec.push(6);

    let peripherals = stm32f429::Peripherals::take().unwrap();
    let gpiob = &peripherals.GPIOB;
    let rcc = &peripherals.RCC;

    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
    rcc.apb1enr.modify(|_, w| w.tim7en().set_bit());

    // LEDs 1-3
    gpiob.moder.modify(|_, w| unsafe { w.moder0().bits(1) } );
    gpiob.moder.modify(|_, w| unsafe { w.moder7().bits(1) } );
    gpiob.moder.modify(|_, w| unsafe { w.moder14().bits(1) } );

    loop {
        gpiob.bsrr.write(|w| w.bs0().set_bit());
        gpiob.bsrr.write(|w| w.bs7().set_bit());
        gpiob.bsrr.write(|w| w.bs14().set_bit());
        delay();

        gpiob.bsrr.write(|w| w.br0().set_bit());
        gpiob.bsrr.write(|w| w.br7().set_bit());
        gpiob.bsrr.write(|w| w.br14().set_bit());
        delay();
        test_vec.push(6);
    }
    
}

fn delay() {
    for _i in 1..30000 {
        asm::nop();
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