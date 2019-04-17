//
// Large Bitfield
//

use crate::{
    find_highest_set_bit, find_lowest_set_bit, FastBitField, LARGE_BIT_FIELD_BIT_SIZE,
    SMALL_BIT_FIELD_BIT_SIZE,
};

pub struct LargeBitField {
    layer_cache: usize,
    bitfield: [usize; SMALL_BIT_FIELD_BIT_SIZE],
}

impl LargeBitField {
    pub fn new() -> LargeBitField {
        LargeBitField {
            layer_cache: 0,
            bitfield: [0; SMALL_BIT_FIELD_BIT_SIZE],
        }
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
        if index >= LARGE_BIT_FIELD_BIT_SIZE {
            return;
        }

        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        self.layer_cache |= 1 << top_layer;

        unsafe {
            let sub_field = self.bitfield.get_unchecked_mut(top_layer);
            *sub_field |= 1 << bottom_layer;
        }
    }

    fn clear_bit(&mut self, index: usize) {
        if index >= LARGE_BIT_FIELD_BIT_SIZE {
            return;
        }

        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        unsafe {
            let sub_field = self.bitfield.get_unchecked_mut(top_layer);
            *sub_field &= !(1 << bottom_layer);

            if *sub_field == 0 {
                self.layer_cache &= !(1 << top_layer);
            }
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
        let level = find_lowest_set_bit(self.layer_cache);

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (level * SMALL_BIT_FIELD_BIT_SIZE) + find_lowest_set_bit(*sub_field);
        }
    }

    fn get_highest_set_bit_unchecked(&self) -> usize {
        let level = find_highest_set_bit(self.layer_cache);

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (level * SMALL_BIT_FIELD_BIT_SIZE) + find_highest_set_bit(*sub_field);
        }
    }

    fn is_empty(&self) -> bool {
        self.layer_cache == 0
    }
}
