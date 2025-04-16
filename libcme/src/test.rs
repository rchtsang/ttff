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
        0x01, 0x48,              //  0: ldr   r0, [pc, #4]   @ (8 <exit+0x2>)
        0x00, 0xf0, 0x18, 0xf8,  //  2: bl 36 <main>

        // 00000006 <exit>:
        0xfe, 0xe7,              //  6: b.n   6 <exit>
        0x40, 0x00, 0x00, 0x00,  //  8: .word 0x00000040

        // 0000000c <smash>:
        0x85, 0xb0,              //  c: sub   sp, #20
        0x00, 0x22,              //  e: movs  r2, #0
        0x00, 0x92,              // 10: str   r2, [sp, #0]
        0x01, 0x92,              // 12: str   r2, [sp, #4]
        0x02, 0x92,              // 14: str   r2, [sp, #8]
        0x03, 0x92,              // 16: str   r2, [sp, #12]
        0x04, 0x92,              // 18: str   r2, [sp, #16]
        0x03, 0x68,              // 1a: ldr   r3, [r0, #0]

        // 0000001c <loop_start>:
        0x4f, 0xea, 0x82, 0x07,  // 1c: mov.w r7, r2, lsl #2
        0x6f, 0x44,              // 20: add   r7, sp
        0x3b, 0x60,              // 22: str   r3, [r7, #0]
        0x01, 0x32,              // 24: adds  r2, #1

        // 00000026 <loop_cond>:
        0x06, 0x2a,              // 26: cmp   r2, #6
        0xf8, 0xdd,              // 28: ble.n 1c <loop_start>

        // 0000002a <loop_end>:
        0x05, 0xb0,              // 2a: add   sp, #20
        0x70, 0x47,              // 2c: bx lr

        // 0000002e <foo>:
        0x00, 0xb5,              // 2e: push  {lr}
        0xff, 0xf7, 0xec, 0xff,  // 30: bl c <smash>
        0x00, 0xbd,              // 34: pop   {pc}

        // 00000036 <main>:
        0x00, 0xb5,              // 36: push  {lr}
        0xff, 0xf7, 0xf9, 0xff,  // 38: bl 2e <foo>
        0x00, 0xbd,              // 3c: pop   {pc}
        0x00, 0x00,

        // 00000040 <.data>:
        0xde, 0xc0, 0xad, 0x0b, // 40: .word 0x0badc0de 
    ];

}