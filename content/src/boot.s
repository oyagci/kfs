;section .multiboot
;_header_start:
;	dd 0xe85250d6
;	dd 0
;	dd _header_end - _header_start
;	dd 0x100000000 - (0xe85250d6 + 0 + (_header_end - _header_start))
;
;	dw 0
;	dw 0
;	dd 8
;_header_end:
section .multiboot
   align 4
   dd 0x1BADB002
   dd 0x0
   dd -0x1BADB002

global start
extern kmain

section .text
bits 32

start:
	mov esp, _stack_top
	call kmain
_halt:
	hlt
	jp _halt

global disable_cursor
disable_cursor:
	mov dx, 0x3D4
	mov al, 0xA
	out dx, al
	inc dx
	mov al, 0x20
	out dx, al
	ret

section .bss
_stack_bottom:
	resb 0x20000
_stack_top:
