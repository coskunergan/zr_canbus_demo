// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

#![no_std]

extern crate alloc;

use alloc::format;
use embassy_time::{Duration, Timer};

#[cfg(feature = "executor-thread")]
use embassy_executor::Executor;

#[cfg(feature = "executor-zephyr")]
use zephyr::embassy::Executor;

use core::cell::OnceCell;
use critical_section::Mutex as CriticalMutex;
use embassy_executor::Spawner;
use static_cell::StaticCell;

use zephyr::{device::gpio::GpioPin, sync::Mutex};

use core::{sync::atomic::AtomicBool, sync::atomic::AtomicI32, sync::atomic::Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use pin::{GlobalPin, Pin};

mod pin;
mod button;
mod usage;

static EXECUTOR_MAIN: StaticCell<Executor> = StaticCell::new();
static LED_PIN: GlobalPin = GlobalPin::new();
//====================================================================================
//====================================================================================
#[embassy_executor::task]
async fn display_task(spawner: Spawner) {

    let led_pin = LED_PIN.get();

    let button = zephyr::devicetree::labels::button::get_instance().unwrap();

    declare_buttons!(
        spawner,
        [(
            button,
            || {
                zephyr::printk!("Button Pressed!\n");
                led_pin.toggle();
            },
            Duration::from_millis(10)
        )]
    );

    loop {
        let _ = Timer::after(Duration::from_millis(1000)).await;
        led_pin.toggle();
    }
}
//====================================================================================
#[no_mangle]
extern "C" fn rust_main() {

    let _ = usage::set_logger();

    log::info!("Restart!!!\r\n");

    LED_PIN.init(Pin::new(
        zephyr::devicetree::labels::my_led::get_instance().expect("my_led not found!"),
    ));    

    let executor = EXECUTOR_MAIN.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(display_task(spawner)).unwrap();
    })
}
//====================================================================================
//====================================================================================
//====================================================================================
