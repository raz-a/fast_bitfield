//
// Fast Bit Field
// Defines a bitfield type with architectural sizes and fast lowest/highest bit determination
//

#![no_std]

use core::mem;

pub trait FastBitField {

    //
    // Functions
    //

    fn get_number_of_bits() -> usize;

    //
    // Methods
    //

    fn set_bit(&mut self, index: usize);
    fn clear_bit(&mut self, index: usize);
    fn get_lowest_set_bit(&self) -> isize;
    fn get_highest_set_bit(&self) -> isize;
    fn is_empty(&self) -> bool;
}

//
// Small Bitfield
//

const SMALL_BIT_FIELD_BIT_SIZE: usize = mem::size_of::<usize>() * 8;

pub struct SmallBitField {
    bitfield: usize
}


impl SmallBitField {
    pub fn new() -> SmallBitField {
        SmallBitField{bitfield: 0}
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
            return;
        }

        self.bitfield |= 1 << index;
    }

    fn clear_bit(&mut self, index: usize) {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            return;
        }

        self.bitfield &= !(1 << index);
    }

    fn get_lowest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        self.bitfield.trailing_zeros() as isize
    }

    fn get_highest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        (SMALL_BIT_FIELD_BIT_SIZE -  1 - self.bitfield.leading_zeros() as usize) as isize
    }

    fn is_empty(&self) -> bool {
        self.bitfield == 0
    }
}

impl Copy for SmallBitField { }

impl Clone for SmallBitField {
    fn clone(&self) -> SmallBitField {
        *self
    }
}

//
// Large Bitfield
//

const LARGE_BIT_FIELD_BIT_SIZE: usize = SMALL_BIT_FIELD_BIT_SIZE * SMALL_BIT_FIELD_BIT_SIZE;

pub struct LargeBitField {
    layer_cache: usize,
    bitfield: [usize; SMALL_BIT_FIELD_BIT_SIZE]
}

impl LargeBitField {
    pub fn new() -> LargeBitField {
        LargeBitField{layer_cache: 0, bitfield: [0; SMALL_BIT_FIELD_BIT_SIZE]}
    }
}

impl FastBitField for LargeBitField {

    //
    // Functions
    //

    fn get_number_of_bits() -> usize {
        LARGE_BIT_FIELD_BIT_SIZE
    }

    //
    // Methods
    //

    fn set_bit(&mut self, index: usize) {
        if index < LARGE_BIT_FIELD_BIT_SIZE {
            return;
        }

        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        self.layer_cache |= 1 << top_layer;
        self.bitfield[top_layer] |= 1 << bottom_layer;
    }

    fn clear_bit(&mut self, index: usize) {
        if index < LARGE_BIT_FIELD_BIT_SIZE {
            return;
        }

        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        self.bitfield[top_layer] &= !(1 << bottom_layer);

        if self.bitfield[top_layer] == 0 {
            self.layer_cache &= ! (1 << top_layer);
        }
    }

    fn get_lowest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        let level = self.layer_cache.trailing_zeros() as usize;

        return (
            (level * SMALL_BIT_FIELD_BIT_SIZE) +
            self.bitfield[level].trailing_zeros() as usize
        ) as isize;
    }

    fn get_highest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        let level =
            (SMALL_BIT_FIELD_BIT_SIZE - 1 - self.layer_cache.leading_zeros() as usize) as usize;

        return (
            (level * SMALL_BIT_FIELD_BIT_SIZE) +
            SMALL_BIT_FIELD_BIT_SIZE - 1 - self.bitfield[level].trailing_zeros() as usize
        ) as isize
    }

    fn is_empty(&self) -> bool {
        self.layer_cache == 0
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
