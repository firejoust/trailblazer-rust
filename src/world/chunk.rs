type BlockState = u16;
const SECTION_BITS: usize = 16;
const SECTION_CAPACITY: usize = 4096;

#[derive(Debug)]
pub struct BitArray {
    pub data: Vec<u32>,
    pub capacity: u32,
    pub bits_per_value: u32,
    pub spanned: bool
}

impl Into<[BlockState; SECTION_CAPACITY]> for &BitArray {
    fn into(self) -> [BlockState; SECTION_CAPACITY] {
        if self.capacity == 1 {
            // will panic if data is too large;
            return [self.data[0] as BlockState; SECTION_CAPACITY]
        }

        assert!(self.capacity as usize == SECTION_CAPACITY);
        assert!(self.bits_per_value as usize <= SECTION_BITS);

        // record buffer length
        let bl = self.data.len();
        let dl = SECTION_CAPACITY;

        // initialise half-long bit capacity 64 = 32 + (32 - mod)
        let mut lh = false;
        let mut ln = 32; // current half bit capacity
        let ls = 32 - (64 % self.bits_per_value); //  second-half bit capacity

        // initialise references to old/new buffers
        let mut bb = self.data.clone();
        let mut db = [0; SECTION_CAPACITY];

        // initialise buffer position index
        let mut b = 0;
        let mut d = 0;

        // initialise counters for current long & block state bits
        let mut l = 0;
        let mut n = 0;

        // extract blockstates into new buffer from compacted data format
        while d < dl && b < bl {
            // shift to get the foremost insignificant bit
            let long = bb[b];
            let part = long >> 1;

            // non 0 bit, append to new buffer
            if part << 1 < long {
                db[d] += BlockState::pow(2, n);
            }
            bb[b] = part;

            l += 1;
            n += 1;

            if l >= ln {
                l = 0;
                b += 1;
                // determine length of the next half-long (non-spanned)
                ln = if self.spanned { ln } else {
                    lh = !lh;
                    if lh { ls } else { 32 }
                };
            }

            if n >= self.bits_per_value {
                n = 0;
                d += 1;
            }
        }
        db
    }
}

#[derive(Debug)]
pub struct ChunkSection {
    pub data: BitArray,
    pub palette: Vec<BlockState>,
}

impl Into<[[[BlockState; 16]; 16]; 16]> for &ChunkSection {
    fn into(self) -> [[[BlockState; 16]; 16]; 16] {
        let mut db: [BlockState; SECTION_CAPACITY] = (&self.data).into();
        // indirect palette conversion
        if self.palette.len() > 0 {
            for i in 0..SECTION_CAPACITY {
                db[i] = self.palette[db[i] as usize];
            }
        }

        // order blockstates by axis
        let mut cb = [[[0; 16]; 16]; 16];
        let mut c = 0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    cb[x][y][z] = db[c];
                    c += 1;
                }
            }
        }
        cb
    }
}

#[derive(Debug)]
pub struct ChunkColumn {
    pub x: i32,
    pub z: i32,
    pub sections: Vec<ChunkSection>
}

#[derive(Debug)]
pub struct ChunkGrid {
    pub position: [i32; 3],
    pub blocks: [[[BlockState; 16]; 16]; 16]
}

impl Into<Vec<ChunkGrid>> for &ChunkColumn {
    fn into(self) -> Vec<ChunkGrid> {
        let mut arr = vec![];
        for y in 0..self.sections.len() {
            let section = &self.sections[y];
            arr.push(
                ChunkGrid {
                    position: [self.x, y as i32, self.z],
                    blocks: section.into()
                }
            );
        }
        arr
    }
}