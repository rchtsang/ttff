*** Test Cases ***
hello_world on nrf52840dk_nrf52840
    ${x}=                       Execute Command         include @${CURDIR}/hello_world.resc
    Create Terminal Tester      sysbus.uart0    timeout=5    defaultPauseEmulation=true

    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***
    Wait For Line On Uart       Hello World! nrf52840dk