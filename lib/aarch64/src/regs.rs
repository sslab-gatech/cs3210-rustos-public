// (ref. D13.2: Multiprocessor Affinity Register)
defreg!(MPIDR_EL1, [
    Aff3 [39-32], // Affinity level 3
    U    [30-30], // Indicates a Uniprocessor system
    MT   [24-24], // Multithreading type approach
    Aff2 [23-16], // Affinity level 2
    Aff1 [15-08], // Affinity level 1
    Aff0 [07-00], // Affinity level 0

    RES0 [63-40|29-25],
    RES1 [31-31],
]);

// (ref. D13.2.10: Secure Configuration Register)
defreg!(SCR_EL3, [
    TERR [15-15], // Trap Error record accesses
    TLOR [14-14], // Trap LOR registers
    TWE  [13-13], // Traps EL2, EL1, and EL0 execution of WFE to EL3
    TWI  [12-12], // Traps EL2, EL1, and EL0 execution of WFI to EL3
    ST   [11-11], // Traps Secure EL1 accesses to the Counter-timer Physical Secure timer registers to EL3
    RW   [10-10], // Execution state control for lower Exception levels
    SIF  [09-09], // Secure instruction fetch
    HCE  [08-08], // Hypervisor Call instruction enable
    SMD  [07-07], // Secure Monitor Call disable
    EA   [03-03], // External Abort and SError interrupt routing
    FIQ  [02-02], // Physical FIQ Routing
    IRQ  [01-01], // Physical IRQ Routing
    NS   [00-00], // Non-secure bit

    RES0 [63-16|06-06],
    RES1 [05-04],
]);

// (ref. C5.2.19: Saved Program Status Register)
defreg!(SPSR_EL3, [
    N    [31-31], // Negative Condition flag
    Z    [30-30], // Zero Condition flag
    C    [29-29], // Carry Condition flag
    V    [28-28], // Overflow Condition flag
    TCO  [25-25], // Tag Check Override
    DIT  [24-24], // Data Independent Timing
    UAO  [23-23], // User Access Override
    PAN  [22-22], // Privileged Access Never
    SS   [21-21], // Software Step
    IL   [20-20], // Illegal Execution state
    SSBS [12-12], // Speculative Store Bypass
    BTYPE[11-10], // Branch Type Indicator
    D    [09-09], // Debug exception mask
    A    [08-08], // SError interrupt mask
    I    [07-07], // IRQ interrupt mask
    F    [06-06], // FIQ interrupt mask
    M4   [04-04], // Execution state
    M    [03-00], // AArch64 Exception level and selected Stack Pointer

    RES0 [63-32|27-26|19-13|05-05],
]);

// (ref. C5.2.18: Saved Program Status Register)
defreg!(SPSR_EL2, [
    N    [31-31], // Negative Condition flag
    Z    [30-30], // Zero Condition flag
    C    [29-29], // Carry Condition flag
    V    [28-28], // Overflow Condition flag
    TCO  [25-25], // Tag Check Override
    DIT  [24-24], // Data Independent Timing
    UAO  [23-23], // User Access Override
    PAN  [22-22], // Privileged Access Never
    SS   [21-21], // Software Step
    IL   [20-20], // Illegal Execution state
    SSBS [12-12], // Speculative Store Bypass
    BTYPE[11-10], // Branch Type Indicator
    D    [09-09], // Debug exception mask
    A    [08-08], // SError interrupt mask
    I    [07-07], // IRQ interrupt mask
    F    [06-06], // FIQ interrupt mask
    M4   [04-04], // Execution state
    M    [03-00], // AArch64 Exception level and selected Stack Pointer

    RES0 [63-32|27-26|19-13|05-05],
]);

// (ref. C5.2.17: Saved Program Status Register)
defreg!(SPSR_EL1, [
    N    [31-31], // Negative Condition flag
    Z    [30-30], // Zero Condition flag
    C    [29-29], // Carry Condition flag
    V    [28-28], // Overflow Condition flag
    TCO  [25-25], // Tag Check Override
    DIT  [24-24], // Data Independent Timing
    UAO  [23-23], // User Access Override
    PAN  [22-22], // Privileged Access Never
    SS   [21-21], // Software Step
    IL   [20-20], // Illegal Execution state
    SSBS [12-12], // Speculative Store Bypass
    BTYPE[11-10], // Branch Type Indicator
    D    [09-09], // Debug exception mask
    A    [08-08], // SError interrupt mask
    I    [07-07], // IRQ interrupt mask
    F    [06-06], // FIQ interrupt mask
    M4   [04-04], // Execution state
    M    [03-00], // AArch64 Exception level and selected Stack Pointer

    RES0 [63-32|27-26|19-13|05-05],
]);

// (ref. D13.2.46: Hypervisor Configuration Register)
defreg!(HCR_EL2, [
    ID   [33-33], // Disables stage 2 instruction cache
    CD   [32-32], // Disables stage 2 data cache
    RW   [31-31], // Execution state control for lower Exception level
    TRVM [30-30], // Trap reads of Virtual Memory controls
    TDZ  [28-28], // Traps DC ZVA instruction
    TGE  [27-27], // Traps general exceptions
    TVM  [26-26], // Traps virtual memory controls
    TTLB [25-25], // Traps TLB maintenance instructions
    TPU  [24-24], // Traps cache maintenance instructions to Point of Unification (POU)
    TPC  [23-23], // Traps data or unified cache maintenance instructions to Point of Coherency (POC)
    TSW  [22-22], // Traps data or unified cache maintenance instructions by Set or Way
    TACR [21-21], // Traps Auxiliary Control registers
    TIDCP[20-20], // Traps Implementation Dependent functionality
    TSC  [19-19], // Traps SMC instruction.
    TID3 [18-18], // Traps ID group 3 
    TID2 [17-17], // Traps ID group 2
    TID1 [16-16], // Traps ID group 1
    TID0 [15-15], // Traps ID group 0
    TWE  [14-14], // Traps WFE instruction
    TWI  [13-13], // Traps WFI instruction
    DC   [12-12], // Default cacheable
    BSU  [11-10], // Barrier shareability upgrade
    FB   [09-09], // Forces broadcast
    VSE  [08-08], // Virtual System Error/Asynchronous Abort.
    VI   [07-07], // Virtual IRQ interrupt
    VF   [06-06], // Virtual FRQ interrupt
    AMO  [05-05], // Asynchronous abort and error interrupt routing
    IMO  [04-04], // Physical IRQ routing
    FMO  [03-03], // Physical FIQ routing
    PTW  [02-02], // Protected Table Walk
    VM   [00-00], // Virtualization enable

    RES0 [63-34|29-29],
    RES1 [01-01],
]);

defreg!(ELR_EL1);
defreg!(ELR_EL2);
defreg!(ELR_EL3);

defreg!(CPTR_EL2);
defreg!(CPACR_EL1);

// (ref. D13.2 Exception Syndrome Register)
defreg!(ESR_EL1, [
    EC   [31-26], // The Exception class field
    IL   [25-25], // The Instruction length bit
    ISS  [24-00], // The Instruction specific syndrome field

    ISS_HSVC_IMM [15-00], // An immediate value for HVC/SVC
    ISS_BRK_CMMT [15-00], // Comment
]);

// (ref. D13.2.39 Fault Address Register)
defreg!(FAR_EL1);
defreg!(FAR_EL2);
defreg!(FAR_EL3);

// (ref. D7.2.88 System Control Register)
defreg!(SCTLR_EL1, [
    UCI  [26-26], // Traps EL0 execution of cache maintenance instructions to EL1
    EE   [25-25], // Endianness of data accesses at EL1
    EOE  [24-24], // Endianness of data accesses at EL0
    WXN  [19-19], // Write permission implies XN (Execute-never)
    nTWE [18-18], // Traps EL0 execution of WFE instructions to EL1
    nTWI [16-16], // Traps EL0 execution of WFI instructions to EL1
    UCT  [15-15], // Traps EL0 accesses to the CTR_EL0 to EL1
    DZE  [14-14], // Traps EL0 execution of DC ZVA instructions to EL1
    I    [12-12], // Instruction access Cacheability control
    UMA  [09-09], // User Mask Access
    SED  [08-08], // SETEND instruction disable
    ITD  [07-07], // IT Disable
    CP15 [05-05], // System instruction memory barrier enable
    SA0  [04-04], // SP Alignment check enable for EL0
    SA   [03-03], // SP Alignment check enable.
    C    [02-02], // Cacheability control
    A    [01-01], // Alignment check enable
    M    [00-00], // MMU enable for EL1 and EL0 stage 1 address translation

    RES1 [29-28|23-22|20-20|11-11],
]);

defreg!(SP_EL0);
defreg!(SP_EL1);
defreg!(SP_EL2);
defreg!(SP_EL3);

// (ref. D1.7.1 PSTATE-related system registers)
defreg!(NZCV, [
    N    [31-31],
    Z    [30-30],
    C    [29-29],
    V    [28-28],
]);

defreg!(DAIF, [
    D    [9-9],
    A    [8-8],
    I    [7-7],
    F    [6-6],
]);

defreg!(SPSel, [
    SP   [00-00], // Stack pointer to use
]);

defreg!(CurrentEL, [
    EL   [3-2],
]);

defreg!(VBAR_EL1, [
    RES0   [10-0],
]);

defreg!(CNTHCTL_EL2, [
    EL0VCTEN [1-1],
    EL0PCTEN [0-0],
]);

defreg!(CNTVOFF_EL2);
