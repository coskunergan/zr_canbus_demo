// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

#include <zephyr/kernel.h>
#include <zephyr/devicetree.h>
#include <zephyr/modbus/modbus.h>
#include <zephyr/sys/util.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(modbus_slave, LOG_LEVEL_DBG);

const uint16_t drv_slave_id = 1;

#define NUMBER_OF_MB_ITEM 64

struct
{
    uint16_t *regs[NUMBER_OF_MB_ITEM];
} modbus_regs_ptr;

static int holding_reg_rd(uint16_t addr, uint16_t *reg)
{
    if(addr >= NUMBER_OF_MB_ITEM)
    {
        return -ENOTSUP;
    }
    else
    {
        if(modbus_regs_ptr.regs[addr] != NULL)
        {
            *reg = *modbus_regs_ptr.regs[addr];
        }
        else
        {
            *reg = 0;
            return -ENOTSUP;
        }
    }
    return 0;
}

static int holding_reg_wr(uint16_t addr, uint16_t reg)
{
    if(addr >= NUMBER_OF_MB_ITEM)
    {
        return -ENOTSUP;
    }
    else
    {
        if(modbus_regs_ptr.regs[addr] != NULL)
        {
            *modbus_regs_ptr.regs[addr] = reg;
        }
        else
        {
            return -ENOTSUP;
        }
    }
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

int mb_slave_init(const char *dev)
{
    const int client_iface = modbus_iface_get_by_name(dev);
    if(modbus_init_server(client_iface, client_param))
    {
        LOG_ERR("Modbus Server initialization failed! client_iface: %d", client_iface);
        return -1;
    }
    LOG_INF("Modbus Server initialization ok. client_iface: %d", client_iface);
    return 0;
}

int mb_add_holding_reg(uint16_t * reg, const uint16_t addr)
{
    if(addr >= NUMBER_OF_MB_ITEM)
    {
        return -1;
    }
    modbus_regs_ptr.regs[addr] = reg;
    return 0;
}