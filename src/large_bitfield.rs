use crate::{
    find_highest_set_bit, find_lowest_set_bit, FastBitField, LARGE_BIT_FIELD_BIT_SIZE,
    SMALL_BIT_FIELD_BIT_SIZE,
};

/// Defines the structure and fast_bitfield interface for Large Bitfieds.
/// A Large Bitfield is a strcture that holds an array of `sizeof(usize) * 8` `usize` values as well
/// as a "layer_cache" `usize` field to quickly determine highest and lowest set bits.
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

    /// Gets whether or not a specific group in the bit field has any bits set.
    ///
    /// # Arguments
    /// group_index - Provides the group to test.
    ///
    /// # Returns
    /// `Some(true)` if the group has any bits set.
    /// `Some(false)` if the group as no bits set.
    /// `None` if group_index is invalid.
    pub fn test_group(&self, group_index: usize) -> Option<bool> {
        if group_index < SMALL_BIT_FIELD_BIT_SIZE {
            //
            // UNSAFE: The index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                return Some(self.test_group_unchecked(group_index));
            }
        }

        None
    }

    /// Sets bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to set.
    /// group_field - Provides the bits to set within the group.
    ///
    /// # Note
    /// If the group_index provided is larger than the number of groups in the bit field. The field
    /// will remain unchanged.
    pub fn set_group(&mut self, group_index: usize, group_field: usize) {
        if group_index < SMALL_BIT_FIELD_BIT_SIZE {
            //
            // UNSAFE: The group_index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                self.set_group_unchecked(group_index, group_field);
            }
        }
    }

    /// Clears bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to clear.
    /// group_field - Provides the bits to clear within the group.
    ///
    /// # Note
    /// If the group_index provided is larger than the number of groups in the bit field. The field
    /// will remain unchanged.
    pub fn clear_group(&mut self, group_index: usize, group_field: usize) {
        if group_index < SMALL_BIT_FIELD_BIT_SIZE {
            //
            // UNSAFE: The group_index check that makes the unsafe variant unsafe is performed before
            // calling it.
            //

            unsafe {
                self.clear_group_unchecked(group_index, group_field);
            }
        }
    }

    /// Sets bits in the bitfield
    ///
    /// # Arguments
    /// values - Provides the bits to be set in the bitfield.
    pub fn set_field(&mut self, values: &[usize; SMALL_BIT_FIELD_BIT_SIZE]) {
        for index in 0..SMALL_BIT_FIELD_BIT_SIZE {
            //
            // UNSAFE: index is guaranteed to be less than the number of groups in the bitfield.
            //

            unsafe {
                self.set_group_unchecked(index, values[index]);
            }
        }
    }

    /// Clears bits in the bitfield
    ///
    /// # Arguments
    /// values - Provides the bits to be cleared in the bitfield.
    pub fn clear_field(&mut self, values: &[usize; SMALL_BIT_FIELD_BIT_SIZE]) {
        for index in 0..SMALL_BIT_FIELD_BIT_SIZE {
            //
            // UNSAFE: index is guaranteed to be less than the number of groups in the bitfield.
            //

            unsafe {
                self.clear_group_unchecked(index, values[index]);
            }
        }
    }

    /// Gets whether or not a specific group in the bit field has any bits set.
    ///
    /// # Arguments
    /// group_index - Provides the group to test.
    ///
    /// # Returns
    /// `true` if the group has any bits set.
    /// `false` if the group as no bits set.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the group_index is valid for the size of
    /// the bit field. The caller must guarantee that group_index is within the number of
    /// groups in the bit field.
    pub unsafe fn test_group_unchecked(&self, group_index: usize) -> bool {
        (self.layer_cache & (1 << group_index)) != 0
    }

    /// Sets bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to set.
    /// group_field - Provides the bits to set within the group.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the group_index is valid for the size of
    /// the bit field. The caller must guarantee that group_index is within the number of
    /// groups in the bit field.
    pub unsafe fn set_group_unchecked(&mut self, group_index: usize, group_field: usize) {
        let field_has_values = (group_field != 0) as usize;
        let layer_cache_update = (1 << group_index) * field_has_values;

        let subfield = self.bitfield.get_unchecked_mut(group_index);
        *subfield |= group_field;

        self.layer_cache |= layer_cache_update;
    }

    /// Clears bits in a specific group in the bit field.
    ///
    /// # Arguments
    /// group_index - Provides the group within the bit field to clear.
    /// group_field - Provides the bits to clear within the group.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the group_index is valid for the size of
    /// the bit field. The caller must guarantee that group_index is within the number of
    /// groups in the bit field.
    pub unsafe fn clear_group_unchecked(&mut self, group_index: usize, group_field: usize) {
        let subfield = self.bitfield.get_unchecked_mut(group_index);
        *subfield &= !group_field;

        let is_clear = (*subfield == 0) as usize;
        let layer_cache_update = (1 << group_index) * is_clear;
        self.layer_cache &= !layer_cache_update;
    }
}

/// Defines the FastBitField interface for LargeBitField.
impl FastBitField for LargeBitField {
    /// Gets the number of bits available in the bitfield type.
    ///
    /// # Returns
    /// The number of bits available.
    ///
    /// # Examples
    /// ```
    /// use fast_bitfield::{FastBitField, LargeBitField};
    ///
    /// let bits_of = core::mem::size_of::<usize>() * 8;
    /// assert_eq!(LargeBitField::get_number_of_bits(), bits_of * bits_of);
    /// ```
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
    /// The lowest set bit index or `None` if no bits are set.
    fn get_lowest_set_bit(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        Some(self.get_lowest_set_bit_unchecked())
    }

    /// Gets the highest set bit.
    ///
    /// # Returns
    /// The highest set bit index or `None` if no bits are set.
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
    /// `Some(true)` if bit is set.
    /// `Some(false)` if bit is cleared.
    /// `None` if index is invalid.
    fn test_bit(&self, index: usize) -> Option<bool> {
        if index < LARGE_BIT_FIELD_BIT_SIZE {
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
    /// # Returns
    /// `true` if empty, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use fast_bitfield::{FastBitField, LargeBitField};
    ///
    /// const BITS_OF: usize = core::mem::size_of::<usize>() * 8;
    ///
    /// let mut large = LargeBitField::new();
    ///
    /// let clear_value = [core::usize::MAX; BITS_OF];
    ///
    /// large.clear_field(&clear_value);
    /// assert!(large.is_empty());
    ///
    /// large.set_bit(0);
    /// assert!(!large.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.layer_cache == 0
    }

    /// Gets the lowest set bit, guaranteed to have no branches and be in constant time, completely
    /// invariant of the state of the bit field. If no bits are set, the result is undefined.
    ///
    /// This function should only be used if the caller can guarantee the bitfield will always
    /// have at least one bit set.
    ///
    /// # Returns
    /// The lowest set bit index or `UNDEFINED` if no bits are set.
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
    /// The highest set bit index or `UNDEFINED` if no bits are set.
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

    /// Sets a bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to set.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    unsafe fn set_bit_unchecked(&mut self, index: usize) {
        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        self.layer_cache |= 1 << top_layer;
        let sub_field = self.bitfield.get_unchecked_mut(top_layer);
        *sub_field |= 1 << bottom_layer;
    }

    /// Clears a bit in the bit field
    ///
    /// # Arguments
    /// index - Provides the bit to clear.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    unsafe fn clear_bit_unchecked(&mut self, index: usize) {
        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_layer = index % SMALL_BIT_FIELD_BIT_SIZE;

        let sub_field = self.bitfield.get_unchecked_mut(top_layer);
        *sub_field &= !(1 << bottom_layer);

        let is_clear = (*sub_field == 0) as usize;
        let layer_cache_update = (1 << top_layer) * is_clear;
        self.layer_cache &= !layer_cache_update
    }

    /// Gets the value of a specific bit in the bit field.
    ///
    /// # Arguments
    /// index - Provides the bit to test.
    ///
    /// # Returns
    /// `true` if bit is set.
    /// `false` if bit is cleared.
    ///
    /// # Unsafe
    /// This unsafe variant does not check if the index is valid for the size of
    /// the bit field. The caller must guarantee that the index is less than `get_number_of_bits()`.
    unsafe fn test_bit_unchecked(&self, index: usize) -> bool {
        let top_layer = index / SMALL_BIT_FIELD_BIT_SIZE;
        let bottom_mask = 1 << (index % SMALL_BIT_FIELD_BIT_SIZE);

        let sub_field = self.bitfield.get_unchecked(top_layer);
        (*sub_field & bottom_mask) != 0
    }
}

// RAZTODO: Doc Tests
// RAZTODO: Unit Tests
