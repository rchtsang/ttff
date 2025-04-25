*** Test Cases ***
hello_world_user on nrf52840dk_nrf52840
    ${x}=                       Execute Command                                             include @${CURDIR}/hello_world_user.resc
    Create Terminal Tester      sysbus.uart0                              timeout=5
    Start Emulation
    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***     pauseEmulation=true
    Wait For Line On Uart       Hello World from UserSpace! (nrf52840dk)    pauseEmulation=true