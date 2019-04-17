//
// Small Bitfield
//

use crate::{find_highest_set_bit, find_lowest_set_bit, FastBitField, SMALL_BIT_FIELD_BIT_SIZE};

pub struct SmallBitField {
    bitfield: usize,
}

impl SmallBitField {
    pub fn new() -> SmallBitField {
        SmallBitField { bitfield: 0 }
    }
}

impl FastBitField for SmallBitField {
    //
    // Functions
    //

    fn get_number_of_bits() -> usize {
        SMALL_BIT_FIELD_BIT_SIZE
    }

    //
    // Methods
    //

    fn set_bit(&mut self, index: usize) {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            self.bitfield |= 1 << index;
        }
    }

    fn clear_bit(&mut self, index: usize) {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            self.bitfield &= !(1 << index);
        }
    }

    fn get_lowest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        self.get_lowest_set_bit_unchecked() as isize
    }

    fn get_highest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        self.get_highest_set_bit_unchecked() as isize
    }

    fn get_lowest_set_bit_unchecked(&self) -> usize {
        find_lowest_set_bit(self.bitfield)
    }

    fn get_highest_set_bit_unchecked(&self) -> usize {
        find_highest_set_bit(self.bitfield)
    }

    fn is_empty(&self) -> bool {
        self.bitfield == 0
    }
}
