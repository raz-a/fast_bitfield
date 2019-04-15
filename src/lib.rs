// Fast Bit Field
// Defines a bitfield type with architectural sizes and fast lowest/highest bit determination

#![cfg_attr(not(test), no_std)]

use core::mem;
use cpu_features;
use debruijin;

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

        find_lowest_set_bit(self.bitfield) as isize
    }

    fn get_highest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        find_highest_set_bit(self.bitfield) as isize
    }

    fn is_empty(&self) -> bool {
        self.bitfield == 0
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

        let level = find_lowest_set_bit(self.layer_cache);

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (
                (level * SMALL_BIT_FIELD_BIT_SIZE) +
                find_lowest_set_bit(*sub_field)
            ) as isize;
        }
    }

    fn get_highest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        let level = find_highest_set_bit(self.layer_cache);

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (
                (level * SMALL_BIT_FIELD_BIT_SIZE) +
                find_highest_set_bit(*sub_field)
            ) as isize;
        }
    }

    fn is_empty(&self) -> bool {
        self.layer_cache == 0
    }
}

fn find_lowest_set_bit(value: usize) -> usize {
    if cpu_features::opcodes::count_leading_zeros_exists() {
        value.trailing_zeros() as usize

    } else {
        debruijin::get_lowest_set_bit(value)
    }

}

fn find_highest_set_bit(value: usize) -> usize {
    if cpu_features::opcodes::count_leading_zeros_exists() {
        SMALL_BIT_FIELD_BIT_SIZE -  1 - value.leading_zeros() as usize

    } else {
        debruijin::get_highest_set_bit(value)
    }
}

