use lazy_static::lazy_static;

#[allow(unused)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(packed)]
pub struct Gdtr {
    limit: u16,
    base: u32
}

#[allow(unused)]
#[repr(packed)]
pub struct GdtEntry {
    limitlo: u16,
    baselo: u16,
    basemi: u8,
    access: u8,
    limithi_flags: u8,
    basehi: u8
}

impl GdtEntry {
    pub fn new(base: usize, limit: usize, access: u8, flags: u8) -> GdtEntry {
        GdtEntry {
            limitlo: (limit & 0xFFFF) as u16,
            baselo: (base & 0xFFFF) as u16,
            basemi: ((base >> 16) & 0xFF) as u8,
            access: access,
            limithi_flags: (((limit >> 16) & 0xF) as u8 | ((flags & 0xF) << 4)) as u8,
            basehi: ((base >> 24) & 0xFF) as u8
        }
    }

    pub fn nullentry() -> GdtEntry {
        GdtEntry::new(0, 0, 0, 0)
    }
}

extern "C" {
    fn memcpy(dst: *mut u8, src: *const u8, size: usize);
}

pub static GDTBASE: u32 = 0x00000800;
pub static GDTR: Gdtr = Gdtr {
    limit: 8 * 7,
    base: 0x00000800
};

pub fn load() {
    unsafe {
        memcpy(GDTBASE as *mut u8, GDT_ENTRIES.as_ptr() as *const u8, 8 * 7);

        asm!("lgdtl (%eax)" : : "{eax}"(&GDTR) : "memory" :);

        asm!("
            movw $$0x10, %ax
            movw %ax, %ds
            movw %ax, %es
            movw %ax, %fs
            movw %ax, %gs
            ljmp $$0x08, $$next
            next:
        " : : : "memory" :);
    };
}

lazy_static! {
    pub static ref GDT_ENTRIES: [GdtEntry; 7] = [
        GdtEntry::nullentry(),
        GdtEntry::new(0x00000000, 0xFFFFFFFF, 0x9A, 0x0D), // code
        GdtEntry::new(0x00000000, 0xFFFFFFFF, 0x92, 0x0D), // data
        GdtEntry::new(0x00000000, 0xFFFFFFFF, 0x96, 0x0D), // stack

        GdtEntry::new(0x00000000, 0xFFFFFFFF, 0xFA, 0x0D), // code
        GdtEntry::new(0x00000000, 0xFFFFFFFF, 0xF2, 0x0D), // data
        GdtEntry::new(0x00000000, 0xFFFFFFFF, 0xF6, 0x0D), // stack
    ];
}
