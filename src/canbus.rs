// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

extern "C" {
    fn canbus_init(dev: *const ::core::ffi::c_char) -> i32;
    fn canbus_isotp_send(data: *const u8, len: u16) -> i32;
}

pub struct CanBus {
    _private: (),
}

impl CanBus {
    pub fn new(dev: &str) -> Self {
        let ret = unsafe { canbus_init(dev.as_ptr()) };
        if ret != 0 {
            panic!("Failed to initialize CanBus: error {}", ret);
        }
        CanBus { _private: () }
    }

    pub fn canbus_isotp_send(&self, data: &[u8]) {
        let ret = unsafe { canbus_isotp_send(data.as_ptr() as *mut u8, data.len() as u16) };
        if ret != 0 {
            panic!("Failed to write to canbus_isotp_send: error {}", ret);
        }
    }
}
