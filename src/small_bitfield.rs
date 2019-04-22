//! # Small Bitfield
//! `small_bitfield` defines the structure and fast_bitfield interface for Small Bitfieds.
//! A Small Bitfield is a wrapper type that holds a `usize` bitfield.

use crate::{find_highest_set_bit, find_lowest_set_bit, FastBitField, SMALL_BIT_FIELD_BIT_SIZE};

/// Defines the structure of the SmallBitField
pub struct SmallBitField {
    /// Holds the bitfield state.
    bitfield: usize,
}

/// Defines functionality unique to SmallBitField.
impl SmallBitField {
    /// Creates a new, empty SmallBitField
    ///
    /// # Returns
    /// A SmallBitField.
    pub fn new() -> SmallBitField {
        SmallBitField { bitfield: 0 }
    }

    /// Sets bits in the bit field.
    ///
    /// # Arguments
    /// field - Provides the bits to be set.
    pub fn set_field(&mut self, field: usize) {
        self.bitfield |= field;
    }

    /// Clears bits in the bit field.
    ///
    /// # Arguments
    /// field - Provides the bits to be cleared.
    pub fn clear_field(&mut self, field: usize) {
        self.bitfield &= !field;
    }
}

/// Defines the FastBitField interface for SmallBitField.
impl FastBitField for SmallBitField {
    /// Gets the number of bits available in the bitfield type.
    ///
    /// # Returns
    /// The number of bits available.
    fn get_number_of_bits() -> usize {
        SMALL_BIT_FIELD_BIT_SIZE
    }

    /// Sets a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    fn set_bit(&mut self, index: usize) {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            self.bitfield |= 1 << index;
        }
    }

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    fn clear_bit(&mut self, index: usize) {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            self.bitfield &= !(1 << index);
        }
    }

    /// Gets the lowest set bit.
    ///
    /// # Returns
    /// The lowest set bit index or None if no bits are set.
    fn get_lowest_set_bit(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        Some(self.get_lowest_set_bit_unchecked())
    }

    /// Gets the highest set bit.
    ///
    /// # Returns
    /// The highest set bit index or None if no bits are set.
    fn get_highest_set_bit(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        Some(self.get_highest_set_bit_unchecked())
    }

    /// Gets the value of a specific bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to test.
    ///
    /// # Returns
    /// Some(true) if bit is set.
    /// Some(false) if bit is cleared.
    /// None if index is invalid.
    fn test_bit(&self, index: usize) -> Option<bool> {
        if index < SMALL_BIT_FIELD_BIT_SIZE {
            //
            // UNSAFE: The index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                return Some(self.test_bit_unchecked(index));
            }
        }

        None
    }

    /// Determines whether or not the bitfield is empty.
    ///
    /// # Retuns
    /// true if empty, false otherwise.
    fn is_empty(&self) -> bool {
        self.bitfield == 0
    }

    /// Gets the lowest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The lowest set bit index or UNDEFINED if no bits are set.
    fn get_lowest_set_bit_unchecked(&self) -> usize {
        find_lowest_set_bit(self.bitfield)
    }

    /// Gets the highest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The highest set bit index or UNDEFINED if no bits are set.
    fn get_highest_set_bit_unchecked(&self) -> usize {
        find_highest_set_bit(self.bitfield)
    }

    /// Sets a bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field.
    unsafe fn set_bit_unchecked(&mut self, index: usize) {
        self.bitfield |= 1 << index;
    }

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field.
    unsafe fn clear_bit_unchecked(&mut self, index: usize) {
        self.bitfield &= !(1 << index);
    }

    /// Gets the value of a specific bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to test.
    ///
    /// # Returns
    /// true if bit is set.
    /// false if bit is cleared.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field.
    unsafe fn test_bit_unchecked(&self, index: usize) -> bool {
        (self.bitfield & (1 << index)) != 0
    }
}

// RAZTODO: Doc Tests
// RAZTODO: Unit Tests
