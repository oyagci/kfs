ARCH		:= x86
KERNEL		:= kernel-$(ARCH).bin
LINKER_FILE	:= x86.ld

SRC_ASM		:= \
	header.s \
	boot.s

SRC_C		:= \
	kmain.c

SRC_RS		:= \
	src/lib.rs

BUILDDIR	:= build

OBJ_ASM		:= $(addprefix $(BUILDDIR)/,$(SRC_ASM:.s=.o))
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

RUST_LIB	:= target/i686-unknown-linux-gnu/debug/libkfs.a

all: $(BUILDDIR)/$(KERNEL)

$(BUILDDIR)/$(KERNEL): $(BUILDDIR) $(OBJ_ASM) $(OBJ_C) rustlib $(LINKER_FILE)
	$(LD) $(LDFLAGS) -T $(LINKER_FILE) -o $(BUILDDIR)/$(KERNEL) $(OBJ_ASM) $(OBJ_C) $(RUST_LIB)

$(BUILDDIR)/%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILDDIR)/%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

rustlib: $(RUST_LIB)

$(RUST_LIB): $(SRC_RS)
		cargo xbuild --target i686-unknown-linux-gnu

%.o: %.rs
	$(RUSTC)

iso: $(BUILDDIR)/$(ISO)

$(BUILDDIR)/$(ISO): $(BUILDDIR)/$(KERNEL)
	mkdir -p $(BUILDDIR)/$(ISODIR)/boot/grub
	cp $(GRUB_CFG) $(BUILDDIR)/$(ISODIR)/boot/grub/grub.cfg
	cp $(BUILDDIR)/$(KERNEL) $(BUILDDIR)/$(ISODIR)/boot/kernel.bin
	grub-mkrescue -o $(BUILDDIR)/$(ISO) $(BUILDDIR)/$(ISODIR)

run: iso
	qemu-system-i386 -cdrom $(BUILDDIR)/$(ISO)

grubcfg: $(GRUB_CFG)
	cp $(GRUB_CFG) $(ISODIR)/boot/grub/grub.cfg

clean:
	rm -Rf $(BUILDDIR)
	cargo clean

builddir: $(BUILDDIR)

$(BUILDDIR):
	mkdir build
