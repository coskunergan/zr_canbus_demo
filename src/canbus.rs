// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

use core::slice;

extern "C" {
    fn canbus_init(dev: *const ::core::ffi::c_char) -> i32;
    fn canbus_isotp_send(data: *const u8, len: u16) -> i32;
}

pub struct CanBus {
    _private: (),
}

type CanDataCallback = fn(data: &[u8]);
static mut CANBUS_RX_CALLBACK: Option<CanDataCallback> = None;

#[no_mangle]
pub extern "C" fn canbus_data_handler(data: *const u8, len: u32) {
    let rust_slice = unsafe { slice::from_raw_parts(data, len as usize) };
    unsafe {
        if let Some(callback) = CANBUS_RX_CALLBACK {
            callback(rust_slice);
        }
    }
}

impl CanBus {
    pub fn new(dev: &str) -> Self {
        let ret = unsafe { canbus_init(dev.as_ptr()) };
        if ret != 0 {
            panic!("Failed to initialize CanBus: error {}", ret);
        }
        CanBus { _private: () }
    }
    pub fn set_data_callback(&mut self, callback: CanDataCallback) {
        unsafe {
            CANBUS_RX_CALLBACK = Some(callback);
        }
    }
    pub fn canbus_isotp_send(&self, data: &[u8]) -> Result<(), i32> {
        let ret = unsafe { canbus_isotp_send(data.as_ptr() as *mut u8, data.len() as u16) };
        if ret == 0 {
            Ok(())
        } else {
            Err(ret)
        }
    }
}
