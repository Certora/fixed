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

macro_rules! fixed_from_to {
    (
        {Self, Inner} = {$Self:ident, $Inner:ident},
        Signedness = $Signedness:ident,
        n = $n:literal,
    ) => {
        comment! {
            r#"Creates a fixed-point number from another number.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[to\_fixed][ToFixed::to_fixed]\()</code>.

# Panics

For floating-point numbers, panics if the value is not [finite].

When debug assertions are enabled, panics if the value does not fit.
When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future
it panics; if wrapping is required use [`wrapping_from_num`] instead.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::from_num(src), Fix::from_bits(0b111 << (4 - 2)));

assert_eq!(Fix::from_num(3i32), Fix::from_bits(3 << 4));
assert_eq!(Fix::from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-3i64), Fix::from_bits(-",
                "3i64), Fix::from_bits(",
            ),
            "3 << 4));

assert_eq!(Fix::from_num(1.75f32), Fix::from_bits(0b111 << (4 - 2)));
assert_eq!(Fix::from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-1.75f64), Fix::from_bits(-",
                "1.75f64), Fix::from_bits(",
            ),
            "0b111 << (4-2)));
```

[`wrapping_from_num`]: Self::wrapping_from_num
[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn from_num<Src: ToFixed>(src: Src) -> $Self<FRAC> {
                src.to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[from\_fixed][FromFixed::from_fixed]\(self)</code>.

# Panics

When debug assertions are enabled, panics if the value does not fit.
When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future
it panics; if wrapping is required use [`wrapping_to_num`] instead.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I30F2, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(src.to_num::<I30F2>(), I30F2::from_bits(0b111));
// src >> 2 is 0.0111, which for I30F2 is truncated to 0.01
assert_eq!((src >> 2u32).to_num::<I30F2>(), I30F2::from_bits(0b1));

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.to_num::<i32>(), 2);
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                "(-two_point_5).to_num::<i64>(), -3",
                "two_point_5.to_num::<i64>(), 2",
            ),
            ");

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.to_num::<f32>(), 1.625f32);
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                "(-one_point_625).to_num::<f64>(), -",
                "one_point_625.to_num::<f64>(), "
            ),
            "1.625f64);
```

[`wrapping_to_num`]: Self::wrapping_to_num
";
            #[inline]
            #[track_caller]
            #[must_use]
            pub fn to_num<Dst: FromFixed>(self) -> Dst {
                Dst::from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number if it
fits, otherwise returns [`None`].

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[checked\_to\_fixed][ToFixed::checked_to_fixed]\()</code>.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::checked_from_num(src), Some(Fix::from_bits(0b111 << (4 - 2))));
let too_large = ", stringify!($Self), "::<2>::MAX;
assert!(Fix::checked_from_num(too_large).is_none());

assert_eq!(Fix::checked_from_num(3), Some(Fix::from_bits(3 << 4)));
let too_large = ", stringify!($Inner), "::MAX;
assert!(Fix::checked_from_num(too_large).is_none());
let too_small = ",
            if_signed_unsigned!(
                $Signedness,
                concat!(stringify!($Inner), "::MIN"),
                "-1",
            ),
            ";
assert!(Fix::checked_from_num(too_small).is_none());

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::checked_from_num(1.75f32), Some(expected));
assert_eq!(Fix::checked_from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-1.75f64), Some(-",
                "1.75f64), Some(",
            ),
            "expected));
assert!(Fix::checked_from_num(2e38).is_none());
assert!(Fix::checked_from_num(std::f64::NAN).is_none());
```
";
            #[inline]
            pub fn checked_from_num<Src: ToFixed>(src: Src) -> Option<$Self<FRAC>> {
                src.checked_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number if it
fits, otherwise returns [`None`].

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[checked\_from\_fixed][FromFixed::checked_from_fixed]\(self)</code>.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.checked_to_num::<I16F16>(), Some(expected));
type TooFewIntBits = ", stringify!($Self), "<6>;
assert!(Fix::MAX.checked_to_num::<TooFewIntBits>().is_none());

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.checked_to_num::<i32>(), Some(2));
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                "(-two_point_5).checked_to_num::<i64>(), Some(-3",
                "two_point_5.checked_to_num::<i64>(), Some(2",
            ),
            "));
type AllInt = ", stringify!($Self), "<0>;
assert!(AllInt::",
            if_signed_unsigned!(
                $Signedness,
                "from_bits(-1).checked_to_num::<u",
                "MAX.checked_to_num::<i",
            ),
            $n, ">().is_none());

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.checked_to_num::<f32>(), Some(1.625f32));
```
";
            #[inline]
            #[must_use]
            pub fn checked_to_num<Dst: FromFixed>(self) -> Option<Dst> {
                Dst::checked_from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number,
saturating if it does not fit.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[saturating\_to\_fixed][ToFixed::saturating_to_fixed]\()</code>.

# Panics

This method panics if the value is a floating-point [NaN].

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::saturating_from_num(src), Fix::from_bits(0b111 << (4 - 2)));
let too_large = ", stringify!($Self), "::<2>::MAX;
assert_eq!(Fix::saturating_from_num(too_large), Fix::MAX);

assert_eq!(Fix::saturating_from_num(3), Fix::from_bits(3 << 4));
let too_small = ",
            if_signed_unsigned!(
                $Signedness,
                concat!(stringify!($Inner), "::MIN"),
                "-1",
            ),
            ";
assert_eq!(Fix::saturating_from_num(too_small), Fix::MIN);

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::saturating_from_num(1.75f32), expected);
assert_eq!(Fix::saturating_from_num(",
            if_signed_unsigned!(
                $Signedness,
                "-1.75f64), -",
                "1.75f64), ",
            ),
            "expected);
assert_eq!(Fix::saturating_from_num(2e38), Fix::MAX);
assert_eq!(Fix::saturating_from_num(std::f64::NEG_INFINITY), Fix::MIN);
```

[NaN]: f64::is_nan
";
            #[inline]
            #[track_caller]
            pub fn saturating_from_num<Src: ToFixed>(src: Src) -> $Self<FRAC> {
                src.saturating_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number,
saturating the value if it does not fit.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[saturating\_from\_fixed][FromFixed::saturating_from_fixed]\(self)</code>.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.saturating_to_num::<I16F16>(), expected);
type TooFewIntBits = ", stringify!($Self), "<6>;
let saturated = Fix::MAX.saturating_to_num::<TooFewIntBits>();
assert_eq!(saturated, TooFewIntBits::MAX);

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.saturating_to_num::<i32>(), 2);
type AllInt = ", stringify!($Self), "<0>;
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                concat!("AllInt::from_bits(-1).saturating_to_num::<u", $n, ">(), 0"),
                concat!(
                    "AllInt::MAX.saturating_to_num::<i", $n, ">(), ",
                    "i", $n, "::MAX",
                ),
            ),
            ");

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.saturating_to_num::<f32>(), 1.625f32);
```
";
            #[inline]
            #[must_use]
            pub fn saturating_to_num<Dst: FromFixed>(self) -> Dst {
                Dst::saturating_from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number,
wrapping the value on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[wrapping\_to\_fixed][ToFixed::wrapping_to_fixed]\()</code>.

# Panics

For floating-point numbers, panics if the value is not [finite].

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::wrapping_from_num(src), Fix::from_bits(0b111 << (4 - 2)));
// integer 0b1101 << (", $n, " - 7) will wrap to fixed-point 1010...
let too_large = ", stringify!($Self), "::<0>::from_bits(0b1101 << (", $n, " - 7));
let wrapped = Fix::from_bits(0b1010 << (", $n, " - 4));
assert_eq!(Fix::wrapping_from_num(too_large), wrapped);

// integer 0b1101 << (", $n, " - 7) will wrap to fixed-point 1010...
let large: ", stringify!($Inner), " = 0b1101 << (", $n, " - 7);
let wrapped = Fix::from_bits(0b1010 << (", $n, " - 4));
assert_eq!(Fix::wrapping_from_num(large), wrapped);

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::wrapping_from_num(1.75f32), expected);
// 1.75 << (", $n, " - 4) wraps to binary 11000...
let large = 1.75 * 2f32.powi(", $n, " - 4);
let wrapped = Fix::from_bits(0b1100 << (", $n, " - 4));
assert_eq!(Fix::wrapping_from_num(large), wrapped);
```

[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn wrapping_from_num<Src: ToFixed>(src: Src) -> $Self<FRAC> {
                src.wrapping_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number,
wrapping the value on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[wrapping\_from\_fixed][FromFixed::wrapping_from_fixed]\(self)</code>.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.wrapping_to_num::<I16F16>(), expected);
type TooFewIntBits = ", stringify!($Self), "<6>;
let wrapped = TooFewIntBits::from_bits(Fix::MAX.to_bits() << 2);
assert_eq!(Fix::MAX.wrapping_to_num::<TooFewIntBits>(), wrapped);

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.wrapping_to_num::<i32>(), 2);
type AllInt = ", stringify!($Self), "<0>;
assert_eq!(",
            if_signed_unsigned!(
                $Signedness,
                concat!(
                    "AllInt::from_bits(-1).wrapping_to_num::<u", $n, ">(), ",
                    "u", $n, "::MAX",
                ),
                concat!("AllInt::MAX.wrapping_to_num::<i", $n, ">(), -1"),
            ),
            ");

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.wrapping_to_num::<f32>(), 1.625f32);
```
";
            #[inline]
            #[must_use]
            pub fn wrapping_to_num<Dst: FromFixed>(self) -> Dst {
                Dst::wrapping_from_fixed(self)
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number,
panicking on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[unwrapped\_to\_fixed][ToFixed::unwrapped_to_fixed]\()</code>.

# Panics

Panics if the value does not fit.

For floating-point numbers, also panics if the value is not [finite].

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(Fix::unwrapped_from_num(src), Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of overflow.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let too_large = ", stringify!($Self), "::<0>::from_bits(0b1101 << (", $n, " - 7));
let _overflow = Fix::unwrapped_from_num(too_large);
```

[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn unwrapped_from_num<Src: ToFixed>(src: Src) -> $Self<FRAC> {
                match src.overflowing_to_fixed() {
                    (_, true) => panic!("overflow"),
                    (ans, false) => ans,
                }
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number,
panicking on overflow.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[unwrapped\_from\_fixed][FromFixed::unwrapped_from_fixed]\(self)</code>.

# Panics

Panics if the value does not fit.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.unwrapped_to_num::<I16F16>(), expected);
```

The following panics because of overflow.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
type TooFewIntBits = ", stringify!($Self), "<6>;
let _overflow = Fix::MAX.unwrapped_to_num::<TooFewIntBits>();
```
";
            #[inline]
            #[track_caller]
            #[must_use]
            pub fn unwrapped_to_num<Dst: FromFixed>(self) -> Dst {
                match Dst::overflowing_from_fixed(self) {
                    (_, true) => panic!("overflow"),
                    (ans, false) => ans,
                }
            }
        }

        comment! {
            r#"Creates a fixed-point number from another number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`].
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other number `src` for which [`ToFixed`] is implemented, in
    which case this method returns
    <code>src.[overflowing\_to\_fixed][ToFixed::overflowing_to_fixed]\()</code>.

# Panics

For floating-point numbers, panics if the value is not [finite].

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = I16F16::from_bits(0b111 << (16 - 2));
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::overflowing_from_num(src), (expected, false));
// integer 0b1101 << (", $n, " - 7) will wrap to fixed-point 1010...
let too_large = ", stringify!($Self), "::<0>::from_bits(0b1101 << (", $n, " - 7));
let wrapped = Fix::from_bits(0b1010 << (", $n, " - 4));
assert_eq!(Fix::overflowing_from_num(too_large), (wrapped, true));

assert_eq!(Fix::overflowing_from_num(3), (Fix::from_bits(3 << 4), false));
// integer 0b1101 << (", $n, " - 7) will wrap to fixed-point 1010...
let large: ", stringify!($Inner), " = 0b1101 << (", $n, " - 7);
let wrapped = Fix::from_bits(0b1010 << (", $n, " - 4));
assert_eq!(Fix::overflowing_from_num(large), (wrapped, true));

// 1.75 is 1.11 in binary
let expected = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(Fix::overflowing_from_num(1.75f32), (expected, false));
// 1.75 << (", $n, " - 4) wraps to binary 11000...
let large = 1.75 * 2f32.powi(", $n, " - 4);
let wrapped = Fix::from_bits(0b1100 << (", $n, " - 4));
assert_eq!(Fix::overflowing_from_num(large), (wrapped, true));
```

[finite]: f64::is_finite
";
            #[inline]
            #[track_caller]
            pub fn overflowing_from_num<Src: ToFixed>(src: Src) -> ($Self<FRAC>, bool) {
                src.overflowing_to_fixed()
            }
        }

        comment! {
            r#"Converts a fixed-point number to another number.

Returns a [tuple] of the number and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

The other number can be:

  * Another fixed-point number. Any extra fractional bits are
    discarded, which rounds towards &minus;∞.
  * An integer of type [`i8`], [`i16`], [`i32`], [`i64`], [`i128`],
    [`isize`], [`u8`], [`u16`], [`u32`], [`u64`], [`u128`], or
    [`usize`]. Any fractional bits are discarded, which rounds towards
    &minus;∞.
  * A floating-point number of type [`f16`], [`f32`], [`f64`], [`f128`],
    <code>[half]::[f16][half::f16]</code> or
    <code>[half]::[bf16][half::bf16]</code>. For this conversion, the method
    rounds to the nearest, with ties rounding to even.
  * Any other type `Dst` for which [`FromFixed`] is implemented, in
    which case this method returns
    <code>Dst::[overflowing\_from\_fixed][FromFixed::overflowing_from_fixed]\(self)</code>.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::{types::I16F16, "#, stringify!($Self), "};
type Fix = ", stringify!($Self), "<4>;

// 1.75 is 1.11 in binary
let src = Fix::from_bits(0b111 << (4 - 2));
let expected = I16F16::from_bits(0b111 << (16 - 2));
assert_eq!(src.overflowing_to_num::<I16F16>(), (expected, false));
type TooFewIntBits = ", stringify!($Self), "<6>;
let wrapped = TooFewIntBits::from_bits(Fix::MAX.to_bits() << 2);
assert_eq!(Fix::MAX.overflowing_to_num::<TooFewIntBits>(), (wrapped, true));

// 2.5 is 10.1 in binary
let two_point_5 = Fix::from_bits(0b101 << (4 - 1));
assert_eq!(two_point_5.overflowing_to_num::<i32>(), (2, false));
let does_not_fit = ", stringify!($Self), "::<0>::",
            if_signed_unsigned!($Signedness, "from_bits(-1)", "MAX"),
            ";
let wrapped = ",
            if_signed_unsigned!(
                $Signedness,
                concat!("1u", $n, ".wrapping_neg()"),
                concat!("-1i", $n),
            ),
            ";
assert_eq!(does_not_fit.overflowing_to_num::<",
            if_signed_unsigned!($Signedness, "u", "i"),
            $n, ">(), (wrapped, true));

// 1.625 is 1.101 in binary
let one_point_625 = Fix::from_bits(0b1101 << (4 - 3));
assert_eq!(one_point_625.overflowing_to_num::<f32>(), (1.625f32, false));
```
";
            #[inline]
            #[must_use]
            pub fn overflowing_to_num<Dst: FromFixed>(self) -> (Dst, bool) {
                Dst::overflowing_from_fixed(self)
            }
        }

        /// Creates a fixed-point number from a fixed-point number with the same
        /// underlying integer type. Usable in constant context.
        ///
        /// This is equivalent to the [`unwrapped_from_num`] method with
        #[doc = concat!("<code>[", stringify!($Self), "]&lt;OtherFrac></code>")]
        /// as its generic parameter, but can also be used in constant context.
        /// Unless required in constant context, use [`unwrapped_from_num`] or
        /// [`from_num`] instead.
        ///
        /// # Planned deprecation
        ///
        /// This method will be deprecated when the [`unwrapped_from_num`]
        /// method is usable in constant context.
        ///
        /// # Panics
        ///
        /// Panics if the value does not fit.
        ///
        /// # Examples
        ///
        /// ```rust
        #[doc = concat!("use fixed::", stringify!($Self), ";")]
        #[doc = concat!("type FixA = ", stringify!($Self), "<2>;")]
        #[doc = concat!("type FixB = ", stringify!($Self), "<4>;")]
        /// const A: FixA = FixA::unwrapped_from_str("3.5");
        /// const B: FixB = FixB::const_from_fixed(A);
        /// assert_eq!(B, 3.5);
        /// ```
        ///
        /// The following would fail to compile because of overflow.
        ///
        /// ```rust,compile_fail
        #[doc = concat!("use fixed::", stringify!($Self), ";")]
        #[doc = concat!(
            "const _OVERFLOW: ", stringify!($Self), "<4> = ",
            stringify!($Self), "::const_from_fixed(",
            stringify!($Self), "::<2>::MAX);"
        )]
        /// ```
        ///
        /// [`from_num`]: Self::from_num
        /// [`unwrapped_from_num`]: Self::unwrapped_from_num
        #[inline]
        #[track_caller]
        #[must_use]
        pub const fn const_from_fixed<const SRC_FRAC: i32>(src: $Self<SRC_FRAC>) -> $Self<FRAC> {
            let shift_left = FRAC - SRC_FRAC;
            let nbits = $Inner::BITS as i32;
            let src_bits = src.to_bits();
            let bits = if shift_left <= -nbits {
                src_bits >> (nbits / 2) >> (nbits / 2)
            } else if shift_left <= 0 {
                src_bits >> -shift_left
            } else if shift_left >= nbits {
                if src_bits != 0 {
                    panic!("overflow");
                }
                0
            } else {
                let shifted = src_bits << shift_left;
                if (shifted >> shift_left) != src_bits {
                    panic!("overflow");
                }
                shifted
            };
            Self::from_bits(bits)
        }

        /// Creates a fixed-point number from the underlying integer type
        #[doc = concat!("[`", stringify!($Inner), "`].")]
        /// Usable in constant context.
        ///
        /// This is equivalent to the [`unwrapped_from_num`] method with
        #[doc = concat!("[`", stringify!($Inner), "`]")]
        /// as its generic parameter, but can also be used in constant context.
        /// Unless required in constant context, use [`unwrapped_from_num`] or
        /// [`from_num`] instead.
        ///
        /// # Planned deprecation
        ///
        /// This method will be deprecated when the [`unwrapped_from_num`]
        /// method is usable in constant context.
        ///
        /// # Panics
        ///
        /// Panics if the value does not fit.
        ///
        /// # Examples
        ///
        /// ```rust
        /// #![feature(generic_const_exprs)]
        /// # #![allow(incomplete_features)]
        ///
        #[doc = concat!("use fixed::", stringify!($Self), ";")]
        #[doc = concat!("type Fix = ", stringify!($Self), "<4>;")]
        /// const FIVE: Fix = Fix::const_from_int(5);
        /// assert_eq!(FIVE, 5);
        /// ```
        ///
        /// The following would fail to compile because of overflow.
        ///
        /// ```rust,compile_fail
        /// #![feature(generic_const_exprs)]
        /// # #![allow(incomplete_features)]
        ///
        #[doc = concat!("use fixed::", stringify!($Self), ";")]
        #[doc = concat!(
            "const _OVERFLOW: ", stringify!($Self), "<4> = ", stringify!($Self), "::const_from_int(", stringify!($Inner), "::MAX);"
        )]
        /// ```
        ///
        /// [`from_num`]: Self::from_num
        /// [`unwrapped_from_num`]: Self::unwrapped_from_num
        #[inline]
        #[track_caller]
        #[must_use]
        pub const fn const_from_int(src: $Inner) -> $Self<FRAC> {
            Self::const_from_fixed($Self::<0>::from_bits(src))
        }
    };
}
