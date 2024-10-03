#pragma once

#include <ports.h>
#include "../display/vga_textmode.c"
#include <mem.h>
#include <primitives.h>

#define KEYBOARD_DATA_PORT 0x60
#define KEYBOARD_STATUS_PORT 0x64

#define KEY_RELEASE_MASK 0x80

const unsigned char scancode_lookup[0x54] = {
    0, /* Err */ 0, /* Esc */ '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '=', 0, // Backspace
    0, /* Tab */ 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']',
    0, // Enter
    0, // LCtrl
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '\'', '`',
    0, // LShift
    '\\',
    'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/', 0, // RShift
    '*', // Keypad
    0 /* LAlt */, ' ',
    0, // Caps Lock
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //F1-F10
    0, // Numlock
    0, // ScrollLock
    0, 0, 0, // Keypad 7-9
    '-', // Keypad -
    0, 0, 0, '+', // Keypad 4-6, +
    0, 0, 0, // Keypad 1-3
    0, '.' // Keypad 0, .
};

const unsigned char scancode_lookup_cap[0x54] = {
    0, /* Err */ 0, /* Esc */ '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', 0, // Backspace
    0, /* Tab */ 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '{', '}',
    0, // Enter
    0, // LCtrl
    'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', ':', '"', '~',
    0, // LShift
    '|',
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', '<', '>', '?', 0, // RShift
    '*', // Keypad
    0 /* LAlt */, ' ',
    0, // Caps Lock
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //F1-F10
    0, // Numlock
    0, // ScrollLock
    0, 0, 0, // Keypad Home, Up, PgUp
    '-', // Keypad -
    0, 0, 0, '+', // Keypad Left, 5, Right, +
    0, 0, 0, // Keypad End, Down, PgDn
    0, 0 // Keypad Ins, Del
};

typedef void (*command_handler)();

typedef struct {
    const uint8* name;
    command_handler handler;
} Command;

bool rshift = false;
bool lshift = false;
bool caps = false;

unsigned char keyboard_buffer[256];
unsigned char index = 0;

__attribute__((interrupt)) void keyboard_isr(void* frame) {
    pb_out(0x20, 0x20);

    unsigned char in = pb_in(KEYBOARD_DATA_PORT);

    if(in & KEY_RELEASE_MASK) {
        in = in & (~((unsigned char) KEY_RELEASE_MASK));

        switch (in) {
            case 0x36:
                rshift = false;
                break;
            case 0x2a:
                lshift = false;
                break;

            default:
                break;
        }

        return;
    }

    switch (in) {
        case 0x36:
            rshift = true;
            break;
        case 0x2a:
            lshift = true;
            break;

        case 0x3a:
            caps = caps ? false : true;
            break;

        case 0x0E: 
            if(get_char_at(0, get_cursor_y()) == '>' && get_cursor_x() == 2) break;
            backspace();
            keyboard_buffer[--index] = 0;
            break;

        case 0x1C:
            newline();
            if(strequ(keyboard_buffer, "?")) {
                print_string("set: Coming soon: set memory addresses to certain number!");
                newline();
                print_string("cls: Clears the terminal");
                newline();
                print_string("run <prgm>: Runs a program");
                newline();
            } else if(strequ(keyboard_buffer, "set")) {
                print_string("Coming soon: set memory addresses to certain numbers!");
                newline();
            } else if(strequ(keyboard_buffer, "cls")) {
                clear_screen();
            } else if(starts_with(keyboard_buffer, "run")) {
                if(keyboard_buffer[4] != 0) {
                    print_string("Unknown program '");
                    print_string(&keyboard_buffer[4]);
                    print_string("'");
                    newline();
                } else {
                    print_string("Allows you to run a program! Just type in the name of a file and you're off!");
                    newline();
                }
            } else {
                print_string("Unknown command: ");
                for(int i = 0; i < index; i++) print_char(keyboard_buffer[i]);
                newline();
            }

            for(char i = 0; i < 80; i++) {
                keyboard_buffer[i] = 0;
            }
            index = 0;

            print_string("> ");
            break;

        case 0x4b:
            if(get_char_at(0, get_cursor_y()) == '>' && get_cursor_x() == 2) break;
            move_cursor_left();
            index--;
            break;

        case 0x4d:
            move_cursor_right();
            index++;
            break;

        case 0x48:
            move_cursor_up();
            break;

        case 0x50:
            move_cursor_down();
            break;

        default:
            unsigned char character = ((rshift || lshift) ^ caps) ? scancode_lookup_cap[in] : scancode_lookup[in];
            if(!character) break;

            print_char(character);
            keyboard_buffer[index++] = character;
            break;
    }
}