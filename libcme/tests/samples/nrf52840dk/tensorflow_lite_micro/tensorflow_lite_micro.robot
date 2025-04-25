*** Test Cases ***
tensorflow_lite_micro on nrf52840dk_nrf52840
    ${x}=                       Execute Command             include @${CURDIR}/tensorflow_lite_micro.resc
    Create Terminal Tester      sysbus.uart0        timeout=15
    Start Emulation
    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***    pauseEmulation=true
    Wait For Line On Uart       x_value: .* y_value: .*     treatAsRegex=true         pauseEmulation=true
    Wait For Line On Uart       x_value: .* y_value: .*     treatAsRegex=true         pauseEmulation=true
    Wait For Line On Uart       x_value: .* y_value: .*     treatAsRegex=true         pauseEmulation=true
    Wait For Line On Uart       x_value: .* y_value: .*     treatAsRegex=true         pauseEmulation=true