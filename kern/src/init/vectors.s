.global context_save
context_save:
    // FIXME: Save the remaining context to the stack.

.global context_restore
context_restore:
    // FIXME: Restore the context from the stack.
    ret

.macro HANDLER source, kind
    .align 7
    stp     lr, xzr, [SP, #-16]!
    stp     x28, x29, [SP, #-16]!
    
    mov     x29, \source
    movk    x29, \kind, LSL #16
    bl      context_save
    
    ldp     x28, x29, [SP], #16
    ldp     lr, xzr, [SP], #16
    eret
.endm
    
.align 11
.global vectors
vectors:
    // FIXME: Setup the 16 exception vectors.

