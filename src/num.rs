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
    /// Zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::extra::U4;
    /// use fixed::Fixed;
    /// type Fix = Fixed<i32, U4>;
    /// assert_eq!(Fix::ZERO, Fix::from_bits(0));
    /// ```
    pub const ZERO: Fixed<Bits, Frac> = Fixed::from_bits(Bits::ZERO);

    /// The difference between any two successive representable numbers, <i>Δ</i>.
    ///
    /// If the number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional bits, then
    /// <i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>f</i></sup>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::extra::U4;
    /// use fixed::Fixed;
    /// type Fix = Fixed<i32, U4>;
    /// assert_eq!(Fix::DELTA, Fix::from_bits(1));
    /// // binary 0.0001 is decimal 0.0625
    /// assert_eq!(Fix::DELTA, 0.0625);
    /// ```
    pub const DELTA: Fixed<Bits, Frac> = Fixed::from_bits(Bits::ONE);

    /// The smallest value that can be represented.
    ///
    /// If the number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional bits,
    /// then
    ///   * for signed numbers, the minimum is
    ///     &minus;2<sup><i>n</i>&nbsp;&minus;&nbsp;1</sup>/2<sup><i>f</i></sup>.
    ///   * for unsigned numbers, the minimum is 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::extra::U4;
    /// use fixed::Fixed;
    /// type Fix = Fixed<i32, U4>;
    /// assert_eq!(Fix::MIN, Fix::from_bits(i32::MIN));
    /// ```
    pub const MIN: Fixed<Bits, Frac> = Fixed::from_bits(Bits::MIN);

    /// The largest value that can be represented.
    ///
    /// If the number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional bits, then
    ///   * for signed numbers, the maximum is
    ///     (2<sup><i>n</i>&nbsp;&minus;&nbsp;1</sup>/2<sup><i>f</i></sup>&nbsp;&minus;&nbsp;1)/2<sup><i>f</i></sup>.
    ///   * for unsigned numbers, the maximum is
    ///     (2<sup><i>n</i></sup>/2<sup><i>f</i></sup>&nbsp;&minus;&nbsp;1)/2<sup><i>f</i></sup>.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::extra::U4;
    /// use fixed::Fixed;
    /// type Fix = Fixed<i32, U4>;
    /// assert_eq!(Fix::MAX, Fix::from_bits(i32::MAX));
    /// ```
    pub const MAX: Fixed<Bits, Frac> = Fixed::from_bits(Bits::MAX);

    /// [`true`] if the number type is signed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fixed::types::extra::U4;
    /// use fixed::Fixed;
    /// type Fix = Fixed<i32, U4>;
    /// assert!(Fixed::<i32, U4>::IS_SIGNED);
    /// assert!(!Fixed::<u32, U4>::IS_SIGNED);
    /// ```
    pub const IS_SIGNED: bool = Bits::IS_SIGNED;

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
