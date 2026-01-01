#[doc = r" Core interrupts. These interrupts are handled by the core itself."]
# [riscv :: pac_enum (unsafe CoreInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreInterrupt {
    #[doc = "3 - Machine Software Interrupt"]
    MachineSoft = 3,
    #[doc = "7 - Machine Timer Interrupt"]
    MachineTimer = 7,
    #[doc = "11 - Machine External Interrupt"]
    MachineExternal = 11,
    #[doc = "16 - TWD FIFO Level Interrupt"]
    TWD = 16,
    #[doc = "17 - CFS (user-defined) Interrupt"]
    CFS = 17,
    #[doc = "18 - UART0 FIFO Level Interrupt"]
    UART0 = 18,
    #[doc = "19 - UART1 FIFO Level Interrupt"]
    UART1 = 19,
    #[doc = "21 - Tracing Stop-Address Match Interrupt"]
    TRACER = 21,
    #[doc = "22 - SPI FIFO Level Interrupt"]
    SPI = 22,
    #[doc = "23 - TWI FIFO Level Interrupt"]
    TWI = 23,
    #[doc = "24 - GPIO FIFO Level Interrupt"]
    GPIO = 24,
    #[doc = "25 - NEOLED TX FIFO Level Interrupt"]
    NEOLED = 25,
    #[doc = "26 - DMA Transfer Done Interrupt"]
    DMA = 26,
    #[doc = "27 - SDI FIFO Level Interrupt"]
    SDI = 27,
    #[doc = "28 - General Purpose Timer Interrupt"]
    GPTMR = 28,
    #[doc = "29 - 1-Wire Idle Interrupt"]
    ONEWIRE = 29,
    #[doc = "30 - SLINK FIFO Level Interrupt"]
    SLINK = 30,
    #[doc = "31 - TRNG FIFO Level Interrupt"]
    TRNG = 31,
}
#[doc = r" Exception sources in the device."]
# [riscv :: pac_enum (unsafe ExceptionNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exception {
    #[doc = "0 - Instruction Address Misaligned"]
    InstructionMisaligned = 0,
    #[doc = "1 - Instruction Access Fault"]
    InstructionFault = 1,
    #[doc = "2 - Illegal Instruction"]
    IllegalInstruction = 2,
    #[doc = "3 - Software Breakpoint/HW Trigger"]
    Breakpoint = 3,
    #[doc = "4 - Load Address Misaligned"]
    LoadMisaligned = 4,
    #[doc = "5 - Load Access Fault"]
    LoadFault = 5,
    #[doc = "6 - Store Address Misaligned"]
    StoreMisaligned = 6,
    #[doc = "7 - Store Access Fault"]
    StoreFault = 7,
    #[doc = "8 - Environment Call From U-Mode"]
    UserEnvCall = 8,
    #[doc = "11 - Environment Call From M-Mode"]
    MachineEnvCall = 11,
}
#[doc = r" Priority levels in the device"]
# [riscv :: pac_enum (unsafe PriorityNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    #[doc = "1 - Priority level 1"]
    P1 = 1,
    #[doc = "2 - Priority level 2"]
    P2 = 2,
    #[doc = "3 - Priority level 3"]
    P3 = 3,
    #[doc = "4 - Priority level 4"]
    P4 = 4,
    #[doc = "5 - Priority level 5"]
    P5 = 5,
    #[doc = "6 - Priority level 6"]
    P6 = 6,
    #[doc = "7 - Priority level 7"]
    P7 = 7,
    #[doc = "8 - Priority level 8"]
    P8 = 8,
    #[doc = "9 - Priority level 9"]
    P9 = 9,
    #[doc = "10 - Priority level 10"]
    P10 = 10,
    #[doc = "11 - Priority level 11"]
    P11 = 11,
    #[doc = "12 - Priority level 12"]
    P12 = 12,
    #[doc = "13 - Priority level 13"]
    P13 = 13,
    #[doc = "14 - Priority level 14"]
    P14 = 14,
    #[doc = "15 - Priority level 15"]
    P15 = 15,
    #[doc = "16 - Priority level 16"]
    P16 = 16,
    #[doc = "17 - Priority level 17"]
    P17 = 17,
    #[doc = "18 - Priority level 18"]
    P18 = 18,
    #[doc = "19 - Priority level 19"]
    P19 = 19,
    #[doc = "20 - Priority level 20"]
    P20 = 20,
    #[doc = "21 - Priority level 21"]
    P21 = 21,
    #[doc = "22 - Priority level 22"]
    P22 = 22,
    #[doc = "23 - Priority level 23"]
    P23 = 23,
    #[doc = "24 - Priority level 24"]
    P24 = 24,
    #[doc = "25 - Priority level 25"]
    P25 = 25,
    #[doc = "26 - Priority level 26"]
    P26 = 26,
    #[doc = "27 - Priority level 27"]
    P27 = 27,
    #[doc = "28 - Priority level 28"]
    P28 = 28,
    #[doc = "29 - Priority level 29"]
    P29 = 29,
}
#[doc = r" HARTs in the device"]
# [riscv :: pac_enum (unsafe HartIdNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hart {
    #[doc = "0 - Hart 0"]
    H0 = 0,
    #[doc = "1 - Hart 1"]
    H1 = 1,
}
pub use riscv::{
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
    interrupt::{disable, enable, free, nested},
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
