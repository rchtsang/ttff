*** Test Cases ***
micropython on nrf52840dk_nrf52840
    ${x}=                       Execute Command             include @${CURDIR}/micropython.resc
    Create Terminal Tester      sysbus.uart0        timeout=15
    Write Char Delay            0.01
    Start Emulation
    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***    pauseEmulation=true
    Wait For Prompt On Uart     >>>
    Write Line To Uart          2+2
    Wait For Line On Uart       4    pauseEmulation=true
    Write Line To Uart          def compare(a, b): return True if a > b else False
    Write Line To Uart           
    Write Line To Uart          compare(3.2, 2.4)
    Wait For Line On Uart       True    pauseEmulation=true
    Write Line To Uart          compare(2.2, 5.8)
    Wait For Line On Uart       False    pauseEmulation=true