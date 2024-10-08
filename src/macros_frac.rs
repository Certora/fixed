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

macro_rules! fixed_frac {
    (
        {Self, Inner} = {$Self:ident, $Inner:ident},
        Signedness = $Signedness:ident,
        {nm4, nm1, n} = {$nm4:literal, $nm1:literal, $n:literal},
        {USelf, UInner} = {$USelf:ident, $UInner:ident},
    ) => {
        /// The items in this block are implemented for
        #[doc = concat!("0&nbsp;≤&nbsp;`FRAC`&nbsp;≤&nbsp;", $n, ".")]
        impl<const FRAC: i32> $Self<FRAC>
        where
            If<{ (0 <= FRAC) & (FRAC <= $n) }>: True,
        {
            comment! {
                "Parses a fixed-point literal.

Rounding is to the nearest, with ties rounded to even.

This is similar to [`from_str`][Self::from_str] but accepts a prefix for setting
the radix, and ignores underscores, such that the parsing is more similar to
numeric literals in Rust code.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "The string can start with “`-`” for a negative number.

",
                },
                r#"Strings starting with “`0b`” are parsed as binary, strings
starting with “`0o`” are parsed as octal, and strings starting with “`0x`” are
parsed as hexadecimal.

Exponents are supported as well. For decimal, binary and octal numbers, the
separator “`e`” or “`E`” can be used to start an exponent, which is then
followed by an optional sign “`+`” or “`-`”, and then by a decimal integer which
is the exponent. For hexadecimal numbers, since “`e`” and “`E`” are hexadecimal
digits, the separator “`@`” has to be used instead. The separator “`@`” is
accepted for all radices. The parsed value is scaled by the radix to the power
of the exponent.

For binary, octal and hexadecimal, base-2 exponents are supported too, using the
separator “`p`” or “`P`”. The parsed value is scaled by 2 to the power of the
exponent. For example, for hexadecimal “`P8`” means ×2⁸, and is equivalent to
“`@2`” which means ×16².

# Constants

The `lit` method is useful to write constant fixed-point literals, and can be
evaluated in constant context.

**Warning:** Normal calls to the `lit` method are *not* evaluated at compile
time. To ensure that the call is evaluated at compile time, `lit` must be used
in an inline constant expression, or used to initialize a constant.

For example, here `lit` would be evaluated at compile time:

```rust
# #![feature(generic_const_exprs)]
# #![allow(incomplete_features)]
# use fixed::FixedU32;
# type Fix = FixedU32<4>;
const ONE_AND_HALF: Fix = Fix::lit("1.5");
# assert_eq!(ONE_AND_HALF, 1.5);
```

However, here `lit` would be evaluated at run time:

```rust
# #![feature(generic_const_exprs)]
# #![allow(incomplete_features)]
# use fixed::FixedU32;
# type Fix = FixedU32<4>;
// Evaluated at run time.
let one_and_half = Fix::lit("1.5");
# assert_eq!(one_and_half, 1.5);
```

To evaluate at compile time without introducing a constant into the scope:

```rust
# #![feature(generic_const_exprs)]
# #![allow(incomplete_features)]
# use fixed::FixedU32;
# type Fix = FixedU32<4>;
// Evaluated at compile time.
let one_and_half = const { Fix::lit("1.5") };
# assert_eq!(one_and_half, 1.5);
```

# Panics

Panics if the number is not valid or overflows.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::"#, stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;

assert_eq!(Fix::lit("1.75"), 1.75);
assert_eq!(Fix::lit("1_.7_5_"), 1.75);
assert_eq!(Fix::lit("17.5e-1"), 1.75);
assert_eq!(Fix::lit("0_.017_5_e+0_2"), 1.75);
"#,
                if_signed_else_empty_str! {
                    $Signedness;
                    r#"assert_eq!(Fix::lit("-01.75"), -1.75);
"#,
                },
                r#"
assert_eq!(Fix::lit("0b1.11"), 1.75);
assert_eq!(Fix::lit("0b_111e-2"), 1.75);
"#,
                if_signed_else_empty_str! {
                    $Signedness;
                    r#"assert_eq!(Fix::lit("-0b1.11"), -1.75);
"#,
                },
                r#"
assert_eq!(Fix::lit("0o1.6"), 1.75);
assert_eq!(Fix::lit("0o.16E1"), 1.75);
assert_eq!(Fix::lit("0o7p-2"), 1.75);

assert_eq!(Fix::lit("0x1.C"), 1.75);
assert_eq!(Fix::lit("0x0.1C@1"), 1.75);
"#,
                if_signed_else_empty_str! {
                    $Signedness;
                    r#"assert_eq!(Fix::lit("-0x1.C"), -1.75);
"#,
                },
                r#"```

This method is useful to write constant fixed-point literals.

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::"#, stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
const ONE_AND_HALF: Fix = Fix::lit("1.5");
assert_eq!(ONE_AND_HALF, 1.5);
```
"#;
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn lit(src: &str) -> $Self<FRAC> {
                    match from_str::$Inner::lit(src, FRAC as u32) {
                        Ok(s) => $Self::from_bits(s),
                        Err(e) => panic!("{}", e.lit_message()),
                    }
                }
            }

            comment! {
                "Parses a string slice containing decimal digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. The separator “`e`”, “`E`” or “`@`”
can be used to start an exponent, which is then followed by an optional sign
“`+`” or “`-`”, and then by a decimal integer which is the exponent. The parsed
value is scaled by 10 to the power of the exponent.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
assert_eq!(Fix::from_str("1.75"), Ok(Fix::from_num(1.75)));
"#,
                if_signed_else_empty_str! {
                    $Signedness;
                    r#"assert_eq!(Fix::from_str("-1.75"), Ok(Fix::from_num(-1.75)));
"#,
                },
                r#"assert_eq!(Fix::from_str("0.00625E+3"), Ok(Fix::from_num(6.25)));
assert_eq!(Fix::from_str("1.25e-1"), Ok(Fix::from_num(0.125)));
```
"#;
                #[inline]
                pub const fn from_str(src: &str) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::from_str_radix(src, 10, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing binary digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. The separator “`e`”, “`E`” or “`@`”
can be used to start an exponent, which is then followed by an optional sign
“`+`” or “`-`”, and then by a decimal integer which is the exponent. The parsed
value is scaled by the radix (2 for binary) to the power of the exponent.

Base-2 exponents are supported too, using the separator “`p`” or “`P`”. The
parsed value is scaled by 2 to the power of the exponent. For binary, since the
radix is 2, base-2 exponents are equivalent to the other form of exponent.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
// 1.11 in binary is 1.75
assert_eq!(Fix::from_str_binary("1.11"), Ok(Fix::from_num(1.75)));
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"assert_eq!(Fix::from_str_binary("-1.11"), Ok(Fix::from_num(-1.75)));
"#,
            },
            r#"
// 111.0101 in binary is 7.3125
assert_eq!(Fix::from_str_binary("1.110101e2"), Ok(Fix::from_num(7.3125)));
assert_eq!(Fix::from_str_binary("11101.01e-2"), Ok(Fix::from_num(7.3125)));
```
"#;
                #[inline]
                pub const fn from_str_binary(src: &str) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::from_str_radix(src, 2, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing octal digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. The separator “`e`”, “`E`” or “`@`”
can be used to start an exponent, which is then followed by an optional sign
“`+`” or “`-`”, and then by a decimal integer which is the exponent. The parsed
value is scaled by 8 to the power of the exponent.

Base-2 exponents are supported too, using the separator “`p`” or “`P`”. The
parsed value is scaled by 2 to the power of the exponent. For example, for octal
“`P6`” means ×2⁶, and is equivalent to “`E2`” which means ×8².

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
// 1.75 is 1.11 in binary, 1.6 in octal
let f = Fix::from_str_octal("1.6");
let check = Fix::from_bits(0b111 << (4 - 2));
assert_eq!(f, Ok(check));
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"let neg = Fix::from_str_octal("-1.6");
assert_eq!(neg, Ok(-check));
"#,
            },
            r#"assert_eq!(Fix::from_str_octal("160e-2"), Ok(check));
```
"#;
                #[inline]
                pub const fn from_str_octal(src: &str) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::from_str_radix(src, 8, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing hexadecimal digits to return a fixed-point number.

Rounding is to the nearest, with ties rounded to even.

The number can have an optional exponent. Since “`e`” and “`E`” are valid
hexadecimal digits, they cannot be used as a separator to start an exponent, so
“`@`” is used instead. This is then followed by an optional sign “`+`” or “`-`”,
and then by a decimal integer which is the exponent. The parsed value is scaled
by 16 to the power of the exponent.

Base-2 exponents are supported too, using the separator “`p`” or “`P`”. The
parsed value is scaled by 2 to the power of the exponent. For example, for
hexadecimal “`P8`” means ×2⁸, and is equivalent to “`@2`” which means ×16².

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
// 1.C in hexadecimal is 1.75
assert_eq!(Fix::from_str_hex("1.C"), Ok(Fix::from_num(1.75)));
"#,
            if_signed_else_empty_str! {
                $Signedness;
                r#"assert_eq!(Fix::from_str_hex("-1.C"), Ok(Fix::from_num(-1.75)));
"#,
            },
            r#"assert_eq!(Fix::from_str_hex("1C@-1"), Ok(Fix::from_num(1.75)));
assert_eq!(Fix::from_str_hex(".01C@+2"), Ok(Fix::from_num(1.75)));
```
"#;
                #[inline]
                pub const fn from_str_hex(src: &str) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::from_str_radix(src, 16, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing decimal digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str("9999"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str("-9999"), Ok(I8F8::MIN));
"#,
                    r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str("9999"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str("-1"), Ok(U8F8::ZERO));
"#,
                ),
                "```
";
                #[inline]
                pub const fn saturating_from_str(
                    src: &str,
                ) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::saturating_from_str_radix(src, 10, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing binary digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str_binary("101100111000"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str_binary("-101100111000"), Ok(I8F8::MIN));
"#,
                    r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str_binary("101100111000"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str_binary("-1"), Ok(U8F8::ZERO));
"#,
                ),
                "```
";
                #[inline]
                pub const fn saturating_from_str_binary(
                    src: &str,
                ) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::saturating_from_str_radix(src, 2, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing octal digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str_octal("7777"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str_octal("-7777"), Ok(I8F8::MIN));
"#,
                    r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str_octal("7777"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str_octal("-1"), Ok(U8F8::ZERO));
"#,
                ),
                "```
";
                #[inline]
                pub const fn saturating_from_str_octal(
                    src: &str,
                ) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::saturating_from_str_radix(src, 8, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Prases a string slice containing hexadecimal digits to return a fixed-point number,
saturating on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
assert_eq!(I8F8::saturating_from_str_hex("FFFF"), Ok(I8F8::MAX));
assert_eq!(I8F8::saturating_from_str_hex("-FFFF"), Ok(I8F8::MIN));
"#,
                    r#"use fixed::types::U8F8;
assert_eq!(U8F8::saturating_from_str_hex("FFFF"), Ok(U8F8::MAX));
assert_eq!(U8F8::saturating_from_str_hex("-1"), Ok(U8F8::ZERO));
"#,
                ),
                "```
";
                #[inline]
                pub const fn saturating_from_str_hex(
                    src: &str,
                ) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::saturating_from_str_radix(src, 16, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing decimal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
// 9999.5 = 15.5 + 256 × n
assert_eq!(I8F8::wrapping_from_str("9999.5"), Ok(I8F8::from_num(15.5)));
assert_eq!(I8F8::wrapping_from_str("-9999.5"), Ok(I8F8::from_num(-15.5)));
"#,
                    r#"use fixed::types::U8F8;
// 9999.5 = 15.5 + 256 × n
assert_eq!(U8F8::wrapping_from_str("9999.5"), Ok(U8F8::from_num(15.5)));
assert_eq!(U8F8::wrapping_from_str("-9999.5"), Ok(U8F8::from_num(240.5)));
"#,
                ),
                "```
";
                #[inline]
                pub const fn wrapping_from_str(src: &str) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::wrapping_from_str_radix(src, 10, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing binary digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(I8F8::wrapping_from_str_binary("101100111000.1"), Ok(check));
assert_eq!(I8F8::wrapping_from_str_binary("-101100111000.1"), Ok(-check));
"#,
                    r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(U8F8::wrapping_from_str_binary("101100111000.1"), Ok(check));
assert_eq!(U8F8::wrapping_from_str_binary("-101100111000.1"), Ok(check.wrapping_neg()));
"#,
                ),
                "```
";
                #[inline]
                pub const fn wrapping_from_str_binary(
                    src: &str,
                ) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::wrapping_from_str_radix(src, 2, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing octal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(I8F8::wrapping_from_str_octal("7165.4"), Ok(check));
assert_eq!(I8F8::wrapping_from_str_octal("-7165.4"), Ok(-check));
"#,
                    r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(U8F8::wrapping_from_str_octal("7165.4"), Ok(check));
assert_eq!(U8F8::wrapping_from_str_octal("-7165.4"), Ok(check.wrapping_neg()));
"#,
                ),
                "```
";
                #[inline]
                pub const fn wrapping_from_str_octal(
                    src: &str,
                ) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::wrapping_from_str_radix(src, 8, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing hexadecimal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0xFFE);
assert_eq!(I8F8::wrapping_from_str_hex("C0F.FE"), Ok(check));
assert_eq!(I8F8::wrapping_from_str_hex("-C0F.FE"), Ok(-check));
"#,
                    r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0xFFE);
assert_eq!(U8F8::wrapping_from_str_hex("C0F.FE"), Ok(check));
assert_eq!(U8F8::wrapping_from_str_hex("-C0F.FE"), Ok(check.wrapping_neg()));
"#,
                ),
                "```
";
                #[inline]
                pub const fn wrapping_from_str_hex(
                    src: &str,
                ) -> Result<$Self<FRAC>, ParseFixedError> {
                    match from_str::$Inner::wrapping_from_str_radix(src, 16, FRAC as u32) {
                        Ok(bits) => Ok($Self::from_bits(bits)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing decimal digits to return a fixed-point number,
panicking on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
// 1.75 is 1.11 in binary
let f = Fix::unwrapped_from_str("1.75");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::"#, stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
let _error = Fix::unwrapped_from_str("1.75.");
```
"#;
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_from_str(src: &str) -> $Self<FRAC> {
                    match $Self::from_str(src) {
                        Ok(o) => o,
                        Err(e) => panic!("{}", e.message()),
                    }
                }
            }

            comment! {
                "Parses a string slice containing binary digits to return a fixed-point number,
panicking on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
// 1.75 is 1.11 in binary
let f = Fix::unwrapped_from_str_binary("1.11");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::"#, stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
let _error = Fix::unwrapped_from_str_binary("1.2");
```
"#;
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_from_str_binary(src: &str) -> $Self<FRAC> {
                    match $Self::from_str_binary(src) {
                        Ok(o) => o,
                        Err(e) => panic!("{}", e.message()),
                    }
                }
            }

            comment! {
                "Parses a string slice containing octal digits to return a fixed-point number,
panicking on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
// 1.75 is 1.11 in binary, 1.6 in octal
let f = Fix::unwrapped_from_str_octal("1.6");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::"#, stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
let _error = Fix::unwrapped_from_str_octal("1.8");
```
"#;
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_from_str_octal(src: &str) -> $Self<FRAC> {
                    match $Self::from_str_octal(src) {
                        Ok(o) => o,
                        Err(e) => panic!("{}", e.message()),
                    }
                }
            }

            comment! {
                "Parses a string slice containing hexadecimal digits to return a fixed-point number,
wrapping on overflow.

Rounding is to the nearest, with ties rounded to even.

# Panics

Panics if the value does not fit or if there is a parsing error.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
// 1.75 is 1.11 in binary, 1.C in hexadecimal
let f = Fix::unwrapped_from_str_hex("1.C");
assert_eq!(f, Fix::from_bits(0b111 << (4 - 2)));
```

The following panics because of a parsing error.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::"#, stringify!($Self), ";
type Fix = ", stringify!($Self), r#"<4>;
let _error = Fix::unwrapped_from_str_hex("1.G");
```
"#;
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_from_str_hex(src: &str) -> $Self<FRAC> {
                    match $Self::from_str_hex(src) {
                        Ok(o) => o,
                        Err(e) => panic!("{}", e.message()),
                    }
                }
            }

            comment! {
                "Parses a string slice containing decimal digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
assert_eq!(I8F8::overflowing_from_str("99.5"), Ok((I8F8::from_num(99.5), false)));
// 9999.5 = 15.5 + 256 × n
assert_eq!(I8F8::overflowing_from_str("-9999.5"), Ok((I8F8::from_num(-15.5), true)));
"#,
                    r#"use fixed::types::U8F8;
assert_eq!(U8F8::overflowing_from_str("99.5"), Ok((U8F8::from_num(99.5), false)));
// 9999.5 = 15.5 + 256 × n
assert_eq!(U8F8::overflowing_from_str("9999.5"), Ok((U8F8::from_num(15.5), true)));
"#,
                ),
                "```
";
                #[inline]
                pub const fn overflowing_from_str(
                    src: &str,
                ) -> Result<($Self<FRAC>, bool), ParseFixedError> {
                    match from_str::$Inner::overflowing_from_str_radix(src, 10, FRAC as u32) {
                        Ok((bits, overflow)) => Ok(($Self::from_bits(bits), overflow)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing binary digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(I8F8::overflowing_from_str_binary("111000.1"), Ok((check, false)));
assert_eq!(I8F8::overflowing_from_str_binary("-101100111000.1"), Ok((-check, true)));
"#,
                    r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0b1110001 << (8 - 1));
assert_eq!(U8F8::overflowing_from_str_binary("111000.1"), Ok((check, false)));
assert_eq!(U8F8::overflowing_from_str_binary("101100111000.1"), Ok((check, true)));
"#,
                ),
                "```
";
                #[inline]
                pub const fn overflowing_from_str_binary(
                    src: &str,
                ) -> Result<($Self<FRAC>, bool), ParseFixedError> {
                    match from_str::$Inner::overflowing_from_str_radix(src, 2, FRAC as u32) {
                        Ok((bits, overflow)) => Ok(($Self::from_bits(bits), overflow)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing octal digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(I8F8::overflowing_from_str_octal("165.4"), Ok((check, false)));
assert_eq!(I8F8::overflowing_from_str_octal("-7165.4"), Ok((-check, true)));
"#,
                    r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0o1654 << (8 - 3));
assert_eq!(U8F8::overflowing_from_str_octal("165.4"), Ok((check, false)));
assert_eq!(U8F8::overflowing_from_str_octal("7165.4"), Ok((check, true)));
"#,
                ),
                "```
";
                #[inline]
                pub const fn overflowing_from_str_octal(
                    src: &str,
                ) -> Result<($Self<FRAC>, bool), ParseFixedError> {
                    match from_str::$Inner::overflowing_from_str_radix(src, 8, FRAC as u32) {
                        Ok((bits, overflow)) => Ok(($Self::from_bits(bits), overflow)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Parses a string slice containing hexadecimal digits to return a fixed-point number.

Returns a [tuple] of the fixed-point number and a [`bool`] indicating
whether an overflow has occurred. On overflow, the wrapped value is
returned.

Rounding is to the nearest, with ties rounded to even.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

",
                if_signed_unsigned!(
                    $Signedness,
                    r#"use fixed::types::I8F8;
let check = I8F8::from_bits(0xFFE);
assert_eq!(I8F8::overflowing_from_str_hex("F.FE"), Ok((check, false)));
assert_eq!(I8F8::overflowing_from_str_hex("-C0F.FE"), Ok((-check, true)));
"#,
                    r#"use fixed::types::U8F8;
let check = U8F8::from_bits(0xFFE);
assert_eq!(U8F8::overflowing_from_str_hex("F.FE"), Ok((check, false)));
assert_eq!(U8F8::overflowing_from_str_hex("C0F.FE"), Ok((check, true)));
"#,
                ),
                "```
";
                #[inline]
                pub const fn overflowing_from_str_hex(
                    src: &str,
                ) -> Result<($Self<FRAC>, bool), ParseFixedError> {
                    match from_str::$Inner::overflowing_from_str_radix(src, 16, FRAC as u32) {
                        Ok((bits, overflow)) => Ok(($Self::from_bits(bits), overflow)),
                        Err(e) => Err(e),
                    }
                }
            }

            comment! {
                "Integer base-10 logarithm, rounded down.

# Panics

Panics if the fixed-point number is ", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), ".

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
assert_eq!(", stringify!($Self), "::<2>::from_num(10).int_log10(), 1);
assert_eq!(", stringify!($Self), "::<2>::from_num(9.75).int_log10(), 0);
assert_eq!(", stringify!($Self), "::<6>::from_num(0.109375).int_log10(), -1);
assert_eq!(", stringify!($Self), "::<6>::from_num(0.09375).int_log10(), -2);
```
";
                #[doc(alias("ilog10"))]
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn int_log10(self) -> i32 {
                    match self.checked_int_log10() {
                        Some(ans) => ans,
                        None => panic!("log of non-positive number"),
                    }
                }
            }

            comment! {
                "Integer logarithm to the specified base, rounded down.

# Panics

Panics if the fixed-point number is ", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), "
or if the base is <&nbsp;2.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(4).int_log(2), 2);
assert_eq!(Fix::from_num(5.75).int_log(5), 1);
assert_eq!(Fix::from_num(0.25).int_log(5), -1);
assert_eq!(Fix::from_num(0.1875).int_log(5), -2);
```
";
                #[doc(alias("ilog"))]
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn int_log(self, base: u32) -> i32 {
                    match self.checked_int_log(base) {
                        Some(s) => s,
                        None => {
                            if base < 2 {
                                panic!("log with base < 2");
                            } else {
                                panic!("log of non-positive number");
                            }
                        }
                    }
                }
            }

            comment! {
                "Checked integer base-10 logarithm, rounded down.
Returns the logarithm or [`None`] if the fixed-point number is
", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), ".

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
assert_eq!(", stringify!($Self), "::<2>::ZERO.checked_int_log10(), None);
assert_eq!(", stringify!($Self), "::<2>::from_num(10).checked_int_log10(), Some(1));
assert_eq!(", stringify!($Self), "::<2>::from_num(9.75).checked_int_log10(), Some(0));
assert_eq!(", stringify!($Self), "::<6>::from_num(0.109375).checked_int_log10(), Some(-1));
assert_eq!(", stringify!($Self), "::<6>::from_num(0.09375).checked_int_log10(), Some(-2));
```
";
                #[inline]
                #[doc(alias("checked_ilog10"))]
                #[must_use]
                pub const fn checked_int_log10(self) -> Option<i32> {
                    if self.to_bits() <= 0 {
                        return None;
                    }
                    // Use unsigned representation because we use all bits in fractional part.
                    let bits = self.to_bits() as $UInner;
                    if Self::FRAC_BITS < $UInner::BITS as i32 {
                        let int = bits >> Self::FRAC_BITS;
                        if let Some(non_zero) = NonZero::<$UInner>::new(int) {
                            return Some(non_zero.ilog10() as i32);
                        }
                    }
                    let frac = bits << Self::INT_BITS;
                    debug_assert!(frac != 0);
                    // SAFETY: at this point, we know that self.to_bits() > 0,
                    // and that the integer bits are zero, so the fractional
                    // bits must be non-zero.
                    let non_zero = unsafe { NonZero::<$UInner>::new_unchecked(frac) };
                    Some(log10::frac_part::$UInner(non_zero))
                }
            }

            comment! {
                "Checked integer logarithm to the specified base, rounded down.
Returns the logarithm, or [`None`] if the fixed-point number is
", if_signed_unsigned!($Signedness, "≤&nbsp;0", "zero"), "
or if the base is <&nbsp;2.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::ZERO.checked_int_log(5), None);
assert_eq!(Fix::from_num(4).checked_int_log(2), Some(2));
assert_eq!(Fix::from_num(5.75).checked_int_log(5), Some(1));
assert_eq!(Fix::from_num(0.25).checked_int_log(5), Some(-1));
assert_eq!(Fix::from_num(0.1875).checked_int_log(5), Some(-2));
```
";
                #[inline]
                #[doc(alias("checked_ilog10"))]
                #[must_use]
                pub const fn checked_int_log(self, base: u32) -> Option<i32> {
                    if self.to_bits() <= 0 {
                        return None;
                    }
                    let Some(base) = Base::new(base) else {
                        return None;
                    };
                    // Use unsigned representation.
                    let bits = self.to_bits() as $UInner;
                    if Self::FRAC_BITS < $UInner::BITS as i32 {
                        let int = bits >> Self::FRAC_BITS;
                        if let Some(non_zero) = NonZero::<$UInner>::new(int) {
                            return Some(log::int_part::$UInner(non_zero, base));
                        }
                    }
                    let frac = bits << Self::INT_BITS;
                    debug_assert!(frac != 0);
                    // SAFETY: at this point, we know that self.to_bits() > 0,
                    // and that the integer bits are zero, so the fractional
                    // bits must be non-zero.
                    let non_zero = unsafe { NonZero::<$UInner>::new_unchecked(frac) };
                    Some(log::frac_part::$UInner(non_zero, base))
                }
            }

            comment! {
                "Returns the reciprocal (inverse) of the fixed-point number, 1/`self`.

# Panics

Panics if the fixed-point number is zero.

When debug assertions are enabled, this method also panics if the
reciprocal overflows. When debug assertions are not enabled, the
wrapped value can be returned, but it is not considered a breaking
change if in the future it panics; if wrapping is required use
[`wrapping_recip`] instead.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(2).recip(), Fix::from_num(0.5));
```

[`wrapping_recip`]: Self::wrapping_recip
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn recip(self) -> $Self<FRAC> {
                    let (ans, overflow) = self.overflowing_recip();
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Euclidean division.

# Panics

Panics if the divisor is zero.

When debug assertions are enabled, this method also panics if the
division overflows. When debug assertions are not enabled, the wrapped
value can be returned, but it is not considered a breaking change if
in the future it panics; if wrapping is required use
[`wrapping_div_euclid`] instead.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).div_euclid(Fix::from_num(2)), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).div_euclid(Fix::from_num(2)), Fix::from_num(-4));
",
                },
                "```

[`wrapping_div_euclid`]: Self::wrapping_div_euclid
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn div_euclid(self, rhs: $Self<FRAC>) -> $Self<FRAC> {
                    let (ans, overflow) = self.overflowing_div_euclid(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Euclidean division by an integer.

# Panics

Panics if the divisor is zero.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "When debug assertions are enabled, this method
also panics if the division overflows. Overflow can only occur when
dividing the minimum value by &minus;1. When debug assertions are not
enabled, the wrapped value can be returned, but it is not considered a
breaking change if in the future it panics; if wrapping is required
use [`wrapping_div_euclid_int`] instead.
",
                },
                "# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).div_euclid_int(2), Fix::from_num(-4));
",
                },
                "```

[`wrapping_div_euclid_int`]: Self::wrapping_div_euclid_int
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn div_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    let (ans, overflow) = self.overflowing_div_euclid_int(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Remainder for Euclidean division by an integer.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).rem_euclid_int(2), Fix::from_num(0.5));
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn rem_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    let (ans, overflow) = self.overflowing_rem_euclid_int(rhs);
                    debug_assert!(!overflow, "overflow");
                    ans
                }
            }

            comment! {
                "Returns the square root.

This method uses an iterative method, with up to ", $n, " iterations for
[`", stringify!($Self), "`]. The result is rounded down, and the error is
&lt;&nbsp;[`DELTA`][Self::DELTA]. That is,
result&nbsp;≤&nbsp;√`self`&nbsp;&lt;&nbsp;result&nbsp;+&nbsp;`DELTA`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "Overflow can only occur when there are no integer bits and
the representable range is &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;&lt;&nbsp;0.5.
In this case, overflow occurs for an input value ≥&nbsp;0.25.

"
                },
                if_signed_else_empty_str! {
                    $Signedness;
                    "# Panics

Panics if the number is negative.

When debug assertions are enabled, this method also panics if the square root
overflows. When debug assertions are not enabled, the wrapped value can be
returned, but it is not considered a breaking change if in the future it panics;
if wrapping is required use [`wrapping_sqrt`] instead.

"
                },
                "# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(2).sqrt(), Fix::SQRT_2);
```

[`wrapping_sqrt`]: Self::wrapping_sqrt
";
                #[inline]
                #[must_use]
                pub const fn sqrt(self) -> Self {
                    let (val, overflow) = self.overflowing_sqrt();
                    debug_assert!(!overflow, "overflow");
                    val
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`.

Returns
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Panics

When debug assertions are enabled, this method panics if the result overflows.
When debug assertions are not enabled, the wrapped value can be returned, but it
is not considered a breaking change if in the future it panics; if wrapping is
required use [`wrapping_lerp`] instead.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let start = Fix::from_num(2);
let end = Fix::from_num(3.5);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-1.0).lerp(start, end), 0.5);
",
                },
                "assert_eq!(Fix::from_num(0.0).lerp(start, end), 2);
assert_eq!(Fix::from_num(0.5).lerp(start, end), 2.75);
assert_eq!(Fix::from_num(1.0).lerp(start, end), 3.5);
assert_eq!(Fix::from_num(2.0).lerp(start, end), 5);
```

[`wrapping_lerp`]: Self::wrapping_lerp
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn lerp<const RANGE_FRAC: i32>(
                    self,
                    start: $Self<RANGE_FRAC>,
                    end: $Self<RANGE_FRAC>,
                ) -> $Self<RANGE_FRAC> {
                    let (ans, overflow) = lerp::$Inner(
                        self.to_bits(),
                        start.to_bits(),
                        end.to_bits(),
                        FRAC as u32,
                    );
                    debug_assert!(!overflow, "overflow");
                    $Self::from_bits(ans)
                }
            }

            comment! {
                "Checked division. Returns the quotient, or [`None`] if
the divisor is zero or on overflow.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::MAX.checked_div(Fix::ONE), Some(Fix::MAX));
assert_eq!(Fix::MAX.checked_div(Fix::ONE / 2), None);
```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_div(self, rhs: $Self<FRAC>) -> Option<$Self<FRAC>> {
                    if rhs.to_bits() == 0 {
                        return None;
                    }
                    match arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), FRAC as u32)
                    {
                        (ans, false) => Some(Self::from_bits(ans)),
                        (_, true) => None,
                    }
                }
            }

            comment! {
                "Checked reciprocal. Returns the reciprocal, or
[`None`] if `self` is zero or on overflow.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(2).checked_recip(), Some(Fix::from_num(0.5)));
assert_eq!(Fix::ZERO.checked_recip(), None);
```
";
                #[inline]
                #[must_use]
                pub const fn checked_recip(self) -> Option<$Self<FRAC>> {
                    if self.to_bits() == 0 {
                        None
                    } else {
                        match self.overflowing_recip() {
                            (ans, false) => Some(ans),
                            (_, true) => None,
                        }
                    }
                }
            }

            comment! {
                "Checked Euclidean division. Returns the quotient, or
[`None`] if the divisor is zero or on overflow.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).checked_div_euclid(Fix::from_num(2)), Some(Fix::from_num(3)));
assert_eq!(Fix::from_num(7.5).checked_div_euclid(Fix::ZERO), None);
assert_eq!(Fix::MAX.checked_div_euclid(Fix::from_num(0.25)), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).checked_div_euclid(Fix::from_num(2)), Some(Fix::from_num(-4)));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_div_euclid(self, rhs: $Self<FRAC>) -> Option<$Self<FRAC>> {
                    let q = match self.checked_div(rhs) {
                        Some(s) => s.round_to_zero(),
                        None => return None,
                    };
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem(rhs).is_negative() {
                            let Some(neg_one) = Self::TRY_NEG_ONE else {
                                return None;
                            };
                            return if rhs.is_positive() {
                                q.checked_add(neg_one)
                            } else {
                                q.checked_sub(neg_one)
                            };
                        }
                    }
                    Some(q)
                }
            }

            comment! {
                "Checked fixed-point remainder for division by an integer.
Returns the remainder, or [`None`] if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(3.75).checked_rem_int(2), Some(Fix::from_num(1.75)));
assert_eq!(Fix::from_num(3.75).checked_rem_int(0), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-3.75).checked_rem_int(2), Some(Fix::from_num(-1.75)));
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_rem_int(self, rhs: $Inner) -> Option<$Self<FRAC>> {
                    // Overflow converting rhs to $Self<FRAC> means that either
                    //   * |rhs| > |self|, and so remainder is self, or
                    //   * self is signed min with at least one integer bit,
                    //     and the value of rhs is -self, so remainder is 0.
                    if rhs == 0 {
                        return None;
                    }
                    let can_shift = if_signed_unsigned!(
                        $Signedness,
                        if rhs.is_negative() {
                            rhs.leading_ones() - 1
                        } else {
                            rhs.leading_zeros() - 1
                        },
                        rhs.leading_zeros(),
                    );
                    if Self::FRAC_BITS as u32 <= can_shift {
                        let fixed_rhs = Self::from_bits(rhs << Self::FRAC_BITS);
                        self.checked_rem(fixed_rhs)
                    } else {
                        if_signed_unsigned!(
                            $Signedness,
                            if self.to_bits() == $Inner::MIN
                                && (Self::INT_BITS > 0 && rhs == 1 << (Self::INT_BITS - 1))
                            {
                                Some(Self::ZERO)
                            } else {
                                Some(self)
                            },
                            Some(self),
                        )
                    }
                }
            }

            comment! {
                "Checked Euclidean division by an integer. Returns the
quotient, or [`None`] if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the division results in overflow",
                },
                ".

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).checked_div_euclid_int(2), Some(Fix::from_num(3)));
assert_eq!(Fix::from_num(7.5).checked_div_euclid_int(0), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::MIN.checked_div_euclid_int(-1), None);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_div_euclid_int(self, rhs: $Inner) -> Option<$Self<FRAC>> {
                    let q = match self.checked_div_int(rhs) {
                        Some(s) => s.round_to_zero(),
                        None => return None,
                    };
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem_int(rhs).is_negative() {
                            let Some(neg_one) = Self::TRY_NEG_ONE else {
                                return None;
                            };
                            return if rhs.is_positive() {
                                q.checked_add(neg_one)
                            } else {
                                q.checked_sub(neg_one)
                            };
                        }
                    }
                    Some(q)
                }
            }

            comment! {
                "Checked remainder for Euclidean division by an integer.
Returns the remainder, or [`None`] if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the remainder results in overflow",
                },
                ".

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<", $nm4, ">;
assert_eq!(Fix::from_num(7.5).checked_rem_euclid_int(2), Some(Fix::from_num(1.5)));
assert_eq!(Fix::from_num(7.5).checked_rem_euclid_int(0), None);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).checked_rem_euclid_int(2), Some(Fix::from_num(0.5)));
// -8 ≤ Fix < 8, so the answer 12.5 overflows
assert_eq!(Fix::from_num(-7.5).checked_rem_euclid_int(20), None);
",
                },
                "```
";
                #[inline]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn checked_rem_euclid_int(self, rhs: $Inner) -> Option<$Self<FRAC>> {
                    if_signed! {
                        $Signedness;
                        let Some(rem) = self.checked_rem_int(rhs) else {
                            return None;
                        };
                        if !rem.is_negative() {
                            return Some(rem);
                        }
                        // Work in unsigned.
                        // Answer required is |rhs| - |rem|, but rhs is int, rem is fixed.
                        // INT_BITS == 0 is a special case, as fraction can be negative.
                        if Self::INT_BITS == 0 {
                            // -0.5 <= rem < 0, so euclidean remainder is in the range
                            // 0.5 <= answer < 1, which does not fit.
                            return None;
                        }
                        let rhs_abs = rhs.wrapping_abs() as $UInner;
                        let remb = rem.to_bits();
                        let remb_abs = remb.wrapping_neg() as $UInner;
                        let rem_int_abs = remb_abs >> Self::FRAC_BITS;
                        let rem_frac = remb & Self::FRAC_MASK;
                        let ans_int = rhs_abs - rem_int_abs - if rem_frac > 0 { 1 } else { 0 };
                        let ansb_abs = if ans_int == 0 {
                            0
                        } else if Self::FRAC_BITS as u32 <= ans_int.leading_zeros() {
                            ans_int << Self::FRAC_BITS
                        } else {
                            return None
                        };
                        let ansb = ansb_abs as $Inner;
                        if ansb.is_negative() {
                            return None;
                        }
                        Some(Self::from_bits(ansb | rem_frac))
                    }
                    if_unsigned! {
                        $Signedness;
                        self.checked_rem_int(rhs)
                    }
                }
            }

            comment! {
                "Checked square root. ",
                if_signed_unsigned!(
                    $Signedness,
                    "Returns [`None`] for negative numbers and on overflow.",
                    "Always returns the square root for unsigned numbers."
                ),
                "

This method uses an iterative method, with up to ", $n, " iterations for
 [`", stringify!($Self), "`]. The result is rounded down, and the error is
&lt;&nbsp;[`DELTA`][Self::DELTA]. That is,
result&nbsp;≤&nbsp;√`self`&nbsp;&lt;&nbsp;result&nbsp;+&nbsp;`DELTA`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "Overflow can only occur when there are no integer bits and
the representable range is &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;&lt;&nbsp;0.5.
In this case, the method returns [`None`] for an input value ≥&nbsp;0.25.

"
                },
                "# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(2).checked_sqrt(), Some(Fix::SQRT_2));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-1).checked_sqrt(), None);

type AllFrac = ", stringify!($Self), "<", $n, ">;
assert_eq!(AllFrac::from_num(0.25).checked_sqrt(), None);
",
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn checked_sqrt(self) -> Option<Self> {
                    if_signed! {
                        $Signedness;
                        if self.is_negative() {
                            return None;
                        }
                    }
                    match self.overflowing_sqrt() {
                        (val, false) => Some(val),
                        (_, true) => None,
                    }
                }
            }

            comment! {
                "Checked linear interpolation between `start` and `end`. Returns
[`None`] on overflow.

The interpolted value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(0.5).checked_lerp(Fix::ZERO, Fix::MAX), Some(Fix::MAX / 2));
assert_eq!(Fix::from_num(1.5).checked_lerp(Fix::ZERO, Fix::MAX), None);
```
";
                #[inline]
                #[must_use]
                pub const fn checked_lerp<const RANGE_FRAC: i32>(
                    self,
                    start: $Self<RANGE_FRAC>,
                    end: $Self<RANGE_FRAC>,
                ) -> Option<$Self<RANGE_FRAC>> {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), FRAC as u32) {
                        (bits, false) => Some($Self::from_bits(bits)),
                        (_, true) => None,
                    }
                }
            }

            comment! {
                "Saturating division. Returns the quotient, saturating on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let one_half = Fix::ONE / 2;
assert_eq!(Fix::ONE.saturating_div(Fix::from_num(2)), one_half);
assert_eq!(Fix::MAX.saturating_div(one_half), Fix::MAX);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_div(self, rhs: $Self<FRAC>) -> $Self<FRAC> {
                    match arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), FRAC as u32)
                    {
                        (ans, false) => Self::from_bits(ans),
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if self.is_negative() != rhs.is_negative() {
                                    Self::MIN
                                } else {
                                    Self::MAX
                                },
                                Self::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Saturating reciprocal. Returns the reciprocal,
saturating on overflow.

# Panics

Panics if the fixed-point number is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
// only one integer bit
type Fix = ", stringify!($Self), "<", $nm1, ">;
assert_eq!(Fix::from_num(0.25).saturating_recip(), Fix::MAX);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-0.25).saturating_recip(), Fix::MIN);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn saturating_recip(self) -> $Self<FRAC> {
                    match self.overflowing_recip() {
                        (ans, false) => ans,
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if self.is_negative() {
                                    Self::MIN
                                } else {
                                    Self::MAX
                                },
                                Self::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Saturating Euclidean division. Returns the quotient,
saturating on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).saturating_div_euclid(Fix::from_num(2)), Fix::from_num(3));
assert_eq!(Fix::MAX.saturating_div_euclid(Fix::from_num(0.25)), Fix::MAX);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).saturating_div_euclid(Fix::from_num(2)), Fix::from_num(-4));
assert_eq!(Fix::MIN.saturating_div_euclid(Fix::from_num(0.25)), Fix::MIN);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_div_euclid(self, rhs: $Self<FRAC>) -> $Self<FRAC> {
                    match self.overflowing_div_euclid(rhs) {
                        (val, false) => val,
                        (_, true) => {
                            if_signed_unsigned!(
                                $Signedness,
                                if self.is_negative() != rhs.is_negative() {
                                    Self::MIN
                                } else {
                                    Self::MAX
                                },
                                Self::MAX,
                            )
                        }
                    }
                }
            }

            comment! {
                "Saturating Euclidean division by an integer. Returns the quotient",
                if_signed_unsigned!(
                    $Signedness,
                    ", saturating on overflow.

Overflow can only occur when dividing the minimum value by &minus;1.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).saturating_div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).saturating_div_euclid_int(2), Fix::from_num(-4));
assert_eq!(Fix::MIN.saturating_div_euclid_int(-1), Fix::MAX);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_div_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    // dividing by integer can never result in something < MIN
                    match self.overflowing_div_euclid_int(rhs) {
                        (val, false) => val,
                        (_, true) => $Self::MAX,
                    }
                }
            }

            comment! {
                "Saturating remainder for Euclidean division by an integer. Returns the remainder",
                if_signed_unsigned!(
                    $Signedness,
                    ", saturating on overflow.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<", $nm4, ">;
assert_eq!(Fix::from_num(7.5).saturating_rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).saturating_rem_euclid_int(2), Fix::from_num(0.5));
// -8 ≤ Fix < 8, so the answer 12.5 saturates
assert_eq!(Fix::from_num(-7.5).saturating_rem_euclid_int(20), Fix::MAX);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn saturating_rem_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    match self.overflowing_rem_euclid_int(rhs) {
                        (val, false) => val,
                        (_, true) => $Self::MAX,
                    }
                }
            }

            comment! {
                "Returns the square root, saturating on overflow.",
                if_unsigned_else_empty_str! {
                    $Signedness;
                    " Can never overflow for unsigned numbers."
                },
                "

This method uses an iterative method, with up to ", $n, " iterations for
 [`", stringify!($Self), "`]. The result is rounded down, and the error is
&lt;&nbsp;[`DELTA`][Self::DELTA]. That is,
result&nbsp;≤&nbsp;√`self`&nbsp;&lt;&nbsp;result&nbsp;+&nbsp;`DELTA`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "Overflow can only occur when there are no integer bits and
the representable range is &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;&lt;&nbsp;0.5.
In this case, the method returns [`MAX`][Self::MAX] for an input value ≥&nbsp;0.25.

"
                },
                if_signed_else_empty_str! {
                    $Signedness;
                    "# Panics

Panics if the number is negative.

"
                },
                "# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(2).saturating_sqrt(), Fix::SQRT_2);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
type AllFrac = ", stringify!($Self), "<", $n, ">;
assert_eq!(AllFrac::from_num(0.25).saturating_sqrt(), AllFrac::MAX);
",
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn saturating_sqrt(self) -> Self {
                    match self.overflowing_sqrt() {
                        (val, false) => val,
                        (_, true) => Self::MAX,
                    }
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`, saturating on
overflow.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(0.5).saturating_lerp(Fix::ZERO, Fix::MAX), Fix::MAX / 2);
assert_eq!(Fix::from_num(1.5).saturating_lerp(Fix::ZERO, Fix::MAX), Fix::MAX);
",
                if_signed_unsigned! (
                    $Signedness,
                    "assert_eq!(Fix::from_num(-2.0).saturating_lerp(Fix::ZERO, Fix::MAX), Fix::MIN);
assert_eq!(Fix::from_num(3.0).saturating_lerp(Fix::MAX, Fix::ZERO), Fix::MIN);
",
                    "assert_eq!(Fix::from_num(3.0).saturating_lerp(Fix::MAX, Fix::ZERO), Fix::ZERO);
",
                ),
                "```
";
                #[inline]
                #[must_use]
                pub const fn saturating_lerp<const RANGE_FRAC: i32>(
                    self,
                    start: $Self<RANGE_FRAC>,
                    end: $Self<RANGE_FRAC>,
                ) -> $Self<RANGE_FRAC> {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), FRAC as u32) {
                        (bits, false) => $Self::from_bits(bits),
                        (_, true) => if_signed_unsigned!(
                            $Signedness,
                            if self.is_negative() == (end.to_bits() < start.to_bits()) {
                                $Self::MAX
                            } else {
                                $Self::MIN
                            },
                            if end.to_bits() < start.to_bits() {
                                $Self::MIN
                            } else {
                                $Self::MAX
                            },
                        ),
                    }
                }
            }

            comment! {
                "Wrapping division. Returns the quotient, wrapping on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).wrapping_div(Fix::from_num(2)), one_point_5);
let quarter = Fix::ONE / 4;
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.wrapping_div(quarter), wrapped);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_div(self, rhs: $Self<FRAC>) -> $Self<FRAC> {
                    let (ans, _) =
                        arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), FRAC as u32);
                    Self::from_bits(ans)
                }
            }

            comment! {
                "Wrapping reciprocal. Returns the reciprocal,
wrapping on overflow.

# Panics

Panics if the fixed-point number is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
// only one integer bit
type Fix = ", stringify!($Self), "<", $nm1, ">;
assert_eq!(Fix::from_num(0.25).wrapping_recip(), Fix::ZERO);
```
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn wrapping_recip(self) -> $Self<FRAC> {
                    let (ans, _) = self.overflowing_recip();
                    ans
                }
            }

            comment! {
                "Wrapping Euclidean division. Returns the quotient, wrapping on overflow.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).wrapping_div_euclid(Fix::from_num(2)), Fix::from_num(3));
let wrapped = Fix::MAX.wrapping_mul_int(4).round_to_zero();
assert_eq!(Fix::MAX.wrapping_div_euclid(Fix::from_num(0.25)), wrapped);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_div_euclid(self, rhs: $Self<FRAC>) -> $Self<FRAC> {
                    self.overflowing_div_euclid(rhs).0
                }
            }

            comment! {
                "Wrapping Euclidean division by an integer. Returns the quotient",
                if_signed_unsigned!(
                    $Signedness,
                    ", wrapping on overflow.

Overflow can only occur when dividing the minimum value by &minus;1.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).wrapping_div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).wrapping_div_euclid_int(2), Fix::from_num(-4));
let wrapped = Fix::MIN.round_to_zero();
assert_eq!(Fix::MIN.wrapping_div_euclid_int(-1), wrapped);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_div_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    self.overflowing_div_euclid_int(rhs).0
                }
            }

            comment! {
                "Wrapping remainder for Euclidean division by an integer. Returns the remainder",
                if_signed_unsigned!(
                    $Signedness,
                    ", wrapping on overflow.

Note that while remainder for Euclidean division cannot be negative,
the wrapped value can be negative.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<", $nm4, ">;
assert_eq!(Fix::from_num(7.5).wrapping_rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).wrapping_rem_euclid_int(2), Fix::from_num(0.5));
// -8 ≤ Fix < 8, so the answer 12.5 wraps to -3.5
assert_eq!(Fix::from_num(-7.5).wrapping_rem_euclid_int(20), Fix::from_num(-3.5));
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn wrapping_rem_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    self.overflowing_rem_euclid_int(rhs).0
                }
            }

            comment! {
                "Returns the square root, wrapping on overflow.",
                if_unsigned_else_empty_str! {
                    $Signedness;
                    " Can never overflow for unsigned numbers."
                },
                "

This method uses an iterative method, with up to ", $n, " iterations for
 [`", stringify!($Self), "`]. The result is rounded down, and the error is
&lt;&nbsp;[`DELTA`][Self::DELTA]. That is,
result&nbsp;≤&nbsp;√`self`&nbsp;&lt;&nbsp;result&nbsp;+&nbsp;`DELTA`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "Overflow can only occur when there are no integer bits and
the representable range is &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;&lt;&nbsp;0.5.
In this case, the method returns the wrapped answer for an input value ≥&nbsp;0.25.

"
                },
                if_signed_else_empty_str! {
                    $Signedness;
                    "# Panics

Panics if the number is negative.

"
                },
                "# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(2).wrapping_sqrt(), Fix::SQRT_2);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
type AllFrac = ", stringify!($Self), "<", $n, ">;
assert_eq!(AllFrac::from_num(0.25).wrapping_sqrt(), AllFrac::from_num(-0.5));
",
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn wrapping_sqrt(self) -> Self {
                    self.overflowing_sqrt().0
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`, wrapping on
overflow.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(0.5).wrapping_lerp(Fix::ZERO, Fix::MAX), Fix::MAX / 2);
assert_eq!(
    Fix::from_num(1.5).wrapping_lerp(Fix::ZERO, Fix::MAX),
    Fix::MAX.wrapping_add(Fix::MAX / 2)
);
```
";
                #[inline]
                #[must_use]
                pub const fn wrapping_lerp<const RANGE_FRAC: i32>(
                    self,
                    start: $Self<RANGE_FRAC>,
                    end: $Self<RANGE_FRAC>,
                ) -> $Self<RANGE_FRAC> {
                    let (bits, _) =
                        lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), FRAC as u32);
                    $Self::from_bits(bits)
                }
            }

            comment! {
                "Unwrapped division. Returns the quotient, panicking on overflow.

# Panics

Panics if the divisor is zero or if the division results in overflow.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).unwrapped_div(Fix::from_num(2)), one_point_5);
```

The following panics because of overflow.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let quarter = Fix::ONE / 4;
let _overflow = Fix::MAX.unwrapped_div(quarter);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_div(self, rhs: $Self<FRAC>) -> $Self<FRAC> {
                    match self.overflowing_div(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Unwrapped reciprocal. Returns the reciprocal,
panicking on overflow.

# Panics

Panics if the fixed-point number is zero or on overflow.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(0.25).unwrapped_recip(), Fix::from_num(4));
```
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_recip(self) -> $Self<FRAC> {
                    match self.overflowing_recip() {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Unwrapped Euclidean division. Returns the quotient, panicking on overflow.

# Panics

Panics if the divisor is zero or if the division results in overflow.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).unwrapped_div_euclid(Fix::from_num(2)), Fix::from_num(3));
```

The following panics because of overflow.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let _overflow = Fix::MAX.unwrapped_div_euclid(Fix::from_num(0.25));
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_div_euclid(self, rhs: $Self<FRAC>) -> $Self<FRAC> {
                    match self.overflowing_div_euclid(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Unwrapped fixed-point remainder for division by an integer.
Returns the remainder, panicking if the divisor is zero.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(3.75).unwrapped_rem_int(2), Fix::from_num(1.75));
```

The following panics because the divisor is zero.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let _divisor_is_zero = Fix::from_num(3.75).unwrapped_rem_int(0);
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_rem_int(self, rhs: $Inner) -> $Self<FRAC> {
                    match self.checked_rem_int(rhs) {
                        Some(ans) => ans,
                        None => panic!("division by zero"),
                    }
                }
            }

            comment! {
                "Unwrapped Euclidean division by an integer. Returns the quotient",
                if_signed_unsigned!(
                    $Signedness,
                    ", panicking on overflow.

Overflow can only occur when dividing the minimum value by &minus;1.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the division results in overflow",
                },
                ".

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).unwrapped_div_euclid_int(2), Fix::from_num(3));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).unwrapped_div_euclid_int(2), Fix::from_num(-4));
```

The following panics because of overflow.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let _overflow = Fix::MIN.unwrapped_div_euclid_int(-1);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_div_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    match self.overflowing_div_euclid_int(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Unwrapped remainder for Euclidean division by an integer. Returns the remainder",
                if_signed_unsigned!(
                    $Signedness,
                    ", panicking on overflow.

Note that while remainder for Euclidean division cannot be negative,
the wrapped value can be negative.",
                    ".

Can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero",
                if_signed_else_empty_str! {
                    $Signedness;
                    " or if the division results in overflow",
                },
                ".

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<", $nm4, ">;
assert_eq!(Fix::from_num(7.5).unwrapped_rem_euclid_int(2), Fix::from_num(1.5));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).unwrapped_rem_euclid_int(2), Fix::from_num(0.5));
```

The following panics because of overflow.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<", $nm4, ">;
// -8 ≤ Fix < 8, so the answer 12.5 overflows
let _overflow = Fix::from_num(-7.5).unwrapped_rem_euclid_int(20);
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn unwrapped_rem_euclid_int(self, rhs: $Inner) -> $Self<FRAC> {
                    match self.overflowing_rem_euclid_int(rhs) {
                        (_, true) => panic!("overflow"),
                        (ans, false) => ans,
                    }
                }
            }

            comment! {
                "Returns the square root",
                if_signed_unsigned!(
                    $Signedness,
                    ", panicking for negative numbers and on overflow.",
                    ". Can never overflow for unsigned numbers.",
                ),
                "

This method uses an iterative method, with up to ", $n, " iterations for
[`", stringify!($Self), "`]. The result is rounded down, and the error is
&lt;&nbsp;[`DELTA`][Self::DELTA]. That is,
result&nbsp;≤&nbsp;√`self`&nbsp;&lt;&nbsp;result&nbsp;+&nbsp;`DELTA`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "Overflow can only occur when there are no integer bits and
the representable range is &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;&lt;&nbsp;0.5.
In this case, the method panics for an input value ≥&nbsp;0.25.

"
                },
                if_signed_else_empty_str! {
                    $Signedness;
                    "# Panics

Panics if the number is negative and on overflow.

"
                },
                "# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(2).unwrapped_sqrt(), Fix::SQRT_2);
```
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
The following panics because the input value is negative.

```should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let _sqrt_neg = Fix::from_num(-1).unwrapped_sqrt();
```

The following panics because of overflow.

```should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type AllFrac = ", stringify!($Self), "<", $n, ">;
let _overflow = AllFrac::from_num(0.25).unwrapped_sqrt();
```
",
                };
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_sqrt(self) -> Self {
                    match self.overflowing_sqrt() {
                        (val, false) => val,
                        (_, true) => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Linear interpolation between `start` and `end`, panicking on
overflow.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Panics

Panics if the result overflows.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(0.5).unwrapped_lerp(Fix::ZERO, Fix::MAX), Fix::MAX / 2);
```

The following panics because of overflow.

```rust,should_panic
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let _overflow = Fix::from_num(1.5).unwrapped_lerp(Fix::ZERO, Fix::MAX);
```
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn unwrapped_lerp<const RANGE_FRAC: i32>(
                    self,
                    start: $Self<RANGE_FRAC>,
                    end: $Self<RANGE_FRAC>,
                ) -> $Self<RANGE_FRAC> {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), FRAC as u32) {
                        (bits, false) => $Self::from_bits(bits),
                        (_, true) => panic!("overflow"),
                    }
                }
            }

            comment! {
                "Overflowing division.

Returns a [tuple] of the quotient and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let one_point_5 = Fix::from_bits(0b11 << (4 - 1));
assert_eq!(Fix::from_num(3).overflowing_div(Fix::from_num(2)), (one_point_5, false));
let quarter = Fix::ONE / 4;
let wrapped = Fix::from_bits(!0 << 2);
assert_eq!(Fix::MAX.overflowing_div(quarter), (wrapped, true));
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_div(self, rhs: $Self<FRAC>) -> ($Self<FRAC>, bool) {
                    let (ans, overflow) =
                        arith::$Inner::overflowing_div(self.to_bits(), rhs.to_bits(), FRAC as u32);
                    (Self::from_bits(ans), overflow)
                }
            }

            comment! {
                "Overflowing reciprocal.

Returns a [tuple] of the reciprocal and a [`bool`] indicating whether
an overflow has occurred. On overflow, the wrapped value is returned.

# Panics

Panics if the fixed-point number is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
// only one integer bit
type Small = ", stringify!($Self), "<", $nm1, ">;
assert_eq!(Fix::from_num(0.25).overflowing_recip(), (Fix::from_num(4), false));
assert_eq!(Small::from_num(0.25).overflowing_recip(), (Small::ZERO, true));
```
";
                #[inline]
                #[track_caller]
                #[must_use]
                pub const fn overflowing_recip(self) -> ($Self<FRAC>, bool) {
                    if let Some(one) = Self::TRY_ONE {
                        return one.overflowing_div(self);
                    }
                    if_signed! {
                        $Signedness;
                        let (neg, abs) = int_helper::$Inner::neg_abs(self.to_bits());
                        let uns_abs = $USelf::<FRAC>::from_bits(abs);
                        let (uns_wrapped, overflow1) = uns_abs.overflowing_recip();
                        let wrapped = $Self::<FRAC>::from_bits(uns_wrapped.to_bits() as $Inner);
                        let overflow2 = wrapped.is_negative();
                        if wrapped.to_bits() == $Inner::MIN && neg {
                            return (wrapped, overflow1);
                        }
                        if neg {
                            // if we do not have overflow yet, we will not overflow now
                            (wrapped.wrapping_neg(), overflow1 | overflow2)
                        } else {
                            (wrapped, overflow1 | overflow2)
                        }
                    }
                    if_unsigned! {
                        $Signedness;
                        // 0 < x < 1: 1/x = 1 + (1 - x) / x, wrapped to (1 - x) / x
                        // x.wrapping_neg() = 1 - x

                        // x = 0: we still get division by zero

                        (self.wrapping_neg().wrapping_div(self), true)
                    }
                }
            }

            comment! {
                "Overflowing Euclidean division.

Returns a [tuple] of the quotient and a [`bool`] indicating whether an
overflow has occurred. On overflow, the wrapped value is returned.

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
let check = Fix::from_num(3);
assert_eq!(Fix::from_num(7.5).overflowing_div_euclid(Fix::from_num(2)), (check, false));
let wrapped = Fix::MAX.wrapping_mul_int(4).round_to_zero();
assert_eq!(Fix::MAX.overflowing_div_euclid(Fix::from_num(0.25)), (wrapped, true));
```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_div_euclid(
                    self,
                    rhs: $Self<FRAC>,
                ) -> ($Self<FRAC>, bool) {
                    let (mut q, overflow) = self.overflowing_div(rhs);
                    q = q.round_to_zero();
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem(rhs).is_negative() {
                            let Some(neg_one) = Self::TRY_NEG_ONE else {
                                return (q, true);
                            };
                            let (q, overflow2) = if rhs.is_positive() {
                                q.overflowing_add(neg_one)
                            } else {
                                q.overflowing_sub(neg_one)
                            };
                            return (q, overflow | overflow2);
                        }
                    }
                    (q, overflow)
                }
            }

            comment! {
                "Overflowing Euclidean division by an integer.

Returns a [tuple] of the quotient and ",
                if_signed_unsigned!(
                    $Signedness,
                    "a [`bool`] indicating whether an overflow has
occurred. On overflow, the wrapped value is returned. Overflow can
only occur when dividing the minimum value by &minus;1.",
                    "[`false`], as the division can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(Fix::from_num(7.5).overflowing_div_euclid_int(2), (Fix::from_num(3), false));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).overflowing_div_euclid_int(2), (Fix::from_num(-4), false));
let wrapped = Fix::MIN.round_to_zero();
assert_eq!(Fix::MIN.overflowing_div_euclid_int(-1), (wrapped, true));
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_div_euclid_int(self, rhs: $Inner) -> ($Self<FRAC>, bool) {
                    let (mut q, overflow) = self.overflowing_div_int(rhs);
                    q = q.round_to_zero();
                    if_signed! {
                        $Signedness;
                        if self.unwrapped_rem_int(rhs).is_negative() {
                            let Some(neg_one) = Self::TRY_NEG_ONE else {
                                return (q, true);
                            };
                            let (q, overflow2) = if rhs.is_positive() {
                                q.overflowing_add(neg_one)
                            } else {
                                q.overflowing_sub(neg_one)
                            };
                            return (q, overflow | overflow2);
                        }
                    }
                    (q, overflow)
                }
            }

            comment! {
                "Remainder for Euclidean division by an integer.

Returns a [tuple] of the remainder and ",
                if_signed_unsigned!(
                    $Signedness,
                    "a [`bool`] indicating whether an overflow has
occurred. On overflow, the wrapped value is returned.

Note that while remainder for Euclidean division cannot be negative,
the wrapped value can be negative.",
                    "[`false`], as this can never overflow for unsigned values.",
                ),
                "

# Panics

Panics if the divisor is zero.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<", $nm4, ">;
assert_eq!(Fix::from_num(7.5).overflowing_rem_euclid_int(2), (Fix::from_num(1.5), false));
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "assert_eq!(Fix::from_num(-7.5).overflowing_rem_euclid_int(2), (Fix::from_num(0.5), false));
// -8 ≤ Fix < 8, so the answer 12.5 wraps to -3.5
assert_eq!(Fix::from_num(-7.5).overflowing_rem_euclid_int(20), (Fix::from_num(-3.5), true));
",
                },
                "```
";
                #[inline]
                #[track_caller]
                #[must_use = "this returns the result of the operation, without modifying the original"]
                pub const fn overflowing_rem_euclid_int(self, rhs: $Inner) -> ($Self<FRAC>, bool) {
                    if_signed! {
                        $Signedness;
                        let rem = self.unwrapped_rem_int(rhs);
                        if !rem.is_negative() {
                            return (rem, false);
                        }
                        // Work in unsigned.
                        // Answer required is |rhs| - |rem|, but rhs is int, rem is fixed.
                        // INT_BITS == 0 is a special case, as fraction can be negative.
                        if Self::INT_BITS == 0 {
                            // -0.5 <= rem < 0, so euclidean remainder is in the range
                            // 0.5 <= answer < 1, which does not fit.
                            return (rem, true);
                        }
                        let rhs_abs = rhs.wrapping_abs() as $UInner;
                        let remb = rem.to_bits();
                        let remb_abs = remb.wrapping_neg() as $UInner;
                        let rem_int_abs = remb_abs >> Self::FRAC_BITS;
                        let rem_frac = remb & Self::FRAC_MASK;
                        let ans_int = rhs_abs - rem_int_abs - if rem_frac > 0 { 1 } else { 0 };
                        let (ansb_abs, overflow1) = if ans_int == 0 {
                            (0, false)
                        } else if Self::FRAC_BITS as u32 <= ans_int.leading_zeros() {
                            (ans_int << Self::FRAC_BITS, false)
                        } else if Self::FRAC_BITS as u32 == $Inner::BITS {
                            (0, true)
                        } else {
                            (ans_int << Self::FRAC_BITS, true)
                        };
                        let ansb = ansb_abs as $Inner;
                        let overflow2 = ansb.is_negative();
                        (Self::from_bits(ansb | rem_frac), overflow1 | overflow2)
                    }
                    if_unsigned! {
                        $Signedness;
                        (self.unwrapped_rem_int(rhs), false)
                    }
                }
            }

            comment! {
                "Returns the square root.

Returns a [tuple] of the result and ",
                if_signed_unsigned!(
                    $Signedness,
                    "a [`bool`] indicationg whether an overflow has occurred. On
overflow, the wrapped value is returned.",
                    "[`false`], since this can never overflow for unsigned numbers.",
                ),
                "

This method uses an iterative method, with up to ", $n, " iterations for
 [`", stringify!($Self), "`]. The result is rounded down, and the error is
&lt;&nbsp;[`DELTA`][Self::DELTA]. That is,
result&nbsp;≤&nbsp;√`self`&nbsp;&lt;&nbsp;result&nbsp;+&nbsp;`DELTA`.

",
                if_signed_else_empty_str! {
                    $Signedness;
                    "Overflow can only occur when there are no integer bits and
the representable range is &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;&lt;&nbsp;0.5.
In this case, overflow occurs for an input value ≥&nbsp;0.25.

"
                },
                if_signed_else_empty_str! {
                    $Signedness;
                    "# Panics

Panics if the number is negative.

"
                },
                "# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(
    Fix::from_num(2).overflowing_sqrt(),
    (Fix::SQRT_2, false)
);
",
                if_signed_else_empty_str! {
                    $Signedness;
                    "
type AllFrac = ", stringify!($Self), "<", $n, ">;
assert_eq!(
    AllFrac::from_num(0.25).overflowing_sqrt(),
    (AllFrac::from_num(-0.5), true)
);
",
                },
                "```
";
                #[inline]
                #[must_use]
                pub const fn overflowing_sqrt(self) -> (Self, bool) {
                    if_signed_unsigned!(
                        $Signedness,
                        {
                            if self.is_negative() {
                                panic!("square root of negative number");
                            }
                            let u = $USelf::<FRAC>::from_bits(self.to_bits() as $UInner);
                            let s = $Self::from_bits(u.sqrt().to_bits() as $Inner);
                            (s, s.is_negative())
                        },
                        {
                            let Some(nz) = NonZero::<$UInner>::new(self.to_bits()) else {
                                return (Self::ZERO, false);
                            };
                            let ret = sqrt::$UInner(nz, Self::FRAC_BITS as u32);
                            (Self::from_bits(ret), false)
                        }
                    )
                }
            }

            comment! {
                "Overflowing linear interpolation between `start` and `end`.

Returns a [tuple] of the result and a [`bool`] indicationg whether an overflow
has occurred. On overflow, the wrapped value is returned.

The interpolated value is
`start`&nbsp;+&nbsp;`self`&nbsp;×&nbsp;(`end`&nbsp;&minus;&nbsp;`start`). This
is `start` when `self`&nbsp;=&nbsp;0, `end` when `self`&nbsp;=&nbsp;1, and
linear interpolation for all other values of `self`. Linear extrapolation is
performed if `self` is not in the range 0&nbsp;≤&nbsp;<i>x</i>&nbsp;≤&nbsp;1.

# Examples

```rust
#![feature(generic_const_exprs)]
# #![allow(incomplete_features)]

use fixed::", stringify!($Self), ";
type Fix = ", stringify!($Self), "<4>;
assert_eq!(
    Fix::from_num(0.5).overflowing_lerp(Fix::ZERO, Fix::MAX),
    (Fix::MAX / 2, false)
);
assert_eq!(
    Fix::from_num(1.5).overflowing_lerp(Fix::ZERO, Fix::MAX),
    (Fix::MAX.wrapping_add(Fix::MAX / 2), true)
);
```
";
                #[inline]
                #[must_use]
                pub const fn overflowing_lerp<const RANGE_FRAC: i32>(
                    self,
                    start: $Self<RANGE_FRAC>,
                    end: $Self<RANGE_FRAC>,
                ) -> ($Self<RANGE_FRAC>, bool) {
                    match lerp::$Inner(self.to_bits(), start.to_bits(), end.to_bits(), FRAC as u32)
                    {
                        (bits, overflow) => ($Self::from_bits(bits), overflow),
                    }
                }
            }
        }
    };
}
