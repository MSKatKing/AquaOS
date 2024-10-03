#ifndef VIDEOMODE_DRIVER
#define VIDEOMODE_DRIVER

#include "../ports.c"

#define WIDTH 320
#define HEIGHT 200
#define GRAPHICS_ADDRESS 0xA0000

void set_graphics_mode() {
    pb_out(0x03D4, 0x00);
    pb_out(0x03D5, 0x13);
}

void clear_screen() {
    char* video_mem = (char*) GRAPHICS_ADDRESS;
    for(int x = 0; x < WIDTH; x++) {
        for(int y = 0; y < HEIGHT; y++) {
            video_mem[(y * 320) + x] = 0;
        }
    }
}

#endif