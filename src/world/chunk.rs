const SECTION_BITS: u32 = 16;
const SECTION_CAPACITY: u32 = 4096;

#[derive(Debug)]
pub struct BitArray {
    pub data: Vec<u32>,
    pub capacity: u32,
    pub bits_per_value: u32,
    pub value_mask: u32
}

#[derive(Debug)]
pub struct ChunkSection {
    pub data: BitArray,
    pub palette: Vec<u16>,
}

impl Into<[[[u16; 16]; 16]; 16]> for &ChunkSection {
    fn into(self) -> [[[u16; 16]; 16]; 16] {
        // single valued palette
        if self.data.capacity == 1 {
            // need to clear 0s, reverse bit order (least - most sigbit), & convert to decimal
            return [[[0; 16]; 16]; 16]
        }
        // validate chunk data
        assert!(self.data.capacity == SECTION_CAPACITY);
        assert!(self.data.bits_per_value <= SECTION_BITS);
        // assert!(/* old buffer & new buffer are iterable */);
        // initialise references to old/new buffers
        let bb = &mut self.data.data.clone();
        let db = &mut [0; SECTION_CAPACITY as usize];
        // record buffer length
        let bl = bb.len();
        let dl = db.len();
        // initialise buffer position index
        let mut b = 0;
        let mut d = 0;
        // initialise counters for current long & block state bits
        let mut l = 1;
        let mut n = 1;
        
        // extract blockstates into new buffer from compacted data format
        while b < bl || d < dl {
            // shift to get the foremost insignificant bit
            let long = bb[b];
            let part = long << 1;
            // non 0 bit, append to new buffer
            if part >> 1 < long {
                db[d] += u16::pow(2, self.data.bits_per_value - n); // insig -> sig bit
            }
            bb[b] = part;

            n += 1;
            l += 1;

            if l > 32 {
                l = 1;
                b += 1;
            }

            if n > self.data.bits_per_value {
                n = 1;
                d += 1;
            }
        }

        // indirect palette conversion
        if self.palette.len() > 0 {
            for i in 0..dl {
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

struct ChunkGrid {
    pub position: [i32; 3],
    pub blocks: [[[u16; 16]; 16]; 16]
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