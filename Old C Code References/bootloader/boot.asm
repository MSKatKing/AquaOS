[BITS 16] ; 16-bit real mode
[ORG 0x7c00] ; BIOS loads bootloader into this memory address

_start:
    mov ah, 0x00
    mov al, 0x03
    int 0x10

    mov     [BOOT_DRIVE],       dl              ; Boot drive is stored in dl on startup

    pusha   

    mov     ah,                 0x02            ; Read sectors from drive
    mov     al,                 0x01   ; Read however many sectors the kernel needs
    mov     dl,                 [BOOT_DRIVE]    ; Select the current drive (boot drive)
    mov     cx,                 0x02
    mov     dh,                 0               ; Head number

    mov     bx,                 0x0000
    mov     es,                 bx
    mov     bx,                 0x1000          ; Load data into memory address

    int     0x13                            ; Disk drive call

    jc      error_msg                      ; Jump to the error message if an error occurred

    mov al, byte [es:bx]
    mov [KERNEL_SIZE], al

    popa

    pusha   

    mov     ah,                 0x02            ; Read sectors from drive
    mov     al,                 [KERNEL_SIZE]   ; Read however many sectors the kernel needs
    mov     dl,                 [BOOT_DRIVE]    ; Select the current drive (boot drive)
    mov     cx,                 0x03
    mov     dh,                 0               ; Head number

    mov     bx,                 0x0000
    mov     es,                 bx
    mov     bx,                 0x1000          ; Load data into memory address

    int     0x13                            ; Disk drive call

    jc      error_msg                     ; Jump to the error message if an error occurred

    popa                                    ; Kernel should now be stored at 0x0000:0x1000

    cli
    lgdt    [gdt_descriptor]

    mov     eax,                cr0
    or      eax,                0x1
    mov     cr0,                eax

    jmp     CODE_SEG:begin_32

[bits 32]
begin_32:
    mov     ax,                 DATA_SEG
    mov     ds,                 ax
    mov     es,                 ax
    mov     fs,                 ax
    mov     gs,                 ax
    mov     ss,                 ax

    mov al, [BOOT_DRIVE]
    jmp     0x1000

[bits 16]
error_msg:
    mov     bl,                 0
    mov     si,                 DISK_ERROR
    call    print_string

    jmp $

; ES:SI contains the string, BL is the x coordinate, BH is the y coordinate
print_string:
    pusha

    mov bx, 0

.print_string_loop:
    mov     al,                 [si]
    cmp     al,                 0
    je      .print_string_done

    mov     ah,                 0x07
    call    print_char

    inc     bx
    inc     si
    jmp     .print_string_loop

.print_string_done:
    popa
    ret

; AL must contain the letter to print, BL is the x coordinate, BH is the y coordinate
print_char:
    pusha

    push    ax
    
    mov     ah,                 0           ; Clear AH
    mov     al,                 bh          ; AL = Y coordinate
    mov     cx,                 80          ; CX = 80 (number of columns)
    mul     cx                              ; AX = Y * 80

    add     al,                 bl          ; AX = Y * 80 + X
    xchg    ax,                 bx          ; BX = Y * 80 + X
    shl     bx,                 1           ; BX = (Y * 80 + X) * 2

    pop     ax

    mov     cx,                 0xB800
    mov     es,                 cx          ; video memory starts at 0bB8000

    mov     [es:bx],            al
    mov     byte [es:bx+1],     0x07
    
    mov     bh,                 0
    mov     ah,                 0x03
    int     0x10
    
    inc     dx
    mov     ah,                 0x02
    int     0x10

    popa
    ret

DISK_ERROR:     db 'A disk error has occurred. The PC will reboot. If this error continually occurs, try a different disk.', 0

BOOT_DRIVE:     db 0
KERNEL_SIZE:    db 0
KERNEL_LOCATION: dw 0

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

times 510-($-$$) db 0 ; Fill the rest of the sector with 0s.

;dw 0x2080
;dw 0x21
;dw 0xfec0
;dw 0xffff
;dw 0x0800
;dw 0x00
;dw 0x1000
;dw 0x01dd

;dd 0x00, 0x00, 0x00, 0x00 ; Partition 2
;dd 0x00, 0x00, 0x00, 0x00 ; Partition 3
;dd 0x00, 0x00, 0x00, 0x00 ; Partition 4
dw 0xAA55 ; Magic BIOS number specifing this disk as bootable.