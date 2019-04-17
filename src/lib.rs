// Fast Bit Field
// Defines a bitfield type with architectural sizes and fast lowest/highest bit determination

#![cfg_attr(not(test), no_std)]

use core;
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
    fn get_lowest_set_bit_unchecked(&self) -> usize;
    fn get_highest_set_bit_unchecked(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub mod large_bit_field;
pub mod small_bit_field;

const SMALL_BIT_FIELD_BIT_SIZE: usize = core::mem::size_of::<usize>() * 8;
const LARGE_BIT_FIELD_BIT_SIZE: usize = SMALL_BIT_FIELD_BIT_SIZE * SMALL_BIT_FIELD_BIT_SIZE;

fn find_lowest_set_bit(value: usize) -> usize {
    if cpu_features::opcodes::count_leading_zeros_exists() {
        value.trailing_zeros() as usize
    } else {
        debruijin::get_lowest_set_bit(value)
    }
}

fn find_highest_set_bit(value: usize) -> usize {
    if cpu_features::opcodes::count_leading_zeros_exists() {
        SMALL_BIT_FIELD_BIT_SIZE - 1 - value.leading_zeros() as usize
    } else {
        debruijin::get_highest_set_bit(value)
    }
}
