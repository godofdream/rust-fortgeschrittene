#![no_std]
#![no_main]


use embassy_stm32::time;
use embassy_sync::channel;
use futures_util::FutureExt;
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::select;
use embassy_futures::{join, select};
use embassy_futures::select::select4;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::gpio::Pull;
use {defmt_rtt as _, panic_probe as _};

use cortex_m::asm;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut button = ExtiInput::new(p.PC13, p.EXTI13, Pull::Down);
    let mut button2 = ExtiInput::new(p.PB1, p.EXTI1, Pull::Down);
    let mut led1 = Output::new(p.PB7, Level::High, Speed::Low);
    let mut led2 = Output::new(p.PB14, Level::High, Speed::Low);

    info!("Press the USER button...");

    loop {
        let (tx,rx) = channel::Channel::new();
        let res = select(
            button_handler(&mut button,&mut led1), 
            button_handler(&mut button2, &mut led2)).await;
        // let pair = embassy_futures::join::join(button_handler(&mut button,led1), button_handler(&mut button2, led2)).await;

        

        // match select(button.wait_for_any_edge().await, button2.wait_for_any_edge().await).await {
        //     Ok(_) => {
        //         info!("Button 1 pressed");
        //     }
        //     Err(_) => {
        //         info!("Button 2 pressed");
        //     }
        // }

    }
}

async fn button_handler(button: &mut ExtiInput<'_>, led: &mut Output<'_>) {
    button.wait_for_rising_edge().await;
    led.set_high();
    for _ in 0..100000 {
        asm::nop();
    }
    button.wait_for_falling_edge().await;
    led.set_low();
}

async fn button_handler_up( button: &mut ExtiInput<'_>, led: &mut Output<'_>) -> u8 {
    button.wait_for_any_edge().await;
    match button.is_high() {
        true => led.set_high(),
        false => led.set_low()
    }
    return 1
}

async fn button_handler_down( button: &mut ExtiInput<'_>, led: &mut Output<'_>) -> u8 {
    button.wait_for_falling_edge().await;
    led.set_low();
    
    return 0
}