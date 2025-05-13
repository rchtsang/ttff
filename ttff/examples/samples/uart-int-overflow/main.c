/**
 * a uart demo
 */

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include "nrf.h"
#include "boards.h"
#include "nrfx_uart.h"
#include "nrfx_errors.h"


#define UART_TX_BUF_SIZE    256     /**< UART TX buffer size. */
#define UART_RX_BUF_SIZE    256     /**< UART RX buffer size. */


int read_uart(nrfx_err_t * err_code)
{
    uint8_t rx_buf[UART_RX_BUF_SIZE];

    nrfx_uart_t uart = NRFX_UART_INSTANCE(0);
    nrfx_uart_config_t uart_config = NRFX_UART_DEFAULT_CONFIG;
    uart_config.pseltxd = TX_PIN_NUMBER;
    uart_config.pselrxd = RX_PIN_NUMBER;
    uart_config.pselcts = CTS_PIN_NUMBER;
    uart_config.pselrts = RTS_PIN_NUMBER;
    uart_config.baudrate = NRF_UART_BAUDRATE_115200;

    *err_code = nrfx_uart_init(&uart, &uart_config, NULL);
    if (*err_code != NRFX_SUCCESS) {
        return -1;
    }

    size_t len = 0;

    // gets the number of bytes to read from uart
    if ((*err_code = nrfx_uart_rx(&uart, rx_buf, 1) != NRFX_SUCCESS)) {
        return -1;
    }

    len = rx_buf[0];

    // a dummy variable we intend to overflow.
    // note: we trigger overflows of 32-bit integers right now,
    // since we follow the all-you-ever implementation.
    // though we could also trigger on any hardware-detected 
    // overflow by examining the OV varnode in pcode.
    uint32_t dummy = 0;

    // reads number of bytes specified by len
    // 
    // this allows stack buffer overflow -> rop attack
    if ((*err_code = nrfx_uart_rx(&uart, rx_buf, len) != NRFX_SUCCESS)) {
        return -1;
    }

    for (int i = 0; i < len; i++) {
        // increase odds of overflow by shifting to top 8 bits
        dummy += (rx_buf[i] << 24);
    }

    return dummy;
}

// compiled at O1 to prevent tail-call optimization
nrfx_err_t dummy_fn(uint8_t * buf)
{
    nrfx_err_t err_code;
    int dummy = 0;
    
    if ((dummy = read_uart(&err_code)) < 0) {
        return err_code;
    }

    buf[dummy] = 0xFF;

    return err_code;
}


int main(void)
{
    #define PADDING_SIZE 256
    uint8_t padding[PADDING_SIZE];
    int i;

    for (i = 0; i < PADDING_SIZE; i++) {
        padding[i] = 0;
    }

    dummy_fn(padding);

    for (i = 0; i < PADDING_SIZE; i++) {
        if (padding[i] == 0xFF) {
            break;
        }
    }

    return i;
}

