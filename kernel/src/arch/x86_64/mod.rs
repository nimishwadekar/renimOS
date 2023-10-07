pub mod interrupts;
mod gdt;
mod pic;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    pic::init();
    interrupts::enable();
}

#[inline]
pub fn hlt() {
    x86_64::instructions::hlt();
}