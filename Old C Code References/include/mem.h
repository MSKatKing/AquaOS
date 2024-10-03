#pragma once

/*
    Copies data from source into dest. WARNING: if the memory less than dest + length is being used, it will be overridden
*/
void memcpy(unsigned char* source, unsigned char* dest, int length) {
    for(int i = 0; i < length; i++) {
        dest[i] = source[i];
    }
}

/*
    Swaps data from source and dest for a length amount of bytes.
*/
void memswp(unsigned char* a, unsigned char* b, int length) {
    for(int i = 0; i < length; i++) {
        char temp = a[i];
        a[i] = b[i];
        b[i] = temp;
    }
}

/*
    Fills the memory at dest with the byte defined by fill for length amount of bytes.
*/
void memfill(unsigned char* dest, unsigned char fill, int length) {
    for(int i = 0; i < length; i++) {
        dest[i] = fill;
    }
}