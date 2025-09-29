// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

extern "C" {
    fn mb_slave_init(dev: *const ::core::ffi::c_char) -> i32;
    fn mb_add_holding_reg(reg: *mut u16, addr: u16) -> i32;
}

pub struct ModbusSlave {
    _private: (),
}

impl ModbusSlave {
    pub fn new(dev: &str) -> Self {
        let ret = unsafe { mb_slave_init(dev.as_ptr()) };
        if ret != 0 {
            panic!("Failed to initialize Modbus Slave: error {}", ret);
        }
        ModbusSlave { _private: () }
    }

    pub fn mb_add_holding_reg(&self, reg: *mut u16, addr: u16) {
        let ret = unsafe { mb_add_holding_reg(reg, addr) };
        if ret != 0 {
            panic!("Failed to write to mb_add_holding_reg: error {}", ret);
        }
    }
}
