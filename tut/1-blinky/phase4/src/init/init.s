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

    // jump to kinit, which shouldn't return. halt if it does
    bl      kinit
    b       1b
