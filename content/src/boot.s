section .multiboot
_header_start:
	dd 0xe85250d6
	dd 0
	dd _header_end - _header_start
	dd 0x100000000 - (0xe85250d6 + 0 + (_header_end - _header_start))

	dw 0
	dw 0
	dd 8
_header_end:

global start
extern kmain
extern krust

section .text
bits 32

start:
	;mov dword [0xb8000], 0x2f4b2f4f
	call krust
_halt:
	hlt
	jp _halt
