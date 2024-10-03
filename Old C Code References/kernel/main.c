#include "drivers/display/vga_textmode.c"
//#include "drivers/display/videomode.c"
#include "drivers/keyboard/basic.c"
#include "idt/idt.c"
#include "drivers/disk/ide.c"

#include <fs/ide.h>

int main() {
    //switch_to_32_pm(); // Not working, manually switching to 32-bit mode before loading kernel.

    unsigned char DRIVE;
	__asm__("movb %%al, %0" : "=q" (DRIVE));

    pb_out(0x20, 0x11);
    pb_out(0xA0, 0x11);
    pb_out(0x21, 0x20);
    pb_out(0xA1, 0x28);
    pb_out(0x21, 0x04);
    pb_out(0xA1, 0x02);
    pb_out(0x21, 0x01);
    pb_out(0xA1, 0x01);
    pb_out(0x21, 0x0);
    pb_out(0xA1, 0x0);

    pb_out(0x64, 0xAE);
    pb_out(0x60, 0xF4);

    install_idt();

    //init_ide();
    //initialize_ahci();
    ata_reset();

    disable_cursor();
    enable_cursor(0, 15);

    clear_screen();

    print_string("Welcome to the AquaOS kernel!");
    newline();
    print_string("> ");

    uint8 buffer[512];
    uint8 READ = 1;
    ata_pio_read(1, 1, buffer);
    newline();
    for(int i = 0; i < 512 * READ; i += 4) {
        printf("%x-%x: %x %x %x %x", i, i + 3, buffer[i], buffer[i + 1], buffer[i + 2], buffer[i + 3]);
        newline();
    }

    while(true);

    return 0;
}