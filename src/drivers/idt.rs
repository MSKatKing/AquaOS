use core::arch::asm;
use kernel_proc::interrupt;
use crate::drivers::io::keyboard::keyboard_isr;
use crate::drivers::ports::Port;
use crate::drivers::timing::pit_interrupt_handler;

#[repr(u8)]
pub enum InterruptGate {
    TRAP = 0b10001111,
    INTERRUPT = 0b10001110
}

pub type ISRFunction = unsafe extern "C" fn() -> !;

#[repr(C, packed)]
#[derive(Default, Copy, Clone)]
struct IDTEntry {
    handler_low: u16,
    selector: u16,
    ist: u8,
    attributes: u8,
    handler_mid: u16,
    handler_high: u32,
    zero: u32
}

impl IDTEntry {
    fn new(gate_type: InterruptGate, handler: ISRFunction) -> Self {
        let handler = handler as u64;

        Self {
            handler_low: (handler & 0xFFFF) as u16,
            selector: 0x08,
            zero: 0,
            attributes: gate_type as u8,
            handler_mid: ((handler >> 16) & 0xFFFF) as u16,
            handler_high: ((handler >> 32) & 0xFFFFFFFF) as u32,
            ist: 0 // TODO: change this to use a specific stack for each interrupt
        }
    }
}

#[repr(C, align(16))]
pub struct IDT([IDTEntry;256]);

impl IDT {
    pub fn new() -> Self {
        Self([IDTEntry::default();256])
    }

    pub fn register_isr(&mut self, idx: usize, handler: ISRFunction) {
        self.0[idx] = IDTEntry::new(InterruptGate::INTERRUPT, handler);
    }

    pub fn register_default_isr(&mut self) {
        self.register_isr(0x20, pit_interrupt_handler);
        self.register_isr(0x21, keyboard_isr);
        self.register_isr(0x22, pic_isr);
        self.register_isr(0x23, pic_isr);
        self.register_isr(0x24, pic_isr);
        self.register_isr(0x25, pic_isr);
        self.register_isr(0x26, pic_isr);
        self.register_isr(0x27, pic_isr);

        self.register_isr(0x28, pic2_isr);
        self.register_isr(0x29, pic2_isr);
        self.register_isr(0x2A, pic2_isr);
        self.register_isr(0x2B, pic2_isr);
        self.register_isr(0x2C, pic2_isr);
        self.register_isr(0x2D, pic2_isr);
        self.register_isr(0x2E, pic2_isr);
        self.register_isr(0x2F, pic2_isr);
    }

    pub fn load(&self) {
        #[repr(C, packed)]
        struct IDTPtr {
            limit: u16,
            base: u64
        }

        let idt_ptr = IDTPtr {
            limit: (size_of::<IDTEntry>() as u16 * 256) - 1,
            base: self as *const _ as u64
        };

        unsafe {
            asm!(
                "lidt [{}]",
                "sti",
                in(reg) &idt_ptr,
                options(nostack)
            )
        }
    }
}

#[interrupt]
fn pic_isr() {
    Port::new(0x20).write(0x20u8);
}

#[interrupt]
fn pic2_isr() {
    Port::new(0xA0).write(0x20u8);
}