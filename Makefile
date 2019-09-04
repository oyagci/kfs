ARCH		:= x86
KERNEL		:= kernel-$(ARCH).bin
LINKER_FILE	:= x86.ld

SRC_ASM_RAW	:= boot.s
SRC_ASM		:= $(addprefix src/,$(SRC_ASM_RAW))

SRC_C		:=

SRC_RS		:= \
	src/lib.rs \
	src/vga_buffer.rs \
	src/keyboard_driver.rs \
	src/utils.rs

SRCDIR		:= src

BUILDDIR	:= build

OBJ_ASM		:= $(addprefix $(BUILDDIR)/,$(SRC_ASM_RAW:.s=.o))
OBJ_C		:= $(addprefix $(BUILDDIR)/,$(SRC_C:.c=.o))
OBJ_RS		:= $(addprefix $(BUILDDIR)/,$(SRC_RS:.rs=.o))

CC			:= gcc
CFLAGS		:= -Wall -Wextra -ffreestanding -m32

AS			:= nasm
ASFLAGS		:= -f elf32

LD			:= ld
LDFLAGS		:= -melf_i386 -n

ISO			:= os.iso
ISODIR		:= isofiles
GRUB_CFG	:= grub.cfg

RUST_LIB	:= target/i386-unknown-linux-gnu/release/libkfs.a

.PHONY : all, iso, run, clean

all: $(BUILDDIR)/$(KERNEL)

$(BUILDDIR)/$(KERNEL): $(BUILDDIR) $(OBJ_ASM) $(OBJ_C) $(RUST_LIB) $(LINKER_FILE)
	$(LD) $(LDFLAGS) -T $(LINKER_FILE) -o $(BUILDDIR)/$(KERNEL) $(OBJ_ASM) $(OBJ_C) $(RUST_LIB)

$(BUILDDIR)/%.o: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILDDIR)/%.o: $(SRCDIR)/%.s
	$(AS) $(ASFLAGS) $< -o $@

$(RUST_LIB): $(SRC_RS)
	cargo xbuild --release --target i386-unknown-linux-gnu.json

iso: $(BUILDDIR)/$(ISO)

$(BUILDDIR)/$(ISO): $(BUILDDIR)/$(KERNEL)
	mkdir -p $(BUILDDIR)/$(ISODIR)/boot/grub
	cp $(GRUB_CFG) $(BUILDDIR)/$(ISODIR)/boot/grub/grub.cfg
	cp $(BUILDDIR)/$(KERNEL) $(BUILDDIR)/$(ISODIR)/boot/kernel.bin
	grub-mkrescue --compress=xz -o $(BUILDDIR)/$(ISO) $(BUILDDIR)/$(ISODIR)

run: iso
	qemu-system-i386 -cdrom $(BUILDDIR)/$(ISO)

clean:
	rm -Rf $(BUILDDIR)
	cargo clean

$(BUILDDIR):
	mkdir build