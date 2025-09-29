// Copyright (c) 2025
// SPDX-License-Identifier: Apache-2.0
// Coskun ERGAN <coskunergan@gmail.com>

#include <zephyr/kernel.h>
#include <zephyr/canbus/isotp.h>
#include <zephyr/sys/util.h>
#include <zephyr/logging/log.h>

LOG_MODULE_REGISTER(canbus, LOG_LEVEL_DBG);

const struct isotp_fc_opts fc_opts_8_0 = {.bs = 8, .stmin = 0};

const struct isotp_msg_id rx_addr_8_0 =
{
    .std_id = 0x80,
#ifdef CONFIG_SAMPLE_CAN_FD_MODE
    .flags = ISOTP_MSG_FDF | ISOTP_MSG_BRS,
#endif
};
const struct isotp_msg_id tx_addr_8_0 =
{
    .std_id = 0x180,
#ifdef CONFIG_SAMPLE_CAN_FD_MODE
    .dl = 64,
    .flags = ISOTP_MSG_FDF | ISOTP_MSG_BRS,
#endif
};

const struct device *can_dev;
struct isotp_recv_ctx recv_ctx_8_0;
struct isotp_send_ctx send_ctx_8_0;

K_THREAD_STACK_DEFINE(rx_8_0_thread_stack, CONFIG_SAMPLE_RX_THREAD_STACK_SIZE);
struct k_thread rx_8_0_thread_data;

extern void canbus_data_handler(const uint8_t *data, uint32_t len);

void rx_8_0_thread(void *arg1, void *arg2, void *arg3)
{
    ARG_UNUSED(arg1);
    ARG_UNUSED(arg2);
    ARG_UNUSED(arg3);
    int ret, rem_len;
    struct net_buf *buf;

    ret = isotp_bind(&recv_ctx_8_0, can_dev,
                     &tx_addr_8_0, &rx_addr_8_0,
                     &fc_opts_8_0, K_FOREVER);
    if(ret != ISOTP_N_OK)
    {
        LOG_ERR("Failed to bind to rx ID %d [%d]\n",
                rx_addr_8_0.std_id, ret);
        return;
    }

    while(1)
    {
        do
        {
            rem_len = isotp_recv_net(&recv_ctx_8_0, &buf,
                                     K_MSEC(2000));
            if(rem_len < 0)
            {
                break;
            }
            while(buf != NULL)
            {
                canbus_data_handler(buf->data, buf->len);
                buf = net_buf_frag_del(NULL, buf);
            }
        }
        while(rem_len);
    }
}

void send_complette_cb(int error_nr, void *arg)
{
    ARG_UNUSED(arg);
    if(error_nr < 0)
    {
        LOG_ERR("TX complete cb [%d]\n", error_nr);
    }
}

int canbus_init(const char *dev_name)
{
    k_tid_t tid;

    int ret = 0;

    can_dev = device_get_binding(dev_name);

    if(!device_is_ready(can_dev))
    {
        LOG_ERR("CAN: Device driver not ready.\n");
        return -1;
    }

    struct can_timing timing;

    ret = can_calc_timing(can_dev, &timing, 500000, 875);
    if(ret < 0)
    {
        LOG_ERR("Failed to calc timing: %d\n", ret);
        return ret;
    }

    ret = can_set_timing(can_dev, &timing);
    if(ret < 0)
    {
        LOG_ERR("Failed to set timing: %d\n", ret);
        return ret;
    }

    can_mode_t mode = (IS_ENABLED(CONFIG_SAMPLE_LOOPBACK_MODE) ? CAN_MODE_LOOPBACK : 0) |
                      (IS_ENABLED(CONFIG_SAMPLE_CAN_FD_MODE) ? CAN_MODE_FD : 0);
    ret = can_set_mode(can_dev, mode);
    if(ret != 0)
    {
        LOG_ERR("CAN: Failed to set mode [%d]", ret);
        return ret;
    }

    ret = can_start(can_dev);
    if(ret != 0)
    {
        LOG_ERR("CAN: Failed to start device [%d]\n", ret);
        return ret;
    }

    tid = k_thread_create(&rx_8_0_thread_data, rx_8_0_thread_stack,
                          K_THREAD_STACK_SIZEOF(rx_8_0_thread_stack),
                          rx_8_0_thread, NULL, NULL, NULL,
                          CONFIG_SAMPLE_RX_THREAD_PRIORITY, 0, K_NO_WAIT);
    if(!tid)
    {
        LOG_ERR("ERROR spawning rx thread\n");
        return 0;
    }
    k_thread_name_set(tid, "rx_8_0");

    return ret;
}

int canbus_isotp_send(const uint8_t *data, const uint16_t len)
{
    int ret = isotp_send(&send_ctx_8_0, can_dev,
                         data, len,
                         &tx_addr_8_0, &rx_addr_8_0, send_complette_cb, NULL);
    if(ret != ISOTP_N_OK)
    {
        LOG_WRN("Error while sending data to ID %d [%d]\n",
                tx_addr_8_0.std_id, ret);
    }
    return ret;
}