*** Test Cases ***
kenning-zephyr-runtime-tflitemicro on nrf52840dk_nrf52840
    ${x}=                       Execute Command         include @${CURDIR}/kenning-zephyr-runtime-tflitemicro.resc
    Create Terminal Tester      sysbus.uart0    timeout=5  defaultPauseEmulation=true

    Wait For Line On Uart       Booting Zephyr OS build\.+cce7e9a706ca      treatAsRegex=true
    Wait For Line On Uart       I: model output: [wing: 1.000000, ring: 0.000000, slope: 0.000000, negative: 0.000000]
    Wait For Line On Uart       I: model output: [wing: 0.000000, ring: 0.000000, slope: 0.000000, negative: 1.000000]
    Wait For Line On Uart       I: model output: [wing: 0.000000, ring: 0.000000, slope: 1.000000, negative: 0.000000]
    Wait For Line On Uart       I: model output: [wing: 1.000000, ring: 0.000000, slope: 0.000000, negative: 0.000000]
    Wait For Line On Uart       I: model output: [wing: 0.000000, ring: 0.997465, slope: 0.000000, negative: 0.002535]
    Wait For Line On Uart       I: model output: [wing: 0.000000, ring: 0.000000, slope: 1.000000, negative: 0.000000]
    Wait For Line On Uart       I: model output: [wing: 1.000000, ring: 0.000000, slope: 0.000000, negative: 0.000000]
    Wait For Line On Uart       I: model output: [wing: 1.000000, ring: 0.000000, slope: 0.000000, negative: 0.000000]
    Wait For Line On Uart       I: model output: [wing: 1.000000, ring: 0.000000, slope: 0.000000, negative: 0.000000]
    Wait For Line On Uart       I: model output: [wing: 0.000000, ring: 0.000000, slope: 1.000000, negative: 0.000000]
    Wait For Line On Uart       I: inference done