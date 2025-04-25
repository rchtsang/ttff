*** Test Cases ***
rust-app on nrf52840dk_nrf52840
    ${x}=                       Execute Command         include @${CURDIR}/rust-app.resc
    Create Terminal Tester      sysbus.uart0    timeout=5
    Start Emulation
    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***    pauseEmulation=true
    Wait For Line On Uart       Next call will crash if userspace is working          pauseEmulation=true
    Wait For Line On Uart       .*ZEPHYR FATAL ERROR.*                                pauseEmulation=true  treatAsRegex=true