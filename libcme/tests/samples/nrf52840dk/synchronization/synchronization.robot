*** Variables ***
 # The sample's total delay between two UART lines should be 600ms, composed of:
 #  - 100ms busy_wait
 #  - 500ms k_sleep
 # An extra 50ms tolerance is allowed for kernel scheduler and UART output
${TIMEOUT}                      0.65

*** Test Cases ***
synchronization on nrf52840dk_nrf52840
    ${x}=                       Execute Command         include @${CURDIR}/synchronization.resc
    Create Terminal Tester      sysbus.uart0                                  timeout=5    defaultPauseEmulation=true

    # limit PerformanceInMips to improve busy waiting handling
    Execute Command             sysbus.cpu0 PerformanceInMips 10
    Create Terminal Tester      sysbus.uart0                                  timeout=${TIMEOUT}  defaultPauseEmulation=true

    Wait For Line On Uart       *** Booting Zephyr OS build cce7e9a706ca ***              timeout=8
    Wait For Line On Uart       thread_a: Hello World from cpu \\d on nrf52840dk!    treatAsRegex=true
    Wait For Line On Uart       thread_b: Hello World from cpu \\d on nrf52840dk!    treatAsRegex=true
    Wait For Line On Uart       thread_a: Hello World from cpu \\d on nrf52840dk!    treatAsRegex=true
    Wait For Line On Uart       thread_b: Hello World from cpu \\d on nrf52840dk!    treatAsRegex=true