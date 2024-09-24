// Copyright © 2018–2024 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

use crate::traits::FixedBits;
use crate::Fixed;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

impl<Bits: FixedBits, Frac> Clone for Fixed<Bits, Frac> {
    #[inline]
    fn clone(&self) -> Fixed<Bits, Frac> {
        *self
    }
}

impl<Bits: FixedBits, Frac> Copy for Fixed<Bits, Frac> {}

impl<Bits: FixedBits, Frac> Default for Fixed<Bits, Frac> {
    #[inline]
    fn default() -> Fixed<Bits, Frac> {
        Fixed {
            bits: Bits::default(),
            phantom: PhantomData,
        }
    }
}

impl<Bits: FixedBits, Frac> Hash for Fixed<Bits, Frac> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}

impl<Bits: FixedBits, Frac> Fixed<Bits, Frac> {
    /// Creates a fixed-point number that has a bitwise representation identical
    /// to the given integer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::extra::U4;
    /// use fixed::Fixed;
    /// type Fix = Fixed<i32, U4>;
    /// // 0010.0000 = 2
    /// assert_eq!(Fix::from_bits(0b10_0000), 2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_bits(bits: Bits) -> Fixed<Bits, Frac> {
        Fixed {
            bits,
            phantom: PhantomData,
        }
    }

    /// Creates an integer that has a bitwise representation identical to the
    /// given fixed-point number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::extra::U4;
    /// use fixed::Fixed;
    /// type Fix = Fixed<Bits, U4>;
    /// // 2 is 0010.0000
    /// assert_eq!(Fix::from_num(2).to_bits(), 0b10_0000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_bits(self) -> Bits {
        self.bits
    }
}
