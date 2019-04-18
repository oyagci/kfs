global start
extern kmain
extern krust

section .text
bits 32

start:
	;mov dword [0xb8000], 0x2f4b2f4f
	call krust
	hlt
