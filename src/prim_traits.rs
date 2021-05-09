// Copyright © 2018–2021 Trevor Spiteri

// This library is free software: you can redistribute it and/or modify it under
// the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT License
// along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

use crate::{
    helpers::{FloatHelper, FloatKind, FromFloatHelper, IntHelper},
    traits::{Fixed, FromFixed, ToFixed},
    F128Bits,
};
use half::{bf16, f16};

impl ToFixed for bool {
    /// Converts a [`bool`] to a fixed-point number.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled, panics if the value does
    /// not fit. When debug assertions are not enabled, the wrapped
    /// value can be returned, but it is not considered a breaking
    /// change if in the future it panics; if wrapping is required use
    /// [`wrapping_to_fixed`] instead.
    ///
    /// [`wrapping_to_fixed`]: ToFixed::wrapping_to_fixed
    #[inline]
    fn to_fixed<F: Fixed>(self) -> F {
        ToFixed::to_fixed(self as u8)
    }

    /// Converts a [`bool`] to a fixed-point number if it fits, otherwise returns [`None`].
    #[inline]
    fn checked_to_fixed<F: Fixed>(self) -> Option<F> {
        ToFixed::checked_to_fixed(self as u8)
    }

    /// Convert a [`bool`] to a fixed-point number, saturating if it does not fit.
    #[inline]
    fn saturating_to_fixed<F: Fixed>(self) -> F {
        ToFixed::saturating_to_fixed(self as u8)
    }

    /// Converts a [`bool`] to a fixed-point number, wrapping if it does not fit.
    #[inline]
    fn wrapping_to_fixed<F: Fixed>(self) -> F {
        ToFixed::wrapping_to_fixed(self as u8)
    }

    /// Converts a [`bool`] to a fixed-point number.
    ///
    /// Returns a [tuple] of the fixed-point number and a [`bool`]
    /// indicating whether an overflow has occurred. On overflow, the
    /// wrapped value is returned.
    #[inline]
    fn overflowing_to_fixed<F: Fixed>(self) -> (F, bool) {
        ToFixed::overflowing_to_fixed(self as u8)
    }

    /// Converts a [`bool`] to a fixed-point number, panicking if it
    /// does not fit.
    ///
    /// # Panics
    ///
    /// Panics if the value does not fit, even when debug assertions
    /// are not enabled.
    #[inline]
    #[track_caller]
    fn unwrapped_to_fixed<F: Fixed>(self) -> F {
        ToFixed::unwrapped_to_fixed(self as u8)
    }
}

macro_rules! impl_int {
    ($Int:ident) => {
        impl FromFixed for $Int {
            /// Converts a fixed-point number to an integer.
            ///
            /// Any fractional bits are discarded, which rounds towards −∞.
            ///
            /// # Panics
            ///
            /// When debug assertions are enabled, panics if the value
            /// does not fit. When debug assertions are not enabled,
            /// the wrapped value can be returned, but it is not
            /// considered a breaking change if in the future it
            /// panics; if wrapping is required use
            /// [`wrapping_from_fixed`] instead.
            ///
            /// [`wrapping_from_fixed`]: FromFixed::wrapping_from_fixed
            #[inline]
            fn from_fixed<F: Fixed>(src: F) -> Self {
                $Int::from_repr_fixed(FromFixed::from_fixed(src))
            }

            /// Converts a fixed-point number to an integer if it fits, otherwise returns [`None`].
            ///
            /// Any fractional bits are discarded, which rounds towards −∞.
            #[inline]
            fn checked_from_fixed<F: Fixed>(src: F) -> Option<Self> {
                FromFixed::checked_from_fixed(src).map($Int::from_repr_fixed)
            }

            /// Converts a fixed-point number to an integer, saturating if it does not fit.
            ///
            /// Any fractional bits are discarded, which rounds towards −∞.
            #[inline]
            fn saturating_from_fixed<F: Fixed>(src: F) -> Self {
                $Int::from_repr_fixed(FromFixed::saturating_from_fixed(src))
            }

            /// Converts a fixed-point number to an integer, wrapping if it does not fit.
            ///
            /// Any fractional bits are discarded, which rounds towards −∞.
            #[inline]
            fn wrapping_from_fixed<F: Fixed>(src: F) -> Self {
                $Int::from_repr_fixed(FromFixed::wrapping_from_fixed(src))
            }

            /// Converts a fixed-point number to an integer.
            ///
            /// Returns a [tuple] of the value and a [`bool`] indicating whether
            /// an overflow has occurred. On overflow, the wrapped value is
            /// returned.
            ///
            /// Any fractional bits are discarded, which rounds towards −∞.
            #[inline]
            fn overflowing_from_fixed<F: Fixed>(src: F) -> (Self, bool) {
                let (repr_fixed, overflow) = FromFixed::overflowing_from_fixed(src);
                ($Int::from_repr_fixed(repr_fixed), overflow)
            }

            /// Converts a fixed-point number to an integer, panicking if it does not fit.
            ///
            /// Any fractional bits are discarded, which rounds towards −∞.
            ///
            /// # Panics
            ///
            /// Panics if the value
            /// does not fit, even when debug assertions are not enabled.
            #[inline]
            fn unwrapped_from_fixed<F: Fixed>(src: F) -> Self {
                $Int::from_repr_fixed(FromFixed::unwrapped_from_fixed(src))
            }
        }

        impl ToFixed for $Int {
            /// Converts an integer to a fixed-point number.
            ///
            /// # Panics
            ///
            /// When debug assertions are enabled, panics if the value
            /// does not fit. When debug assertions are not enabled,
            /// the wrapped value can be returned, but it is not
            /// considered a breaking change if in the future it
            /// panics; if wrapping is required use
            /// [`wrapping_to_fixed`] instead.
            ///
            /// [`wrapping_to_fixed`]: ToFixed::wrapping_to_fixed
            #[inline]
            fn to_fixed<F: Fixed>(self) -> F {
                ToFixed::to_fixed(self.to_repr_fixed())
            }

            /// Converts an integer to a fixed-point number if it fits, otherwise returns [`None`].
            #[inline]
            fn checked_to_fixed<F: Fixed>(self) -> Option<F> {
                ToFixed::checked_to_fixed(self.to_repr_fixed())
            }

            /// Converts an integer to a fixed-point number, saturating if it does not fit.
            #[inline]
            fn saturating_to_fixed<F: Fixed>(self) -> F {
                ToFixed::saturating_to_fixed(self.to_repr_fixed())
            }

            /// Converts an integer to a fixed-point number, wrapping if it does not fit.
            #[inline]
            fn wrapping_to_fixed<F: Fixed>(self) -> F {
                ToFixed::wrapping_to_fixed(self.to_repr_fixed())
            }

            /// Converts an integer to a fixed-point number.
            ///
            /// Returns a [tuple] of the fixed-point number and a [`bool`]
            /// indicating whether an overflow has occurred. On overflow, the
            /// wrapped value is returned.
            #[inline]
            fn overflowing_to_fixed<F: Fixed>(self) -> (F, bool) {
                ToFixed::overflowing_to_fixed(self.to_repr_fixed())
            }

            /// Converts an integer to a fixed-point number, panicking if it does not fit.
            ///
            /// # Panics
            ///
            /// Panics if the value does not fit, even when debug
            /// assertions are not enabled.
            #[inline]
            fn unwrapped_to_fixed<F: Fixed>(self) -> F {
                ToFixed::unwrapped_to_fixed(self.to_repr_fixed())
            }
        }
    };
}

impl_int! { i8 }
impl_int! { i16 }
impl_int! { i32 }
impl_int! { i64 }
impl_int! { i128 }
impl_int! { isize }
impl_int! { u8 }
impl_int! { u16 }
impl_int! { u32 }
impl_int! { u64 }
impl_int! { u128 }
impl_int! { usize }

macro_rules! impl_float {
    ($Float:ty, $link:expr, $overflows_fmt:expr, $overflows_filt:expr) => {
        impl FromFixed for $Float {
            /// Converts a fixed-point number to a floating-point number.
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            ///
            /// # Panics
            ///
            /// When debug assertions are enabled, panics if the value
            /// does not fit. When debug assertions are not enabled,
            /// the wrapped value can be returned, but it is not
            /// considered a breaking change if in the future it
            /// panics; if wrapping is required use
            /// [`wrapping_from_fixed`] instead.
            ///
            /// [`wrapping_from_fixed`]: FromFixed::wrapping_from_fixed
            #[inline]
            fn from_fixed<F: Fixed>(src: F) -> Self {
                let helper = src.private_to_float_helper();
                FloatHelper::from_to_float_helper(helper, F::FRAC_NBITS, F::INT_NBITS)
            }

            /// Converts a fixed-point number to a floating-point
            /// number if it fits, otherwise returns [`None`].
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            #[inline]
            fn checked_from_fixed<F: Fixed>(src: F) -> Option<Self> {
                Some(FromFixed::from_fixed(src))
            }

            /// Converts a fixed-point number to a floating-point
            /// number, saturating if it does not fit.
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            #[inline]
            fn saturating_from_fixed<F: Fixed>(src: F) -> Self {
                FromFixed::from_fixed(src)
            }

            /// Converts a fixed-point number to a floating-point
            /// number, wrapping if it does not fit.
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            #[inline]
            fn wrapping_from_fixed<F: Fixed>(src: F) -> Self {
                FromFixed::from_fixed(src)
            }

            /// Converts a fixed-point number to a floating-point number.
            ///
            /// Returns a [tuple] of the value and a [`bool`]
            /// indicating whether an overflow has occurred. On
            /// overflow, the wrapped value is returned.
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            #[inline]
            fn overflowing_from_fixed<F: Fixed>(src: F) -> (Self, bool) {
                (FromFixed::from_fixed(src), false)
            }

            /// Converts a fixed-point number to a floating-point
            /// number, panicking if it does not fit.
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            ///
            /// # Panics
            ///
            /// Panics if the value does not fit, even when debug
            /// assertions are not enabled.
            #[inline]
            fn unwrapped_from_fixed<F: Fixed>(src: F) -> Self {
                FromFixed::from_fixed(src)
            }
        }

        impl ToFixed for $Float {
            comment! {
                "Converts a floating-point number to a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if `self` is not [finite].

When debug assertions are enabled, also panics if the value does not
fit. When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future
it panics; if wrapping is required use [`wrapping_to_fixed`] instead.

[`wrapping_to_fixed`]: ToFixed::wrapping_to_fixed
[finite]: ", $link, "::is_finite
";
                #[inline]
                fn to_fixed<F: Fixed>(self) -> F {
                    let (wrapped, overflow) = ToFixed::overflowing_to_fixed(self);
                    debug_assert!(!overflow, $overflows_fmt, $overflows_filt(self));
                    let _ = overflow;
                    wrapped
                }
            }

            /// Converts a floating-point number to a fixed-point
            /// number if it fits, otherwise returns [`None`].
            ///
            /// Rounding is to the nearest, with ties rounded to even.
            #[inline]
            fn checked_to_fixed<F: Fixed>(self) -> Option<F> {
                let kind = self.to_float_kind(F::FRAC_NBITS, F::INT_NBITS);
                match kind {
                    FloatKind::Finite { .. } => {
                        let helper = FromFloatHelper { kind };
                        match F::private_overflowing_from_float_helper(helper) {
                            (_, true) => None,
                            (wrapped, false) => Some(wrapped),
                        }
                    }
                    _ => None,
                }
            }

            comment! {
                "Converts a floating-point number to a fixed-point
number, saturating if it does not fit.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if `self` is [NaN].

[NaN]: ", $link, "::is_nan
";
                #[inline]
                fn saturating_to_fixed<F: Fixed>(self) -> F {
                    let kind = self.to_float_kind(F::FRAC_NBITS, F::INT_NBITS);
                    let helper = FromFloatHelper { kind };
                    F::private_saturating_from_float_helper(helper)
                }
            }

            comment! {
                "Converts a floating-point number to a fixed-point
number, wrapping if it does not fit.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if `self` is not [finite].

[finite]: ", $link, "::is_finite
";
                #[inline]
                fn wrapping_to_fixed<F: Fixed>(self) -> F {
                    let (wrapped, _) = ToFixed::overflowing_to_fixed(self);
                    wrapped
                }
            }

            comment! {
            "Converts a floating-point number to a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if `self` is not [finite].

[finite]: ", $link, "::is_finite
";
                #[inline]
                #[track_caller]
                fn overflowing_to_fixed<F: Fixed>(self) -> (F, bool) {
                    let kind = self.to_float_kind(F::FRAC_NBITS, F::INT_NBITS);
                    let helper = FromFloatHelper { kind };
                    F::private_overflowing_from_float_helper(helper)
                }
            }

            comment! {
                "Converts a floating-point number to a fixed-point
number, panicking if it does not fit.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if `self` is not [finite] or if the value does not fit, even
when debug assertions are not enabled.

[finite]: ", $link, "::is_finite
";
                #[inline]
                fn unwrapped_to_fixed<F: Fixed>(self) -> F {
                    match ToFixed::overflowing_to_fixed(self) {
                        (val, false) => val,
                        (_, true) => panic!("overflow"),
                    }
                }
            }
        }
    };
}

impl_float! { f16, "f16", "{} overflows", |x| x }
impl_float! { bf16, "bf16", "{} overflows", |x| x }
impl_float! { f32, "f32", "{} overflows", |x| x }
impl_float! { f64, "f64", "{} overflows", |x| x }
impl_float! { F128Bits, "f64", "F128Bits({}) overflows", |x: F128Bits| x.0 }
