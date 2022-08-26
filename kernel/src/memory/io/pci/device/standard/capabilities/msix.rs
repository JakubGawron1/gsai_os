use crate::{
    io::pci::{standard::StandardRegister, PCIeDevice, Standard},
    memory::volatile::{Volatile, VolatileCell},
    volatile_bitfield_getter, InterruptDeliveryMode, ReadOnly, ReadWrite,
};
use bit_field::BitField;
use core::{convert::TryFrom, fmt};

#[repr(C)]
pub struct MessageControl {
    reg0: VolatileCell<u32, ReadWrite>,
}

impl MessageControl {
    pub fn get_table_len(&self) -> usize {
        self.reg0.read().get_bits(16..27) as usize
    }

    volatile_bitfield_getter!(reg0, force_mask, 30);
    volatile_bitfield_getter!(reg0, enable, 31);
}

impl fmt::Debug for MessageControl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Message Control")
            .field("Enabled", &self.get_enable())
            .field("Force Mask", &self.get_force_mask())
            .field("Table Size", &self.get_table_len())
            .finish()
    }
}

#[repr(C)]
pub struct Message {
    addr_low: VolatileCell<u32, ReadWrite>,
    addr_high: VolatileCell<u32, ReadWrite>,
    data: VolatileCell<u32, ReadWrite>,
    mask: VolatileCell<u32, ReadWrite>,
}

impl Message {
    pub fn get_masked(&self) -> bool {
        self.mask.read().get_bit(0)
    }

    pub fn set_masked(&self, masked: bool) {
        self.mask.write(*self.mask.read().set_bit(0, masked));
    }

    // TODO features gate this function behind x86, because its contents are arch-specific
    pub fn configure(&self, processor_id: u8, vector: u8, delivery_mode: InterruptDeliveryMode) {
        assert!(
            self.get_masked(),
            "Cannot modify MSI-X message when it is unmasked."
        );
        assert!(vector > 0xF, "MSI-X message vector cannot be <=0xF.");

        let mut data = 0;
        data.set_bits(0..8, vector as u32);
        data.set_bits(8..11, delivery_mode as u32);
        data.set_bit(14, false);
        data.set_bit(15, false);
        data.set_bits(16..32, 0);

        let mut addr = 0;
        addr.set_bits(0..2, 0);
        addr.set_bit(2, false);
        addr.set_bit(3, false);
        addr.set_bits(12..20, processor_id as u32);
        addr.set_bits(20..32, 0xFEE);

        self.data.write(data);
        self.addr_low.write(addr as u32);
        // High address is reserved (zeroed) in x86.
        self.addr_high.write(0);
    }
}

impl Volatile for Message {}

impl fmt::Debug for Message {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Message Table Entry")
            .field("Masked", &self.get_masked())
            .field(
                "Address",
                &format_args!(
                    "0x{:X}",
                    ((self.addr_high.read() as u64) << 32) | (self.addr_low.read() as u64)
                ),
            )
            .field("Data", &format_args!("0b{:b}", self.data.read()))
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingBit {
    Unset,
    Set,
}

impl From<usize> for PendingBit {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Unset,
            1 => Self::Set,
            value => panic!("Invalid pending bit value: {}", value),
        }
    }
}

impl Into<usize> for PendingBit {
    fn into(self) -> usize {
        match self {
            PendingBit::Unset => 0,
            PendingBit::Set => 1,
        }
    }
}

impl crate::collections::bv_array::BitValue for PendingBit {
    const BIT_WIDTH: usize = 0x1;
    const MASK: usize = 0x1;
}

#[repr(C)]
struct Data {
    reg0: VolatileCell<u32, ReadWrite>,
    reg1: VolatileCell<u32, ReadOnly>,
    reg2: VolatileCell<u32, ReadOnly>,
}

impl Data {
    pub fn get_enable(&self) -> bool {
        self.reg0.read().get_bit(31)
    }

    pub fn set_enable(&self, enable: bool) {
        self.reg0.write(*self.reg0.read().set_bit(31, enable));
    }

    pub fn get_function_mask(&self) -> bool {
        self.reg0.read().get_bit(30)
    }

    pub fn set_function_mask(&self, mask_all: bool) {
        self.reg0.write(*self.reg0.read().set_bit(30, mask_all));
    }

    fn get_table_len(&self) -> usize {
        // Field is encoded as N-1, so add one to get N (table length).
        (self.reg0.read().get_bits(16..27) as usize) + 1
    }

    fn get_table_info(&self) -> (StandardRegister, usize) {
        let reg1 = self.reg1.read();

        (
            StandardRegister::try_from(reg1.get_bits(0..3) as usize).unwrap(),
            (reg1 & !0b111) as usize,
        )
    }

    fn get_pending_info(&self) -> (StandardRegister, usize) {
        let reg2 = self.reg2.read();

        (
            StandardRegister::try_from(reg2.get_bits(0..3) as usize).unwrap(),
            (reg2 & !0b111) as usize,
        )
    }
}

pub struct MSIX<'dev> {
    data: &'dev Data,
    messages: &'dev [Message],
}

impl<'dev> MSIX<'dev> {
    pub(in crate::io::pci::device::standard) fn try_new(
        device: &'dev PCIeDevice<Standard>,
    ) -> Option<Self> {
        device
            .capabilities()
            .find(|(_, capability)| *capability == super::Capablities::MSIX)
            .map(|(addr, _)| {
                let data = unsafe { addr.as_ptr::<Data>().as_ref() }.unwrap();
                let (msg_register, msg_offset) = data.get_table_info();

                Self {
                    data,
                    messages: device
                        .get_register(msg_register)
                        .map(|mmio| unsafe {
                            mmio.slice(msg_offset, data.get_table_len()).unwrap()
                        })
                        .expect(
                            "Device does not have requisite BARs to construct MSIX capability.",
                        ),
                }
            })
    }

    pub fn get_enable(&self) -> bool {
        self.data.get_enable()
    }

    pub fn set_enable(&self, enable: bool) {
        self.data.set_enable(enable);
    }

    pub fn get_function_mask(&self) -> bool {
        self.data.get_function_mask()
    }

    pub fn set_function_mask(&self, mask_all: bool) {
        self.data.set_function_mask(mask_all);
    }
}

impl core::ops::Index<usize> for MSIX<'_> {
    type Output = Message;

    fn index(&self, index: usize) -> &Self::Output {
        &self.messages[index]
    }
}

impl fmt::Debug for MSIX<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MSI-X")
            .field("Enabled", &self.get_enable())
            .field("Function Mask", &self.get_function_mask())
            .field("Messages", &self.messages)
            .finish()
    }
}