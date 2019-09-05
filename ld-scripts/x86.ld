ENTRY(start)
OUTPUT_FORMAT(elf32-i386)

PHDRS {
	boot PT_LOAD;
	text PT_LOAD;
	data PT_LOAD;
	rodata PT_LOAD;
}

SECTIONS {
	. = 0M;

	.multiboot ALIGN(1K) : {
		KEEP(*(.multiboot))
	} : boot

	. = 1M;

	.text ALIGN(4K) : {
		*(.text)
		*(.text.*)
	} : text

	.rodata ALIGN(4K) : {
		*(.rodata)
		*(.rodata.*)
	} : rodata

	.data ALIGN(4K) : {
		*(.data)
		*(.data.*)
	} : data

	.got ALIGN(4K) : {
		*(.got)
		*(.got.plt)
	} : data

	.bss ALIGN(4K) : {
		*(COMMON)
		*(.bss)
		*(.bss.*)
	} : data
}
