use pic8259::ChainedPics;
use crate::spinlock::SpinLock;

pub const PIC1_OFFSET: u8 = 0x20;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

static PICS: SpinLock<ChainedPics> = SpinLock::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });

pub fn init() {
    unsafe {
        let mut pics = PICS.lock();
        pics.initialize();
        pics.write_masks(0xff, 0xff); // Disable all hardware interrupts currently.
    }
}

pub fn end_of_interrupt(interrupt: InterruptIndex) {
    unsafe { PICS.lock().notify_end_of_interrupt(interrupt.as_u8()); }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        (self as u8) as usize
    }
}