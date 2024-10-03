[bits 16]
global switch_to_32_pm
global switch_to_16_pm

switch_to_32_pm:
    cli
    lgdt [gdt_descriptor]

    mov eax, cr0
    or eax, 0x1
    mov cr0, eax

    jmp CODE_SEG:finish_32_pm

[bits 32]
finish_32_pm:
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    ret

switch_to_16_pm:
    ret

[bits 16]
gdt_start:

gdt_null:
	dd 0x00
	dd 0x00
	
gdt_code:
	dw 0xFFFF
	dw 0x0000
	db 0x00
	db 10011010b
	db 11001111b
	db 0x00
	
gdt_data:
	dw 0xFFFF
	dw 0x0000
	db 0x00
	db 10010010b
	db 11001111b
	db 0x00
	
gdt_end:

gdt_descriptor:
	dw gdt_end - gdt_start - 1
	dd gdt_start
	
CODE_SEG: equ gdt_code - gdt_start
DATA_SEG: equ gdt_data - gdt_start