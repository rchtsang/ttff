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



#define MAX_TEST_DATA_BYTES (15U)   /**< max number of test bytes to be used for tx and rx. */
#define UART_TX_BUF_SIZE    200     /**< UART TX buffer size. */
#define UART_RX_BUF_SIZE    200     /**< UART RX buffer size. */


int main(void)
{
    nrfx_err_t err_code;

    uint8_t rx_buf[UART_RX_BUF_SIZE];

    nrfx_uart_t uart = NRFX_UART_INSTANCE(0);
    nrfx_uart_config_t uart_config = NRFX_UART_DEFAULT_CONFIG;
    uart_config.pseltxd = TX_PIN_NUMBER;
    uart_config.pselrxd = RX_PIN_NUMBER;
    uart_config.pselcts = CTS_PIN_NUMBER;
    uart_config.pselrts = RTS_PIN_NUMBER;
    uart_config.baudrate = NRF_UART_BAUDRATE_115200;

    err_code = nrfx_uart_init(&uart, &uart_config, NULL);
    if (err_code != NRFX_SUCCESS) {
        return (int) err_code;
    }

    size_t len = 0;

    // gets the number of bytes to read from uart
    if ((err_code = nrfx_uart_rx(&uart, rx_buf, 1) != NRFX_SUCCESS)) {
        return err_code;
    }

    len = rx_buf[0];

    // reads number of bytes specified by len
    // 
    // this allows stack buffer overflow -> rop attack
    if ((err_code = nrfx_uart_rx(&uart, rx_buf, len) != NRFX_SUCCESS)) {
        return err_code;
    }

    return err_code;
}

