// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

#include <zephyr/kernel.h>
#include <zephyr/devicetree.h>
#include <zephyr/modbus/modbus.h>
#include <zephyr/sys/util.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(modbus_slave, LOG_LEVEL_DBG);

//const struct device *const modbus_slave = DEVICE_DT_GET_OR_NULL(DT_ALIAS(modbus0));
const char *dev = "modbus0";

const uint16_t drv_slave_id = 1;

int mb_add_holding_reg(uint16_t * reg);

#define NUMBER_OF_MB_ITEM 64

struct
{
    uint16_t reserved[NUMBER_OF_MB_ITEM];
} modbus_regs;

static int holding_reg_rd(uint16_t addr, uint16_t *reg)
{
    if(addr >= (sizeof(modbus_regs) / 2))
    {
        return -ENOTSUP;
    }
    else
    {
        //(void)modbus_read_cv.notify_all();
    }

    //LOG_INF("Holding register read, addr %u", addr);

    *reg = ((uint16_t *)&modbus_regs)[addr];

    return 0;
}

static int holding_reg_wr(uint16_t addr, uint16_t reg)
{
    if(addr >= (sizeof(modbus_regs) / 2))
    {
        return -ENOTSUP;
    }
    else
    {
        //(void)modbus_write_cv.notify_all();
    }

    ((uint16_t *)&modbus_regs)[addr] = reg;

    //LOG_INF("Holding register write, addr %u", addr);

    return 0;
}

static struct modbus_user_callbacks mbs_cbs =
{
    .coil_rd = NULL,
    .coil_wr = NULL,
    .input_reg_rd = NULL,
    .holding_reg_rd = holding_reg_rd,
    .holding_reg_wr = holding_reg_wr,
};

const static struct modbus_iface_param client_param =
{
    .mode = MODBUS_MODE_RTU,
    .server = {
        .user_cb = &mbs_cbs,
        .unit_id = drv_slave_id,
    },
    .serial = {
        .baud = 115200,
        .parity = UART_CFG_PARITY_NONE,
        .stop_bits = UART_CFG_STOP_BITS_1,
    },
};

int mb_slave_init()
{
    if(modbus_init_server(modbus_iface_get_by_name(dev), client_param))
    {
        LOG_ERR("Modbus Server initialization failed");
        return -1;
    }
    LOG_INF("Modbus Server initialization ok.");
    return 0;
}

int mb_add_holding_reg(uint16_t * reg)
{
    return 0;
}