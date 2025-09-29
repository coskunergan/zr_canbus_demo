// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

#![no_std]

extern crate alloc;

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

use core::{sync::atomic::AtomicBool, sync::atomic::AtomicU16, sync::atomic::Ordering};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use canbus::CanBus;
use modbus_slave::Modbus_Slave;
use pin::{GlobalPin, Pin};

mod button;
mod canbus;
mod modbus_slave;
mod pin;
mod usage;

static EXECUTOR_MAIN: StaticCell<Executor> = StaticCell::new();
static RED_LED_PIN: GlobalPin = GlobalPin::new();
static GREEN_LED_PIN: GlobalPin = GlobalPin::new();

static COUNTER: AtomicU16 = AtomicU16::new(0);
static REGISTER: AtomicU16 = AtomicU16::new(0);

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
                REGISTER.fetch_add(1, Ordering::SeqCst);
                red_led_pin.toggle();
            },
            Duration::from_millis(10)
        )]
    );

    loop {
        let _ = Timer::after(Duration::from_millis(1000)).await;
        red_led_pin.toggle();
        green_led_pin.toggle();
        log::info!("Endless Loop!!!\r\n");
        COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}
//====================================================================================
#[embassy_executor::task]
async fn canbus_task(can: CanBus) {

    loop {
        can.canbus_isotp_send("merhaba dunyaaaaaaaaaa!".as_bytes());
        Timer::after(Duration::from_secs(1)).await;
    }
}
//====================================================================================
#[no_mangle]
extern "C" fn rust_main() {
    let _ = usage::set_logger();

    log::info!("Restart!!!\r\n");

    let mut local_reg = 0x456;

    let can_fd = CanBus::new("canbus0\0");
    let modbus = Modbus_Slave::new("modbus0\0");
    let modbus_vcp = Modbus_Slave::new("modbus1\0");

    modbus.mb_add_holding_reg(COUNTER.as_ptr(), 0);
    modbus.mb_add_holding_reg(REGISTER.as_ptr(), 1);
    modbus.mb_add_holding_reg(&mut local_reg, 2);

    modbus_vcp.mb_add_holding_reg(COUNTER.as_ptr(), 0);
    modbus_vcp.mb_add_holding_reg(REGISTER.as_ptr(), 1);
    modbus_vcp.mb_add_holding_reg(&mut local_reg, 2);

    RED_LED_PIN.init(Pin::new(
        zephyr::devicetree::labels::my_red_led::get_instance().expect("my_red_led not found!"),
    ));
    GREEN_LED_PIN.init(Pin::new(
        zephyr::devicetree::labels::my_green_led::get_instance().expect("my_green_led not found!"),
    ));

    let executor = EXECUTOR_MAIN.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(display_task(spawner)).unwrap();
        spawner.spawn(canbus_task(can_fd)).unwrap();
    })
}
//====================================================================================
//====================================================================================
//====================================================================================
