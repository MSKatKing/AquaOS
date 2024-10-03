#pragma once

inline __attribute__((always_inline)) void pb_out(unsigned short port, unsigned char byte) {
    __asm__("out %%al, %%dx" : : "a" (byte), "d" (port));
}

inline __attribute__((always_inline)) unsigned char pb_in(unsigned short port) {
    unsigned char result;

	__asm__("in %%dx, %%al" : "=a" (result) : "d" (port));

	return result;
}

inline __attribute__((always_inline)) void pw_out(unsigned short port, unsigned short word) {
    __asm__("outw %%ax, %%dx" : : "a" (word), "d" (port));
}

inline __attribute__((always_inline)) unsigned short pw_in(unsigned short port) {
    unsigned short result;

	__asm__("inw %%dx, %%ax" : "=a" (result) : "dN" (port));

	return result;
}

inline __attribute__((always_inline)) void pw_ins(unsigned short port, void* addr, int cnt) {
    __asm__ volatile("rep insw" : "+D"(addr), "+c"(cnt) : "d"(port) : "memory");
}

inline __attribute__((always_inline)) void pl_out(unsigned short port, unsigned int word) {
    __asm__("out %%eax, %%dx" : : "a" (word), "d" (port));
}

inline __attribute__((always_inline)) unsigned int pl_in(unsigned short port) {
    unsigned int result;

	__asm__("in %%dx, %%eax" : "=a" (result) : "d" (port));

	return result;
}