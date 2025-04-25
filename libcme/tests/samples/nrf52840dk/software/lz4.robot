*** Test Cases ***
lz4 on nrf52840dk_nrf52840
    ${x}=                       Execute Command         include @${CURDIR}/lz4.resc
    Create Terminal Tester      sysbus.uart0                                 timeout=5
    Start Emulation
    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***         pauseEmulation=true
    Wait For Line On Uart       Original Data size: \\d+              treatAsRegex=true    pauseEmulation=true
    Wait For Line On Uart       Compressed Data size : \\d+           treatAsRegex=true    pauseEmulation=true
    Wait For Line On Uart       Successfully decompressed some data   treatAsRegex=true    pauseEmulation=true
    Wait For Line On Uart       Validation done. The string we ended up with is:           pauseEmulation=true
    Wait For Line On Uart       .*                                    treatAsRegex=true    pauseEmulation=true