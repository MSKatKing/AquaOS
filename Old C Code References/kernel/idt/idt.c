#pragma once

#include "../drivers/keyboard/basic.c"
#include <primitives.h>
#include "../api/display.c"

typedef enum {
    TRAP = 0b10001111,
    INTERRUPT = 0b10001110
} InterruptGate;

typedef void (*ISRFunction)(void*);

void install_idt();
void load_idt();
void register_isr(int irq, ISRFunction handler, InterruptGate gate_type);
void remove_isr(int irq);

void register_default_isr();

struct idtEntry {
    uint16 handler_low;
    uint16 selector;
    uint8 zero;
    uint8 attributes;
    uint16 handler_high;    
} __attribute__((packed));

typedef struct idtEntry InterruptDescriptor32;

typedef struct {
    uint16 handler_low;
    uint16 selector;
    uint8 ist;
    uint8 attributes;
    uint16 handler_mid;
    uint32 handler_high;
    uint32 zero;
} InterruptDescriptor64;

struct idtptr {
    uint16 limit;
    uint32 base;
} __attribute__((packed));

typedef struct idtptr IDTPtr;


/*
    Typically 0x30-0xFF are unused interrupts, so the kernel can use these for basic device functions
*/
InterruptDescriptor32 idt32[256];
InterruptDescriptor64 idt64[256];
IDTPtr idtp;

__attribute__((interrupt)) void default_isr(void* frame) {}

inline void load_idt() {
    idtp.limit = (sizeof(InterruptDescriptor32) * 256) - 1;
    idtp.base = (uint32) &idt32;

    __asm__("lidt %0" : : "m" (idtp));
}

void install_idt() {
    for(int i = 0; i < 256; i++) {
        remove_isr(i);
    }

    register_default_isr();

    load_idt();

    __asm__("sti");
}

__attribute__((interrupt)) void pic_isr(void* frame) {
    pb_out(0x20, 0x20);
}

__attribute__((interrupt)) void pic2_isr(void* frame) {
    pb_out(0xA0, 0x20);
}

inline void register_default_isr() {
    register_isr(0x20, pic_isr, INTERRUPT);
    register_isr(0x21, keyboard_isr, INTERRUPT);
    register_isr(0x22, pic_isr, INTERRUPT);
    register_isr(0x23, pic_isr, INTERRUPT);
    register_isr(0x24, pic_isr, INTERRUPT);
    register_isr(0x25, pic_isr, INTERRUPT);
    register_isr(0x26, pic_isr, INTERRUPT);
    register_isr(0x27, pic_isr, INTERRUPT);

    register_isr(0x28, pic2_isr, INTERRUPT);
    register_isr(0x29, pic2_isr, INTERRUPT);
    register_isr(0x2A, pic2_isr, INTERRUPT);
    register_isr(0x2B, pic2_isr, INTERRUPT);
    register_isr(0x2C, pic2_isr, INTERRUPT);
    register_isr(0x2D, pic2_isr, INTERRUPT);
    register_isr(0x2E, pic2_isr, INTERRUPT);
    register_isr(0x2F, pic2_isr, INTERRUPT);

    // Kernel interface

    // Display adapter
    register_isr(0x30, display_interface_isr, INTERRUPT);
}

void register_isr(int n, ISRFunction handler, InterruptGate type) {
    idt32[n].handler_low = ((uint32) handler) & 0xFFFF;
    idt32[n].selector = 0x08;
    idt32[n].zero = 0;
    idt32[n].attributes = type;
    idt32[n].handler_high = (((uint32) handler) >> 16) & 0xFFFF;
}

void remove_isr(int n) {
    register_isr(n, default_isr, INTERRUPT);
}

inline void interrupt(unsigned char in) {
    __asm__("int %0" : : "i" (in));
}
