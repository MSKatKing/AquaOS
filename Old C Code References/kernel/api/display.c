#pragma once

#include "../drivers/display/vga_textmode.c"

/*
    INT 0x30 - Display driver interface

    AX = 0x00 - Print string to the current cursor location; EBX contains the pointer to the string
    AX = 0x01 - Clears the screen
*/
__attribute__((interrupt)) void display_interface_isr(void* frame) {
    unsigned short operation;
    __asm__("movw %%ax, %0\n" : "=r"(operation) : : "%ax");

    switch(operation) {
        case 0x00: {
            const char* str;
            __asm__("movl %%ebx, %0\n" : "=r"(str) : : "%ebx");

            print_string(str);
            break;
        }
        case 0x01: {
            clear_screen();
            break;
        }

        default: {
            print_string("Unknown operation code.");
        }
    }
}