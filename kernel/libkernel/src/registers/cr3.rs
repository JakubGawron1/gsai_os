use bitflags::bitflags;
use x86_64::PhysAddr;

use crate::memory::Frame;

bitflags! {
    pub struct CR3Flags : u64 {
        const PAGE_LEVEL_WRITE_THROUGH = 1 << 3;
        const PAGE_LEVEL_CACHE_DISABLE = 1 << 4;
    }
}

pub struct CR3;

impl CR3 {
    #[inline(always)]
    pub unsafe fn write(frame: &Frame, flags: Option<CR3Flags>) {
        let addr = frame.addr().as_u64();
        let flags = match flags {
            Some(some) => some.bits(),
            None => 0,
        };

        asm!("mov cr3, {}", in(reg) addr | flags, options(nostack));
    }

    #[inline(always)]
    pub fn read() -> (Frame, Option<CR3Flags>) {
        let value: u64;

        unsafe {
            asm!("mov {}, cr3", out(reg) value, options(nostack));
        }

        (
            Frame::from_addr(PhysAddr::new(value & !0xFFF)),
            CR3Flags::from_bits(value),
        )
    }
}