.section .text.init

.global _start

_start:
    // read cpu affinity, start core 0, halt rest
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f

1:
    // core affinity != 0, halt it
    wfe
    b       1b

2:
    // set the stack to start before our boot code
    adr     x1, _start
    mov     sp, x1

    // load the start address and number of bytes in BSS section
    ldr     x1, =__bss_beg
    ldr     x2, =__bss_len

3:
    // zero out the BSS section, 64-bits at a time
    cbz     x2, 4f
    str     xzr, [x1], #8
    sub     x2, x2, #8
    cbnz    x2, 3b

4:
    // jump to kmain, which shouldn't return. halt if it does
    bl      kmain
    b       1b
