use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct BitMap( pub [[u8; 8]; 8] );

impl fmt::Display for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "___")?;
        for line in self.0.iter() {
            for bit in line.iter() {
                write!(f, "{}, ", bit)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "---")
    }
}

impl BitMap {
    /*pub fn new() -> BitMap {
        BitMap([[0; 8]; 8])
    }*/

    pub fn compress_map(self) -> (u64, u64, u64) {
        let mut a:u64=0;
        let mut b:u64=0;
        let mut c:u64=0;
        let mut i=0;

        for line in self.0.iter() {
            for color in line.iter() {
                a |= ((*color as u64&4)>>2)<<i;
                b |= ((*color as u64&2)>>1)<<i;
                c |= (*color as u64&1)<<i;
                i+=1;
            }
        }

        (a, b, c)
    }

    pub fn from_compress(compressed: (u64, u64, u64)) -> BitMap {
        let (a, b, c) = compressed;
        let mut map: [[u8; 8]; 8] = [[0; 8]; 8];
        let mut i=0;

        for line in 0..8 {
            for bit in 0..8 {
                let temp:u8 = ((((a&(1<<i))>>i)<<2) | (((b&(1<<i))>>i)<<1) | (c&(1<<i))>>i) as u8;
                map[line][bit] = temp;
                i+=1;
            }
        }

        BitMap(map)
    }

    pub fn invert_side(&mut self) {
        for i in 0..8 {
            for j in 0..4 {
                let temp = self.0[i][j];
                self.0[i][j] = self.0[i][(7-j) as usize];
                self.0[i][(7-j) as usize] = temp;
            }
        }
    }

    /*pub fn right_rotate(&mut self) {
        let mut temp = BitMap::new();
        for i in 0..8 {
            for j in 0..8 {
                temp.0[i][j] = self.0[7-j][i];
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                self.0[i][j] = temp.0[i][j];
            }
        }
    }*/
}
