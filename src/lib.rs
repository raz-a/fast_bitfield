//
// Fast Bit Field
// Defines a bitfield type with architectural sizes and fast lowest/highest bit determination
//

#![no_std]

use core::mem;

pub trait FastBitField {
    fn set_bit(&mut self, index: usize);
    fn clear_bit(&mut self, index: usize);
    fn get_lowest_set_bit(&self) -> isize;
    fn get_highest_set_bit(&self) -> isize;
}

pub struct SmallBitField {
    bitfield: usize
}

impl SmallBitField {
    fn new() -> SmallBitField {
        SmallBitField{bitfield: 0}
    }
}

impl FastBitField for SmallBitField {
    fn set_bit(&mut self, index: usize) {
        self.bitfield |= 1 << index;
    }

    fn clear_bit(&mut self, index: usize) {
        self.bitfield &= !(1 << index);
    }

    fn get_lowest_set_bit(&self) -> isize {
        if self.bitfield == 0 {
            return -1;
        }

        self.bitfield.trailing_zeros() as isize
    }

    fn get_highest_set_bit(&self) -> isize {
        if self.bitfield == 0 {
            return -1;
        }

        let bits_of_size = mem::size_of::<usize>() * 8;
        (bits_of_size -  1 - self.bitfield.leading_zeros() as usize) as isize
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
