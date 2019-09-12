const PAGESIZE: usize = 4096;
const RAM_MAXPAGE: usize = 0x100000;

pub struct FrameAllocator {
    mem_bitmap: [u8; RAM_MAXPAGE / 8],
}

impl FrameAllocator {
    pub fn new() -> FrameAllocator {
        FrameAllocator {
            mem_bitmap: [0; RAM_MAXPAGE / 8],
        }
    }

    fn set_page_frame_used(&mut self, page: usize) {
        self.mem_bitmap[(page / 8)] |= 1 << (page % 8) as u32;
    }

    pub fn free(&mut self, frame: *mut u8) {
        let mut page: usize;

        page = (frame as usize / PAGESIZE) as usize;
        self.mem_bitmap[(page / 8)] &= !(1 << (page % 8) as u32);
    }

    pub fn alloc(&mut self) -> Option <*mut u8> {
        let mut page: usize;

        for byte in 0..(RAM_MAXPAGE / 8) {
            if self.mem_bitmap[byte] != 0xFF {
                for bit in 0..8 {
                    if (self.mem_bitmap[byte] & (1 << bit)) == 0 {
                        page = (8 * byte + bit) as usize;
                        self.set_page_frame_used(page);
                        return Some ((page * PAGESIZE) as *mut u8);
                    }
                }
            }
        }
        None
    }
}
