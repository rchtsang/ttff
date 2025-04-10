//! tests.rs
//! 
//! miscellaneous testing assets


pub mod programs {

    // a program that computes (((3 ** 2) ** 2) ** 2)
    // compiled with xpack arm-none-eabi-gcc arm64 11.3.1 20220712
    // arm-none-eabi-gcc main.c -mcpu=cortex-m4 -mthumb -nostdlib
    pub(crate) static TEST_PROG_SQUARE: &[u8] = &[
        // 00000000 <_start>:
        0x00, 0xf0, 0x01, 0xf8, //  0: bl  6 <main>
        // 00000004 <exit>:
        0xfe, 0xe7,             //  4: b.n 4 <exit>
        // 00000006 <main>:
        0x80, 0xb5,             //  6: push     {r7, lr}
        0x82, 0xb0,             //  8: sub      sp, #8
        0x00, 0xaf,             //  a: add      r7, sp, #0
        0x03, 0x23,             //  c: movs     r3, #3
        0x7b, 0x60,             //  e: str      r3, [r7, #4]
        0x00, 0x23,             // 10: movs     r3, #0
        0x3b, 0x60,             // 12: str      r3, [r7, #0]
        0x06, 0xe0,             // 14: b.n      24 <main+0x1e>
        0x78, 0x68,             // 16: ldr      r0, [r7, #4]
        0x00, 0xf0, 0x0c, 0xf8, // 18: bl       34 <square>
        0x78, 0x60,             // 1c: str      r0, [r7, #4]
        0x3b, 0x68,             // 1e: ldr      r3, [r7, #0]
        0x01, 0x33,             // 20: adds     r3, #1
        0x3b, 0x60,             // 22: str      r3, [r7, #0]
        0x3b, 0x68,             // 24: ldr      r3, [r7, #0]
        0x02, 0x2b,             // 26: cmp      r3, #2
        0xf5, 0xdd,             // 28: ble.n    16 <main+0x10>
        0x7b, 0x68,             // 2a: ldr      r3, [r7, #4]
        0x18, 0x46,             // 2c: mov      r0, r3
        0x08, 0x37,             // 2e: adds     r7, #8
        0xbd, 0x46,             // 30: mov      sp, r7
        0x80, 0xbd,             // 32: pop      {r7, pc}
        // 00000034 <square>:
        0x80, 0xb4,             // 34: push     {r7}
        0x83, 0xb0,             // 36: sub      sp, #12
        0x00, 0xaf,             // 38: add      r7, sp, #0
        0x78, 0x60,             // 3a: str      r0, [r7, #4]
        0x7b, 0x68,             // 3c: ldr      r3, [r7, #4]
        0x03, 0xfb, 0x03, 0xf3, // 3e: mul.w    r3, r3, r3
        0x18, 0x46,             // 42: mov      r0, r3
        0x0c, 0x37,             // 44: adds     r7, #12
        0xbd, 0x46,             // 46: mov      sp, r7
        0x80, 0xbc,             // 48: pop      {r7}
        0x70, 0x47,             // 4a: bx       lr
    ];


    /// a test program that smashes its own stack
    pub(crate) static STACK_SMASH_TEST: &[u8] = &[
        // 00000000 <_start>:
        0x01, 0x48,             //  0: ldr   r0, [pc, #4]   @ (8 <exit+0x2>)
        0x00, 0xf0, 0x20, 0xf8, //  2: bl 46 <main>

        // 00000006 <exit>:
        0xfe, 0xe7,             //  6: b.n   6 <exit>
        0x60, 0x00, 0x00, 0x00, //  8: .word 0x00000060

        // 0000000c <smash>:
        0x80, 0xb4,             //  c: push  {r7}
        0x89, 0xb0,             //  e: sub   sp, #36  @ 0x24
        0x00, 0xaf,             // 10: add   r7, sp, #0
        0x78, 0x60,             // 12: str   r0, [r7, #4]
        0x07, 0xf1,  0x03, 0x08,// 14: add.w r3, r7, #8
        0x00, 0x22,             // 18: movs  r2, #0
        0x1a, 0x60,             // 1a: str   r2, [r3, #0]
        0x5a, 0x60,             // 1c: str   r2, [r3, #4]
        0x9a, 0x60,             // 1e: str   r2, [r3, #8]
        0xda, 0x60,             // 20: str   r2, [r3, #12]
        0x1a, 0x61,             // 22: str   r2, [r3, #16]
        0x00, 0x23,             // 24: movs  r3, #0
        0xfb, 0x61,             // 26: str   r3, [r7, #28]
        0x05, 0xe0,             // 28: b.n   36 <smash+0x2a>
        0x7b, 0x68,             // 2a: ldr   r3, [r7, #4]
        0x1b, 0x68,             // 2c: ldr   r3, [r3, #0]
        0xbb, 0x60,             // 2e: str   r3, [r7, #8]
        0xfb, 0x69,             // 30: ldr   r3, [r7, #28]
        0x01, 0x33,             // 32: adds  r3, #1
        0xfb, 0x61,             // 34: str   r3, [r7, #28]
        0xfb, 0x69,             // 36: ldr   r3, [r7, #28]
        0x13, 0x2b,             // 38: cmp   r3, #19
        0xf6, 0xdd,             // 3a: ble.n 2a <smash+0x1e>
        0x00, 0xbf,             // 3c: nop
        0x24, 0x37,             // 3e: adds  r7, #36  @ 0x24
        0xbd, 0x46,             // 40: mov   sp, r7
        0x80, 0xbc,             // 42: pop   {r7}
        0x70, 0x47,             // 44: bx lr

        // 00000046 <main>:
        0x80, 0xb5,             // 46: push  {r7, lr}
        0x82, 0xb0,             // 48: sub   sp, #8
        0x00, 0xaf,             // 4a: add   r7, sp, #0
        0x78, 0x60,             // 4c: str   r0, [r7, #4]
        0x78, 0x68,             // 4e: ldr   r0, [r7, #4]
        0xff, 0xf7, 0xdc, 0xff, // 50: bl c <smash>
        0x00, 0x23,             // 54: movs  r3, #0
        0x18, 0x46,             // 56: mov   r0, r3
        0x08, 0x37,             // 58: adds  r7, #8
        0xbd, 0x46,             // 5a: mov   sp, r7
        0x80, 0xbd,             // 5c: pop   {r7, pc}
        0x00, 0x00,

        // 00000060 .data
        0xde, 0xc0, 0xad, 0x0b,
    ];

}