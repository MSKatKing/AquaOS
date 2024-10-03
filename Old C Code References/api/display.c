#pragma once

/*
    Prints a string to the screen at the current cursor location

    Updates the cursor location to the end of the string when finished printing
*/
void print(const char* str) {
    __asm__(
        "movw $0x00, %%ax\n"
        "movl %0, %%ebx\n"
        "int $0x30\n"
        :
        : "r"(str)
        : "%ax", "%ebx"
    );
}

/*
    Prints a string to the screen at the current cursor location

    Updates the cursor location to the end of the string when finished printing
*/
void println(const char* str) {
    print(str);
    print("\n");
}

void clear_screen() {
    __asm__(
        "movw $0x01, %%ax\n"
        :
        :
        : "%ax"
    );
}