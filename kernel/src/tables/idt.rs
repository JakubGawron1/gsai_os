use x86_64::structures::idt::InterruptDescriptorTable;

/// Loads the IDT from the interrupt descriptor table register.
pub unsafe fn get_current() -> Option<&'static mut InterruptDescriptorTable> {
    let idt_pointer = x86_64::instructions::tables::sidt();
    idt_pointer.base.as_mut_ptr::<InterruptDescriptorTable>().as_mut()
}
