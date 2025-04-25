*** Test Cases ***
philosophers on nrf52840dk_nrf52840
    ${x}=                       Execute Command             include @${CURDIR}/philosophers.resc
    Create Terminal Tester      sysbus.uart0        timeout=5
    Start Emulation
    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***    pauseEmulation=true
    Wait For Line On Uart       Philosopher 5.*THINKING     treatAsRegex=true         pauseEmulation=true
    Wait For Line On Uart       Philosopher 5.*HOLDING      treatAsRegex=true         pauseEmulation=true
    Wait For Line On Uart       Philosopher 5.*EATING       treatAsRegex=true         pauseEmulation=true