// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

extern "C" {
    fn mb_slave_init() -> i32;
    fn mb_add_holding_reg(reg: &u16) -> i32;
}

pub struct Modbus_Slave {
    _private: (),
}

impl Modbus_Slave {
    pub fn new(baudrate: u32) -> Self {
        let ret = unsafe { mb_slave_init() };
        if ret != 0 {
            panic!("Failed to initialize Modbus Slave: error {}", ret);
        }

        Modbus_Slave { _private: () }
    }

    pub fn mb_add_holding_reg(&self, reg: &u16) {
        let ret = unsafe { mb_add_holding_reg(reg) };
        if ret != 0 {
            panic!("Failed to write to mb_add_holding_reg: error {}", ret);
        }
    }
}
