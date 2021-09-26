use libkernel::{
    addr_ty::{Physical, Virtual},
    memory::{falloc, malloc, volatile::VolatileCell},
    Address, ReadWrite,
};

use super::command::NVME_COMMAND;

pub struct CompletionQueue<'q> {
    frames: libkernel::memory::FrameIterator,
    entries: &'q [super::command::Completion],
    doorbell: &'q VolatileCell<u32, ReadWrite>,
    cur_index: u16,
    phase_tag: bool,
}

impl<'q> CompletionQueue<'q> {
    pub fn new(doorbell_addr: Address<Virtual>, entry_count: u16, entry_size: usize) -> Self {
        // TODO somehow validate entry size

        let size_in_bytes = (entry_count as usize) * entry_size;
        let minimum_frame_count = libkernel::align_up_div(size_in_bytes, 0x1000);
        let frames = falloc::get().autolock_many(minimum_frame_count).unwrap();
        let alloc_addr = malloc::get().alloc_to(&frames) as *mut _;

        unsafe {
            core::ptr::write_bytes(alloc_addr as *mut u8, 0, size_in_bytes);
            let entries = &mut *core::slice::from_raw_parts_mut(alloc_addr, entry_count as usize);

            Self {
                frames,
                doorbell: &*doorbell_addr.as_ptr(),
                entries,
                cur_index: 0,
                phase_tag: true,
            }
        }
    }

    pub fn base_phys_addr(&self) -> Address<Physical> {
        self.frames.start().base_addr()
    }

    fn increment_index(&mut self) {
        if (self.cur_index as usize) == (self.entries.len() - 1) {
            self.phase_tag = !self.phase_tag;
            self.cur_index = 0;
        } else {
            self.cur_index += 1;
        }

        self.doorbell.write(self.cur_index as u32);
    }

    pub fn next_entry(&mut self) -> Option<&super::command::Completion> {
        let completion = &self.entries[self.cur_index as usize];

        if completion.phase_tag() == self.phase_tag {
            self.increment_index();
            Some(completion)
        } else {
            None
        }
    }
}

pub struct SubmissionQueue<'q> {
    frames: libkernel::memory::FrameIterator,
    entries: &'q mut [NVME_COMMAND],
    doorbell: &'q VolatileCell<u32, ReadWrite>,
    cur_index: u16,
    unsubmitted_entries: u16,
}

impl<'q> SubmissionQueue<'q> {
    pub fn new(doorbell_addr: Address<Virtual>, entry_count: u16, entry_size: usize) -> Self {
        // TODO somehow validate entry size

        let size_in_bytes = (entry_count as usize) * entry_size;
        let minimum_frame_count = libkernel::align_up_div(size_in_bytes, 0x1000);
        let frames = falloc::get().autolock_many(minimum_frame_count).unwrap();
        let alloc_addr = malloc::get().alloc_to(&frames) as *mut _;

        unsafe {
            core::ptr::write_bytes(alloc_addr as *mut u8, 0, size_in_bytes);
            let entries = &mut *core::slice::from_raw_parts_mut(alloc_addr, entry_count as usize);

            Self {
                frames,
                doorbell: &*doorbell_addr.as_ptr(),
                entries,
                cur_index: 0,
                unsubmitted_entries: 0,
            }
        }
    }

    pub fn base_phys_addr(&self) -> Address<Physical> {
        self.frames.start().base_addr()
    }

    fn increment_index(&mut self) {
        if (self.cur_index as usize) == self.entries.len() {
            self.cur_index = 0;
        } else {
            self.cur_index += 1;
        }
    }

    pub fn next_entry<T: super::command::CommandType>(
        &mut self,
    ) -> Option<&mut super::command::Command<T>> {
        if (self.unsubmitted_entries as usize) < self.entries.len() {
            // transmute the &mut [u32] to an &mut NVMeCommand
            let entry: &mut super::command::Command<T> =
                unsafe { core::mem::transmute(&mut self.entries[self.cur_index as usize]) };
            entry.clear();
            // Automatically set opcode for command type T.
            entry.set_opcode(T::OPCODE);

            self.unsubmitted_entries += 1;
            self.increment_index();

            Some(entry)
        } else {
            None
        }
    }

    pub fn flush_commands(&mut self) {
        self.doorbell.write(self.cur_index as u32);
        self.unsubmitted_entries = 0;
    }
}

// TODO implement this
// impl Drop for SubmissionQueue<'_> {
//     fn drop(&mut self) {
//         unsafe {
//             libkernel::memory::malloc::get().dealloc(
//                 self.entries.as_mut_ptr() as *mut u8,
//                 core::alloc::Layout::from_size_align_unchecked(
//                     self.entries.len() * core::mem::size_of::<[u32; COMMAND_DWORDS]>(),
//                     1,
//                 ),
//             )
//         }
//     }
// }
