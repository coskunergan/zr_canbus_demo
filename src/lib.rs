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
use modbus_slave::Modbus_Slave;

mod button;
mod pin;
mod usage;
mod modbus_slave;

static EXECUTOR_MAIN: StaticCell<Executor> = StaticCell::new();
static RED_LED_PIN: GlobalPin = GlobalPin::new();
static GREEN_LED_PIN: GlobalPin = GlobalPin::new();
//====================================================================================
//====================================================================================
#[embassy_executor::task]
async fn display_task(spawner: Spawner) {
    let red_led_pin = RED_LED_PIN.get();
    let green_led_pin = GREEN_LED_PIN.get();

    let button = zephyr::devicetree::labels::button::get_instance().unwrap();

    declare_buttons!(
        spawner,
        [(
            button,
            || {
                zephyr::printk!("Button Pressed!\n");
                red_led_pin.toggle();
            },
            Duration::from_millis(10)
        )]
    );

    loop {
        let _ = Timer::after(Duration::from_millis(1000)).await;
        red_led_pin.toggle();
        green_led_pin.toggle();
        log::info!("Coskun Ergan!!!\r\n");
    }
}
//====================================================================================
#[no_mangle]
extern "C" fn rust_main() {
    let _ = usage::set_logger();

    log::info!("Restart!!!\r\n");

    let modbus = Modbus_Slave::new(9600);

    RED_LED_PIN.init(Pin::new(
        zephyr::devicetree::labels::my_red_led::get_instance().expect("my_red_led not found!"),
    ));
    GREEN_LED_PIN.init(Pin::new(
        zephyr::devicetree::labels::my_green_led::get_instance().expect("my_green_led not found!"),
    ));

    let executor = EXECUTOR_MAIN.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(display_task(spawner)).unwrap();
    })
}
//====================================================================================
//====================================================================================
//====================================================================================
