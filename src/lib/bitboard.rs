use std::fmt::{Display, Formatter, Result};


pub struct BitBoard(u64);


impl BitBoard {
    #[inline]
    pub fn set_bit(&mut self, position: usize) {
        self.0 |= 1 << position;
    }


    #[inline]
    pub fn clear_bit(&mut self, position: usize) {
        self.0 &= !(1 << position);
    }


    #[inline]
    pub fn is_bit_set(&self, position: usize) -> bool {
        self.0 & (1 << position) != 0
    }


    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }
}


impl Default for BitBoard {
    fn default() -> Self {
        BitBoard(0)
    }
}


impl Display for BitBoard {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let mut str = String::new();

        for y in (0..6).rev() {
            str.push_str(&(y+1).to_string());

            for x in 0..7 {
                let position = (x * 8) + y;

                if self.is_bit_set(position) {
                    str.push_str(" *");
                } else {
                    str.push_str(" .");
                }
            }

            str.push_str("\n");
        }

        str.push_str(" ");
        for x in 0..7 {
            let ch = (x + b'A') as char;
            str.push_str(&format!(" {}", ch));
        }
        str.push_str("\n");

        write!(formatter, "{}", str)
    }
}
