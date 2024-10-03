#pragma once

#define VIDEO_MEMORY 0xB8000
#define SCREEN_MEMORY 0xB9000
#define ROW_SIZE 0xA0

#define SCREEN_HEIGHT 25
#define SCREEN_WIDTH 80

#define VGA_CTRL_REGISTER 0x3D4
#define VGA_DATA_REGISTER 0x3D5
#define VGA_OFFSET_LOW 0x0F
#define VGA_OFFSET_HIGH 0x0E

#include <primitives.h>
#include <string.h>
#include <ports.h>
#include <mem.h>
#include <stdarg.h>

uint32 CURSOR_X = 0;
uint32 CURSOR_Y = 0;

uint32 startRow = 0;
uint32 endRow = 0;

uint8 color = 0x07;

typedef enum {
    BLACK = 0x0,
    BLUE = 0x1,
    GREEN = 0x2,
    CYAN = 0x3,
    RED = 0x4,
    MAGENTA = 0x5,
    BROWN = 0x6,
    LIGHT_GRAY = 0x7,
    DARK_GRAY = 0x8,
    LIGHT_BLUE = 0x9,
    LIGHT_GREEN = 0xA,
    LIGHT_CYAN = 0xB,
    LIGHT_RED = 0xC,
    LIGHT_MAGENTA = 0xD,
    LIGHT_YELLOW = 0xE,
    WHITE = 0xF
} Color;

/*
    Screen functions
*/
void print_char_at(const uint8 c, const uint32 x, const uint32 y);
void print_char(const uint8 c);
void clear_char(const uint32 x, const uint32 y);

void print_string_at(const char* str, uint32 x, uint32 y);
void print_string(const char* str);
void print_int(uint32 num);
void print_hex(uint32 num);

void draw_box(const uint32 x, const uint32 y, const uint8 width, const uint8 height, const bool double_line);
void clear_screen();
void newline();
void clear_line(const uint32 y);
void backspace();

void scroll_up();
void scroll_down();

void set_color(Color foreground, Color background);

void printf(const char* format, ...);

/*
    Cursor functions
*/
void set_cursor_pos(const uint32 x, const uint32 y);
uint32 get_cursor_x();
uint32 get_cursor_y();

void move_cursor_left();
void move_cursor_right();
void move_cursor_up();
void move_cursor_down();

void enable_cursor(uint8 start, uint8 end);
void disable_cursor();


/*
    Prints a character to the screen at these coordinates: (x, y)
*/
void print_char_at(const uint8 c, const uint32 x, const uint32 y) {
    uint32 pos = (y * SCREEN_WIDTH) + x;

    ((uint8*) SCREEN_MEMORY)[pos * 2] = c;
    ((uint8*) SCREEN_MEMORY)[pos * 2 + 1] = color;

    if(y >= startRow && y < (startRow + SCREEN_HEIGHT)) {
        ((uint8*) VIDEO_MEMORY)[pos * 2] = c;
        ((uint8*) VIDEO_MEMORY)[pos * 2 + 1] = color;
    }
}

/*
    Prints a character to the screen at the current cursor position. Then, moves the cursor right.
*/
void print_char(const uint8 c) {
    if(c == '\n') {
        newline();
        return;
    }

    print_char_at(c, get_cursor_x(), get_cursor_y());

    move_cursor_right();
}

/*
    Clears a character at these coordinates: (x, y)
*/
void clear_char(const uint32 x, const uint32 y) {
    print_char_at(0, x, y);
}

/*
    Prints a string starting at these coordinates: (x, y), moving to the next lines if needed.
*/
void print_string_at(const char* str, uint32 x, uint32 y) {
    uint32 i = 0;
    while(str[i]) {
        print_char_at(str[i++], x, y);
        x++;
        if(x >= SCREEN_WIDTH) {
            x = 0;
            y++;
        }
    }
}

/*
    Prints a string to the current cursor location, and then moves the cursor to the end of the string.
*/
void print_string(const char* str) {
    uint32 i = 0;
    while(str[i]) {
        print_char(str[i++]);
    }
}

/*
    Prints an integer to the current cursor location, then moves the cursor to the end of the int.
*/
void print_int(uint32 num) {
    if(num == 0) {
        print_char('0');
        return;
    }

    uint8 buffer[11];
    uint8 i = 0;
    while(num > 0) {
        buffer[i++] = (num % 10) + '0';
        num /= 10;
    }

    while(i > 0) {
        print_char(buffer[--i]);
    }
}

/*
    Prints an integer to the current cursor location in hex form, then moves the cursor to the end of the int.
*/
void print_hex(uint32 num) {
    print_char('0');
    print_char('x');

    const char hex_chars[] = "0123456789ABCDEF";
    for(int i = 28; i >= 0; i -= 4) {
        uint8 nibble = (num >> i) & 0xF;
        print_char(hex_chars[nibble]);
    }
}

/*
    Draws a box at this location: (x, y), with size: (width, height). Then moves the cursor inside the box
*/
void draw_box(const uint32 x, const uint32 y, const uint8 width, const uint8 height, const bool double_line) {
    print_char_at(double_line ? 0xC9 : 0xDA, x, y);
    print_char_at(double_line ? 0xBB : 0xBF, x + width, y);
    print_char_at(double_line ? 0xC8 : 0xC0, x, y + height);
    print_char_at(double_line ? 0xBC : 0xD9, x + width, y + height);

    // Top + Bottom
    for(int dx = x + 1; dx < x + width; dx++) {
        print_char_at(double_line ? 0xCD : 0xC4, dx, y);
        print_char_at(double_line ? 0xCD : 0xC4, dx, y + height);
    }

    // Left + Right
    for(int dy = y + 1; dy < y + height; dy++) {
        print_char_at(double_line ? 0xBA : 0xB3, x, dy);
        print_char_at(double_line ? 0xBA : 0xB3, x + width, dy);
    }

    set_cursor_pos(x + 1, y + 1);
}

/*
    Clears the screen memory AND the current video memory. Resets it all back to default colors.
*/
void clear_screen() {
    memfill((uint8*) SCREEN_MEMORY, 0, (startRow + SCREEN_HEIGHT) * ROW_SIZE);
    for(int x = 0; x < SCREEN_WIDTH; x++) {
        for(int y = 0; y < SCREEN_HEIGHT; y++) {
            ((uint8*) SCREEN_MEMORY)[((y * SCREEN_WIDTH) + x) * 2 + 1] = 0x07;
        }
    }
    memcpy((uint8*) SCREEN_MEMORY, (uint8*) VIDEO_MEMORY, 4000);
    startRow = 0;
    endRow = 0;

    set_cursor_pos(0, 0);
}

/*
    Moves the cursor to the start of the next line
*/
void newline() {
    move_cursor_down();
    set_cursor_pos(0, get_cursor_y());
}

/*
    Clears the video memory and screen memory at line y
*/
void clear_line(const uint32 y) {
    memfill((uint8*) (SCREEN_MEMORY + (y * ROW_SIZE)), 0, ROW_SIZE);
    
    if(y >= startRow && y < (startRow + SCREEN_HEIGHT)) {
        memfill((uint8*) (VIDEO_MEMORY + ((y - startRow) * ROW_SIZE)), 0, ROW_SIZE);
    }
}

/*
    Moves the cursor back and delete the character
*/
void backspace() {
    move_cursor_left();

    clear_char(get_cursor_x(), get_cursor_y());
}

/*
    Scrolls the screen up
*/
void scroll_up() {
    if(startRow == 0) return;
    startRow--;

    memcpy((uint8*) (SCREEN_MEMORY + (startRow * ROW_SIZE)), (uint8*) VIDEO_MEMORY, 4000);
}

/*
    Scrolls the screen down
*/
void scroll_down() {
    if(startRow + 1 == 0) return;
    startRow++;

    memcpy((uint8*) (SCREEN_MEMORY + (startRow * ROW_SIZE)), (uint8*) VIDEO_MEMORY, 4000);
}

/*
    Changes the current writing color to the new color.
*/
void set_color(Color foreground, Color background) {
    color = (foreground << 4) | (background & 0x0F);
}

void printf(const char* format, ...) {
    va_list args;
    va_start(args, format);

    uint32 i = 0;
    while(format[i]) {
        if(format[i] == '%') {
            i++;
            switch (format[i]) {
                case 'd':
                case 'i':
                    print_int(va_arg(args, int));
                    break;
                case 'x':
                    print_hex(va_arg(args, int));
                    break;
                case 's':
                    print_string(va_arg(args, uint8*));
                    break;
                case 'c':
                    print_char(va_arg(args, int));
                    break;
            
                default:
                    print_char('%');
                    print_char(format[i]);
                    break;
            }
        } else {
            print_char(format[i]);
        }
        i++;
    }
}

void set_cursor_pos(const uint32 x, const uint32 y) {
    CURSOR_X = x;
    CURSOR_Y = y;

    uint16 pos = ((y - startRow) * SCREEN_WIDTH) + x;

    uint8 upper = (pos >> 8) & 0xFF;
    uint8 lower = pos & 0xFF;

    pb_out(VGA_CTRL_REGISTER, VGA_OFFSET_HIGH);
    pb_out(VGA_DATA_REGISTER, upper);
    pb_out(VGA_CTRL_REGISTER, VGA_OFFSET_LOW);
    pb_out(VGA_DATA_REGISTER, lower);
}

uint32 get_cursor_x() {
    return CURSOR_X;
}

uint32 get_cursor_y() {
    return CURSOR_Y;
}

void move_cursor_up() {
    if(get_cursor_y() == 0) return;
    uint8 newY = get_cursor_y() - 1;
    if(newY < startRow) {
        scroll_up();
    }

    set_cursor_pos(get_cursor_x(), newY);
}

void move_cursor_down() {
    uint8 newY = get_cursor_y() + 1;
    if(newY >= startRow + SCREEN_HEIGHT) {
        if(newY > endRow) endRow++;
        scroll_down();
    }

    set_cursor_pos(get_cursor_x(), newY);
}

void move_cursor_left() {
    uint8 newX = get_cursor_x() - 1;
    if(newX >= SCREEN_WIDTH) {
        newX = SCREEN_WIDTH - 1;
        set_cursor_pos(newX, get_cursor_y());
        move_cursor_up();
        return;
    }

    set_cursor_pos(newX, get_cursor_y());
}

void move_cursor_right() {
    uint8 newX = get_cursor_x() + 1;
    if(newX >= SCREEN_WIDTH) {
        newX = 0;
        set_cursor_pos(newX, get_cursor_y());
        move_cursor_down();
        return;
    }
    
    set_cursor_pos(newX, get_cursor_y());
}

void enable_cursor(uint8 start, uint8 end) {
    pb_out(0x3D4, 0x0A);
    pb_out(0x3D5, (pb_in(0x3D5) & 0xC0) | start);

    pb_out(0x3D4, 0x0B);
    pb_out(0x3D5, (pb_in(0x3D5) & 0xE0) | end);
}

void disable_cursor() {
    pb_out(0x3D4, 0x0A);
    pb_out(0x3D5, 0x20);
}

uint8 get_char_at(const uint32 x, const uint32 y) {
    uint16 pos = (y * SCREEN_WIDTH) + x;

    return ((uint8*) SCREEN_MEMORY)[pos * 2];
}