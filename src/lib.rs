//! # Fast Bitfield
//! `fast_bitfield` defines the interface as well as structures for fast bitfields.
//! Fast bitfields are bitfields that can evaluate the lowest and highest set bits quickly and in a
//! constant time invariant (or nearly invariaant) of the contents of the bitfield.

#![cfg_attr(not(test), no_std)]

use core;
use cpu_features;
use debruijin;

/// Defines the required functionality for fast bitfields
pub trait FastBitField {

    /// Gets the number of bits available in the bitfield type.
    ///
    /// # Returns
    /// The number of bits available.
    fn get_number_of_bits() -> usize;

    /// Sets a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    fn set_bit(&mut self, index: usize);

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    fn clear_bit(&mut self, index: usize);

    /// Gets the lowest set bit.
    ///
    /// # Returns
    /// The lowest set bit index or -1 if no bits are set.
    fn get_lowest_set_bit(&self) -> isize;

    /// Gets the highest set bit.
    ///
    /// # Returns
    /// The highest set bit index or -1 if no bits are set.
    fn get_highest_set_bit(&self) -> isize;

    /// Gets the lowest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The lowest set bit index or UNDEFINED if no bits are set.
    fn get_lowest_set_bit_unchecked(&self) -> usize;

    /// Gets the highest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The highest set bit index or UNDEFINED if no bits are set.
    fn get_highest_set_bit_unchecked(&self) -> usize;

    /// Determines whether or not the bitfield is empty.
    ///
    /// # Retuns
    /// true if empty, false otherwise.
    fn is_empty(&self) -> bool;
}

/// Defines a fast bitfield that contains `sizeof(usize) * 8` bits.
pub mod small_bit_field;

/// Defines a fast bitfield that contains `sizeof(usize) * sizeof(usize) * 8` bits.
pub mod large_bit_field;


const SMALL_BIT_FIELD_BIT_SIZE: usize = core::mem::size_of::<usize>() * 8;
const LARGE_BIT_FIELD_BIT_SIZE: usize = SMALL_BIT_FIELD_BIT_SIZE * SMALL_BIT_FIELD_BIT_SIZE;

/// Gets the lowest set bit of a usize value.
///
/// # Arguments
/// value - The value to find the lowest set bit for.
///
/// # Returns
/// The lowest set bit index or UNDEFINED if no bits are set.
fn find_lowest_set_bit(value: usize) -> usize {
    if cpu_features::opcodes::count_leading_zeros_exists() {
        value.trailing_zeros() as usize
    } else {
        debruijin::get_lowest_set_bit(value)
    }
}

/// Gets the highest set bit of a usize value.
///
/// # Arguments
/// value - The value to find the highest set bit for.
///
/// # Returns
/// The highest set bit index or UNDEFINED if no bits are set.
fn find_highest_set_bit(value: usize) -> usize {
    if cpu_features::opcodes::count_leading_zeros_exists() {
        SMALL_BIT_FIELD_BIT_SIZE - 1 - value.leading_zeros() as usize
    } else {
        debruijin::get_highest_set_bit(value)
    }
}
