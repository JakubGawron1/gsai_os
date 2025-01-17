#[cfg(target_arch = "x86_64")]
use crate::arch::x64;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ControlContext {
    pub ip: u64,
    pub sp: u64,
}

#[cfg(target_arch = "x86_64")]
pub type ArchContext = (x64::registers::GeneralRegisters, x64::registers::SpecialRegisters);
#[cfg(target_arch = "x86_64")]
pub type SyscallContext = x64::registers::PreservedRegistersSysv64;

#[cfg(target_arch = "x86_64")]
pub fn default_arch_context() -> ArchContext {
    (
        x64::registers::GeneralRegisters::empty(),
        x64::registers::SpecialRegisters::with_kernel_segments(x64::registers::RFlags::INTERRUPT_FLAG),
    )
}
