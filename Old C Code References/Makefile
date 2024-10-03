EMULATOR=qemu-system-x86_64

CFLAGS?=-O2 -g
CPPFLAGS?=
LDFLAGS?=
LIBS?=

CFLAGS:=$(CFLAGS) -ffreestanding -Wall -Wextra -fno-pie -mgeneral-regs-only
CPPFLAGS:=$(CPPFLAGS) -D__is_kernel -Iinclude
LDFLAGS:=$(LDFLAGS)
LIBS:=$(LIBS) -nostdlib -lk -lgcc

all:
	rm -rf build
	mkdir -p build
	nasm -f bin bootloader/boot.asm -o build/bootloader.bin

	nasm -f elf32 kernel/kernel_entry.asm -o build/kernel_entry.o
	gcc -Iinclude -mgeneral-regs-only -fno-pie -m32 -nostdlib -ffreestanding -Os -s -c kernel/main.c -o build/main.o

	ld -Ttext 0x1000 -m elf_i386 --oformat binary -o build/kernel.bin build/kernel_entry.o build/main.o

	bash generate_kfs.sh

	gcc -m32 -fno-pie -nostdlib -ffreestanding -Os -s -c example_program/main.c -o build/example.o

	ld -Ttext 0x9000 -m elf_i386 --oformat binary -o build/example.bin build/example.o

	cat build/bootloader.bin build/kfs.bin build/kernel.bin > os.bin
	cat build/kfs.bin build/kernel.bin > kernel.bin
	qemu-system-x86_64 -device piix3-ide,id=ide -drive id=disk,file=os.bin,format=raw,if=none -device ide-hd,drive=disk,bus=ide.0