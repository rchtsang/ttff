*** Test Cases ***
shell_module on nrf52840dk_nrf52840
    ${x}=                       Execute Command         include @${CURDIR}/shell_module.resc
    Create Terminal Tester      sysbus.uart0    timeout=10   defaultPauseEmulation=true
    Write Char Delay            0.01

    Wait For Prompt On Uart     uart:~$
    Write Line To Uart
    Wait For Prompt On Uart     uart:~$
    Write Line To Uart          demo board
    Wait For Line On Uart       nrf52840dk