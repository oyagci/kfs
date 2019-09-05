cargo xbuild --release --target i386-unknown-none.json && \
	mkdir -p build/isodir/boot/grub && \
	cp grub.cfg build/isodir/boot/grub/grub.cfg && \
	cp target/i386-unknown-none/release/kfs build/isodir/boot/kernel.bin && \
	grub-mkrescue -o kernel.iso build/isodir && \
	qemu-system-i386 -cdrom kernel.iso
