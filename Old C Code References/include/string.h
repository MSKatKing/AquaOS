#pragma once

#include "primitives.h"

uint32 strlen(const uint8* str);

/*
    Compares the two char* and if they are the same true is returned, else false.
*/
bool strequ(const uint8* a, const uint8* b) {
    if(strlen(a) != strlen(b)) return false;
    for(int i = 0; i < strlen(a); i++)
        if(a[i] != b[i]) 
            return false;
    return true;
}

/*
    Gets the length of the null-terminated char*
*/
uint32 strlen(const uint8* str) {
    uint32 i = 0;
    while(str[i++]);
    return i - 1;
}

bool starts_with(const uint8* a, const uint8* b) {
    if(strlen(b) > strlen(a)) return false;
    for(int i = 0; i < strlen(b); i++)
        if(a[i] != b[i]) 
            return false;
    return true;
}