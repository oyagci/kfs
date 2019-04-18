#include <stddef.h>
#include <stdint.h>

#define VGA_WIDTH	80
#define VGA_HEIGHT	25

struct {
	void		*buffer;
	unsigned short	row;
	unsigned short	col;
} g_vga_buffer_status = {
	.buffer = (void *)0xb8000, 0, 0
};

enum e_vga_color {
	VGA_COLOR_BLACK = 0,
	VGA_COLOR_BLUE = 1,
	VGA_COLOR_GREEN = 2,
	VGA_COLOR_CYAN = 3,
	VGA_COLOR_RED = 4,
	VGA_COLOR_MAGENTA = 5,
	VGA_COLOR_BROWN = 6,
	VGA_COLOR_LIGHT_GREY = 7,
	VGA_COLOR_DARK_GREY = 8,
	VGA_COLOR_LIGHT_BLUE = 9,
	VGA_COLOR_LIGHT_GREEN = 10,
	VGA_COLOR_LIGHT_CYAN = 11,
	VGA_COLOR_LIGHT_RED = 12,
	VGA_COLOR_LIGHT_MAGENTA = 13,
	VGA_COLOR_LIGHT_BROWN = 14,
	VGA_COLOR_WHITE = 15,
};

uint8_t		vga_color(enum e_vga_color fg, enum e_vga_color bg) {
	return fg | bg << 4;
}

uint16_t	vga_entry(unsigned char c, enum e_vga_color fg, enum e_vga_color bg) {
	return (uint16_t)c | (uint16_t)vga_color(fg, bg) << 8;
}

void	vga_putc(char c, enum e_vga_color fg, enum e_vga_color bg) {

	if (g_vga_buffer_status.col >= VGA_WIDTH || c == '\n') {
		g_vga_buffer_status.col = 0;
		g_vga_buffer_status.row++;
	}

	if (c == '\n')
		return ;

	size_t index = VGA_WIDTH * g_vga_buffer_status.row + g_vga_buffer_status.col;

	((uint16_t *)g_vga_buffer_status.buffer)[index] = vga_entry(c, fg, bg);
	g_vga_buffer_status.col++;
}

size_t	kstrlen(char const *const s) {
	size_t i = 0;

	for (; s[i] != 0; i++)
		;
	return i;
}

void kputc(char c) {
	vga_putc(c, VGA_COLOR_WHITE, VGA_COLOR_BLACK);
}

void kwrite(char const *const s) {
	size_t len = kstrlen(s);

	for (size_t i = 0; i < len; i++)
		kputc(s[i]);
}

void kputs(char const *const s) {
	if (s) {
		kwrite(s);
		kputc('\n');
	}
	else {
		kwrite("(null)\n");
	}
}

void krust();

int kmain() {
	/*kputs("KFS-1 by Oguzhan YAGCI <oyagci@student.42.fr>");*/
	/*kputs("Calling RUST");*/
	krust();
	return (0);
}
