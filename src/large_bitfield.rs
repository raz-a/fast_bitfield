//! # Large Bitfield
//! `large_bitfield` defines the structure and fast_bitfield interface for Large Bitfieds.
//! A Large Bitfield is a strcture that holds an array of `sizeof(usize) * 8` `usize` values as well
//! as a "layer_cache" `usize` field to quickly determine highest and lowest set bits.

use crate::{
    find_highest_set_bit, find_lowest_set_bit, FastBitField, LARGE_BIT_FIELD_BIT_SIZE,
    SMALL_BIT_FIELD_BIT_SIZE,
};

/// Defines the structure of the LargeBitField
pub struct LargeBitField {
    /// Holds a bitfield describing which sub bitfields currently have any set bits.
    layer_cache: usize,

    /// Holds the bitfield state.
    bitfield: [usize; SMALL_BIT_FIELD_BIT_SIZE],
}

/// Defines the FastBitField interface for LargeBitField.
impl LargeBitField {
    /// Creates a new, empty LargeBitField
    ///
    /// # Returns
    /// A LargeBitField.
    pub fn new() -> LargeBitField {
        LargeBitField {
            layer_cache: 0,
            bitfield: [0; SMALL_BIT_FIELD_BIT_SIZE],
        }
    }
}

/// Defines the FastBitField interface for LargeBitField.
impl FastBitField for LargeBitField {
    /// Gets the number of bits available in the bitfield type.
    ///
    /// # Returns
    /// The number of bits available.
    fn get_number_of_bits() -> usize {
        LARGE_BIT_FIELD_BIT_SIZE
    }

    /// Sets a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    fn set_bit(&mut self, index: usize) {
        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        self.layer_cache |= 1 << top_layer;

        let sub_field = self.bitfield.get_mut(top_layer);
        let sub_field = match sub_field {
            Some(s) => s,
            None => return,
        };

        *sub_field |= 1 << bottom_layer;
    }

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    fn clear_bit(&mut self, index: usize) {
        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        let sub_field = self.bitfield.get_mut(top_layer);
        let sub_field = match sub_field {
            Some(s) => s,
            None => return,
        };

        *sub_field &= !(1 << bottom_layer);
        if *sub_field == 0 {
            self.layer_cache &= !(1 << top_layer);
        }
    }

    /// Gets the lowest set bit.
    ///
    /// # Returns
    /// The lowest set bit index or -1 if no bits are set.
    fn get_lowest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        self.get_lowest_set_bit_unchecked() as isize
    }

    /// Gets the highest set bit.
    ///
    /// # Returns
    /// The highest set bit index or -1 if no bits are set.
    fn get_highest_set_bit(&self) -> isize {
        if self.is_empty() {
            return -1;
        }

        self.get_highest_set_bit_unchecked() as isize
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
        let level = find_lowest_set_bit(self.layer_cache);

        //
        // UNSAFE: level is guaranteed to be between 0 and SMALL_BIT_FIELD_SIZE - 1 by the
        // the definition of find_lowest_set_bit. No need to perform bounds checking on the array.
        //

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (level * SMALL_BIT_FIELD_BIT_SIZE) + find_lowest_set_bit(*sub_field);
        }
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
        let level = find_highest_set_bit(self.layer_cache);

        //
        // UNSAFE: level is guaranteed to be between 0 and SMALL_BIT_FIELD_SIZE - 1 by the
        // the definition of find_highest_set_bit. No need to perform bounds checking on the array.
        //

        unsafe {
            let sub_field = self.bitfield.get_unchecked(level);
            return (level * SMALL_BIT_FIELD_BIT_SIZE) + find_highest_set_bit(*sub_field);
        }
    }

    /// Determines whether or not the bitfield is empty.
    ///
    /// # Retuns
    /// true if empty, false otherwise.
    fn is_empty(&self) -> bool {
        self.layer_cache == 0
    }
}
