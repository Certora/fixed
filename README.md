<!-- Copyright © 2018–2024 Trevor Spiteri -->

<!-- Copying and distribution of this file, with or without
modification, are permitted in any medium without royalty provided the
copyright notice and this notice are preserved. This file is offered
as-is, without any warranty. -->

# Fixed-point numbers

**Alpha:** This is an alpha release of the new major version 2.0.0 that makes
use of const generics instead of the [*typenum*
crate](https://crates.io/crate/typenum). This version requires the nightly
compiler with the [`generic_const_exprs` feature] enabled. The stable version
2.0.0 itself will not be released before the [`generic_const_exprs` feature] is
stabilized. See the documentation for [porting from version 1 to version 2].

[`generic_const_exprs` feature]: https://github.com/rust-lang/rust/issues/76560
[porting from version 1 to version 2]: https://tspiteri.gitlab.io/fixed/dev/fixed/index.html#porting-from-version-1-to-version-2

The [*fixed* crate] provides fixed-point numbers.

  * [`FixedI8`] and [`FixedU8`] are eight-bit fixed-point numbers.
  * [`FixedI16`] and [`FixedU16`] are 16-bit fixed-point numbers.
  * [`FixedI32`] and [`FixedU32`] are 32-bit fixed-point numbers.
  * [`FixedI64`] and [`FixedU64`] are 64-bit fixed-point numbers.
  * [`FixedI128`] and [`FixedU128`] are 128-bit fixed-point numbers.

An <i>n</i>-bit fixed-point number has <i>f</i>&nbsp;=&nbsp;`FRAC` fractional
bits, and <i>n</i>&nbsp;&minus;&nbsp;<i>f</i> integer bits. For example,
<code>[FixedI32]\<24></code> is a 32-bit signed fixed-point number with
<i>n</i>&nbsp;=&nbsp;32 total bits, <i>f</i>&nbsp;=&nbsp;24 fractional bits, and
<i>n</i>&nbsp;&minus;&nbsp;<i>f</i>&nbsp;=&nbsp;8 integer bits.
<code>[FixedI32]\<0></code> behaves like [`i32`], and
<code>[FixedU32]\<0></code> behaves like [`u32`].

The difference between any two successive representable numbers is constant
throughout the possible range for a fixed-point number:
<i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>f</i></sup>. When <i>f</i>&nbsp;=&nbsp;0, like
in <code>[FixedI32]\<0></code>, <i>Δ</i>&nbsp;=&nbsp;1 because representable
numbers are integers, and the difference between two successive integers is 1.
When <i>f</i>&nbsp;=&nbsp;<i>n</i>, <i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>n</i></sup>
and the value lies in the range &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;0.5
for signed numbers like <code>[FixedI32]\<32></code>, and in the range
0&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;1 for unsigned numbers like
<code>[FixedU32]\<32></code>.

The main features are

  * Representation of binary fixed-point numbers up to 128 bits wide.
  * Conversions between fixed-point numbers and numeric primitives.
  * Comparisons between fixed-point numbers and numeric primitives.
  * Parsing from strings in decimal, binary, octal and hexadecimal.
  * Display as decimal, binary, octal and hexadecimal.
  * Arithmetic and logic operations.

This crate does *not* provide decimal fixed-point numbers. For example 0.001
cannot be represented exactly, as it is 1/10<sup>3</sup>. It is binary fractions
like 1/2<sup>4</sup> (0.0625) that can be represented exactly, provided there
are enough fractional bits.

This crate does *not* provide general analytic functions.

  * No algebraic functions are provided, for example no `pow`.
  * No trigonometric functions are provided, for example no `sin` or `cos`.
  * No other transcendental functions are provided, for example no `log` or
    `exp`.

These functions are not provided because different implementations can have
different trade-offs, for example trading some correctness for speed.
Implementations can be provided in other crates.

  * The [*cordic* crate] provides various functions implemented using the
    [CORDIC] algorithm.

The conversions supported cover the following cases.

  * Infallible lossless conversions between fixed-point numbers and numeric
    primitives are provided using [`From`] and [`Into`]. These never fail
    (infallible) and do not lose any bits (lossless).
  * Infallible lossy conversions between fixed-point numbers and numeric
    primitives are provided using the [`LossyFrom`] and [`LossyInto`] traits.
    The source can have more fractional bits than the destination.
  * Checked lossless conversions between fixed-point numbers and numeric
    primitives are provided using the [`LosslessTryFrom`] and
    [`LosslessTryInto`] traits. The source cannot have more fractional bits than
    the destination.
  * Checked conversions between fixed-point numbers and numeric primitives are
    provided using the [`FromFixed`] and [`ToFixed`] traits, or using the
    [`from_num`] and [`to_num`] methods and [their checked
    versions][`checked_from_num`].
  * Additionally, [`az`] casts are implemented for conversion between
    fixed-point numbers and numeric primitives.
  * Fixed-point numbers can be parsed from decimal strings using [`FromStr`],
    and from binary, octal and hexadecimal strings using the
    [`from_str_binary`], [`from_str_octal`] and [`from_str_hex`] methods. The
    result is rounded to the nearest, with ties rounded to even.
  * Fixed-point numbers can be converted to strings using [`Display`],
    [`Binary`], [`Octal`], [`LowerHex`], [`UpperHex`], [`LowerExp`] and
    [`UpperExp`]. The output is rounded to the nearest, with ties rounded to
    even.
  * All fixed-point numbers are plain old data, so [`bytemuck`] bit casting
    conversions can be used.

## What’s new

### Version 2.0.0-alpha.28.0 news (2024-07-25)

  * The crate now requires the nightly compiler with the [`generic_const_exprs`
    feature] enabled.
  * The crate now uses generic constant expressions to specify the number of
    fractional bits.
  * The [`Fixed`][tf-2-0a] trait constraints have been relaxed. Methods that
    required the number of fractional bits to be bounded have been moved to the
    new subtrait [`FixedBoundFrac`][tfbf-2-0a].
  * The `INT_NBITS` and `FRAC_NBITS` associated constants were replaced with
    [`INT_BITS`][f-ib-2-0a] and [`FRAC_BITS`][f-fb-2-0a] which can be negative.
  * The [`Unwrapped`][u-2-0a] methods [`from_str_binary`][u-fsb-2-0a],
    [`from_str_octal`][u-fso-2-0a] and [`from_str_hex`][u-fsh-2-0a] return the
    value directly instead of a [`Result`].
  * The deprecated `F128Bits` and `F128` structs have been removed. They were
    replaced by the [`f128`] primitive.
  * The deprecated `const_fixed_from_int` macro has been removed. It was
    replaced by the [`const_from_int`][f-cfi-2-0a] method in version 1.20.0.
  * The deprecated optional features `az` and `f16` were removed. These features
    had no effect, as the functionality they enabled is now always enabled.
  * All deprecated methods have been removed.
  * The new generic associated type [`Fixed`][fb-f-2-0a] was added to the
    [`FixedBits`][fb-2-0a] trait.
  * The following methods of the [`Fixed`][tf-2-0a] trait and of the
    [`Saturating`][s-2-0a], [`Wrapping`][w-2-0a] and [`Unwrapped`][u-2-0a]
    wrappers now have some parameters and return types that can be generic:
      * [`mul_add`][tf-mad-2-0a], [`add_prod`][tf-ap-2-0a],
        [`mul_acc`][tf-mac-2-0a]
  * The following methods of the [`Fixed`][tf-2-0a] trait now have some
    parameters and return types that can be generic:
      * [`checked_mul_add`][tf-cmad-2-0a], [`checked_add_prod`][tf-cap-2-0a],
        [`checked_mul_acc`][tf-cmac-2-0a]
      * [`saturating_mul_add`][tf-smad-2-0a],
        [`saturating_add_prod`][tf-sap-2-0a],
        [`saturating_mul_acc`][tf-smac-2-0a]
      * [`wrapping_mul_add`][tf-wmad-2-0a], [`wrapping_add_prod`][tf-wap-2-0a],
        [`wrapping_mul_acc`][tf-wmac-2-0a]
      * [`unwrapped_mul_add`][tf-umad-2-0a],
        [`unwrapped_add_prod`][tf-uap-2-0a], [`unwrapped_mul_acc`][tf-umac-2-0a]
      * [`overflowing_mul_add`][tf-omad-2-0a],
        [`overflowing_add_prod`][tf-oap-2-0a],
        [`overflowing_mul_acc`][tf-omac-2-0a]

[`generic_const_exprs` feature]: https://github.com/rust-lang/rust/issues/76560
[f-cfi-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.const_from_int
[f-fb-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#associatedconstant.FRAC_BITS
[f-ib-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#associatedconstant.INT_BITS
[fb-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.FixedBits.html
[fb-f-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.FixedBits.html#associatedtype.Fixed
[s-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Saturating.html
[tf-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html
[tf-ap-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.add_prod
[tf-cap-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.checked_add_prod
[tf-cmac-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.checked_mul_acc
[tf-cmad-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.checked_mul_add
[tf-mac-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.mul_acc
[tf-mad-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.mul_add
[tf-oap-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.overflowing_add_prod
[tf-omac-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.overflowing_mul_acc
[tf-omad-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.overflowing_mul_add
[tf-sap-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.saturating_add_prod
[tf-smac-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.saturating_mul_acc
[tf-smad-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.saturating_mul_add
[tf-uap-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.unwrapped_add_prod
[tf-umac-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.unwrapped_mul_acc
[tf-umad-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.unwrapped_mul_add
[tf-wap-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.wrapping_add_prod
[tf-wmac-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.wrapping_mul_acc
[tf-wmad-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html#tymethod.wrapping_mul_add
[tfbf-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.FixedBoundFrac.html
[u-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html
[u-fsb-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html#method.from_str_binary
[u-fsh-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html#method.from_str_hex
[u-fso-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html#method.from_str_octal
[w-2-0a]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Wrapping.html

### Version 1.28.0 news (2024-07-25)

  * The crate now requires rustc version 1.79.0 or later.
  * The following methods were added to all fixed-point numbers and to the
    [`Fixed`][tf-1-28] trait:
      * [`unchecked_add`][f-ua-1-28], [`unchecked_sub`][f-us-1-28],
        [`unchecked_mul_int`][f-umi-1-28]
  * For all fixed-point numbers and the [`Fixed`][tf-1-28] trait, the following
    methods were renamed. The old method names are deprecated.
      * [`round_ties_to_even`][f-rtte-1-28] renamed to
        [`round_ties_even`][f-rte-1-28]
      * [`checked_round_ties_to_even`][f-crtte-1-28] renamed to
        [`checked_round_ties_even`][f-crte-1-28]
      * [`saturating_round_ties_to_even`][f-srtte-1-28] renamed to
        [`saturating_round_ties_even`][f-srte-1-28]
      * [`wrapping_round_ties_to_even`][f-wrtte-1-28] renamed to
        [`wrapping_round_ties_even`][f-wrte-1-28]
      * [`unwrapped_round_ties_to_even`][f-urtte-1-28] renamed to
        [`unwrapped_round_ties_even`][f-urte-1-28]
      * [`overflowing_round_ties_to_even`][f-ortte-1-28] renamed to
        [`overflowing_round_ties_even`][f-orte-1-28]
  * The following constants were added to the [`consts`][c-1-28] module and as
    associated constants to fixed-point types:
      * [`SQRT_2PI`][c-r2p-1-28], [`FRAC_1_SQRT_2PI`][c-1r2p-1-28]
  * The experimental feature [`nightly-float`][feat-exp-1-28] was added.
  * For the experimental feature [`num-traits`][feat-exp-1-28], the following
    traits were implemented for all fixed-point numbers:
      * [`ConstZero`][nt-0-2-cz], [`ConstOne`][nt-0-2-co]
      * [`ToBytes`][nt-0-2-tb], [`FromBytes`][nt-0-2-fb]

[c-1-28]: https://docs.rs/fixed/~1.28/fixed/consts/index.html
[c-1r2p-1-28]: https://docs.rs/fixed/~1.28/fixed/consts/constant.FRAC_1_SQRT_2PI.html
[c-r2p-1-28]: https://docs.rs/fixed/~1.28/fixed/consts/constant.SQRT_2PI.html
[f-crte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.checked_round_ties_even
[f-crtte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.checked_round_ties_to_even
[f-orte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.overflowing_round_ties_even
[f-ortte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.overflowing_round_ties_to_even
[f-rte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.round_ties_even
[f-rtte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.round_ties_to_even
[f-srte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.saturating_round_ties_even
[f-srtte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.saturating_round_ties_to_even
[f-ua-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.unchecked_add
[f-umi-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.unchecked_mul_int
[f-urte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.unwrapped_round_ties_even
[f-urtte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.unwrapped_round_ties_to_even
[f-us-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.unchecked_sub
[f-wrte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.wrapping_round_ties_even
[f-wrtte-1-28]: https://docs.rs/fixed/~1.28/fixed/struct.FixedI32.html#method.wrapping_round_ties_to_even
[feat-exp-1-28]: https://docs.rs/fixed/~1.28/fixed/index.html#experimental-optional-features
[nt-0-2-co]: https://docs.rs/num-traits/^0.2/num_traits/identities/trait.ConstOne.html
[nt-0-2-cz]: https://docs.rs/num-traits/^0.2/num_traits/identities/trait.ConstZero.html
[nt-0-2-fb]: https://docs.rs/num-traits/^0.2/num_traits/ops/bytes/trait.FromBytes.html
[nt-0-2-tb]: https://docs.rs/num-traits/^0.2/num_traits/ops/bytes/trait.ToBytes.html
[tf-1-28]: https://docs.rs/fixed/~1.28/fixed/traits/trait.Fixed.html

### Other releases

Details on other releases can be found in [*RELEASES.md*].

[*RELEASES.md*]: https://gitlab.com/tspiteri/fixed/blob/master/RELEASES.md

## Quick examples

```rust
#![feature(generic_const_exprs)]

use fixed::types::I20F12;

// 19/3 = 6 1/3
let six_and_third = I20F12::from_num(19) / 3;
// four decimal digits for 12 binary digits
assert_eq!(six_and_third.to_string(), "6.3333");
// find the ceil and convert to i32
assert_eq!(six_and_third.ceil().to_num::<i32>(), 7);
// we can also compare directly to integers
assert_eq!(six_and_third.ceil(), 7);
```

The type [`I20F12`] is a 32-bit fixed-point signed number with 20 integer bits
and 12 fractional bits. It is an alias to <code>[FixedI32]\<12></code>. The
unsigned counterpart would be [`U20F12`]. Aliases are provided for all
combinations of integer and fractional bits adding up to a total of eight, 16,
32, 64 or 128 bits.

```rust
#![feature(generic_const_exprs)]

use fixed::types::{I4F4, I4F12};

// -8 ≤ I4F4 < 8 with steps of 1/16 (~0.06)
let a = I4F4::from_num(1);
// multiplication and division by integers are possible
let ans1 = a / 5 * 17;
// 1 / 5 × 17 = 3 2/5 (3.4), but we get 3 3/16 (~3.2)
assert_eq!(ans1, I4F4::from_bits((3 << 4) + 3));
assert_eq!(ans1.to_string(), "3.2");

// -8 ≤ I4F12 < 8 with steps of 1/4096 (~0.0002)
let wider_a = I4F12::from(a);
let wider_ans = wider_a / 5 * 17;
let ans2 = I4F4::from_num(wider_ans);
// now the answer is the much closer 3 6/16 (~3.4)
assert_eq!(ans2, I4F4::from_bits((3 << 4) + 6));
assert_eq!(ans2.to_string(), "3.4");
```

The second example shows some precision and conversion issues. The low precision
of `a` means that `a / 5` is 3⁄16 instead of 1⁄5, leading to an inaccurate
result `ans1` = 3 3⁄16 (~3.2). With a higher precision, we get `wider_a / 5`
equal to 819⁄4096, leading to a more accurate intermediate result `wider_ans` =
3 1635⁄4096. When we convert back to four fractional bits, we get `ans2` = 3
6⁄16 (~3.4).

Note that we can convert from [`I4F4`] to [`I4F12`] using [`From`], as the
target type has the same number of integer bits and a larger number of
fractional bits. Converting from [`I4F12`] to [`I4F4`] cannot use [`From`] as we
have less fractional bits, so we use [`from_num`] instead.

## Writing fixed-point constants and values literally

The [`lit`] method, which is available as a `const` function, can be used to
parse literals. It supports
  * underscores as separators;
  * prefixes “`0b`”, “`0o`” and “`0x`” for binary, octal and hexadecimal
    numbers;
  * an optional decimal exponent with separator “`e`” or “`E`” for decimal,
    binary and octal numbers, or with separator “`@`” for all supported radices
    including hexadecimal.

```rust
#![feature(generic_const_exprs)]

use fixed::types::I16F16;

// 0.1275e2 is 12.75
const TWELVE_POINT_75: I16F16 = I16F16::lit("0.127_5e2");
// 1.8 hexadecimal is 1.5 decimal, and 18@-1 is 1.8
const ONE_POINT_5: I16F16 = I16F16::lit("0x_18@-1");
// 12.75 + 1.5 = 14.25
let sum = TWELVE_POINT_75 + ONE_POINT_5;
assert_eq!(sum, 14.25);
```

## Using the *fixed* crate

The *fixed* crate is available on [crates.io][*fixed* crate]. To use it in your
crate, add it as a dependency inside [*Cargo.toml*]:

```toml
[dependencies]
fixed = "2.0.0-alpha.28.0"
```

This alpha version of the *fixed* crate requires the nightly compiler with the
[`generic_const_exprs` feature] enabled.

[`generic_const_exprs` feature]: https://github.com/rust-lang/rust/issues/76560

## Optional features

The *fixed* crate has these optional feature:

 1. `arbitrary`, disabled by default. This provides the generation of arbitrary
    fixed-point numbers from raw, unstructured data. This feature requires the
    [*arbitrary* crate].
 2. `borsh`, disabled by default. This implements serialization and
    deserialization using the [*borsh* crate].
 3. `serde`, disabled by default. This provides serialization support for the
    fixed-point types. This feature requires the [*serde* crate].
 4. `std`, disabled by default. This is for features that are not possible under
    `no_std`: currently the implementation of the [`Error`] trait for
    [`ParseFixedError`].
 5. `serde-str`, disabled by default. Fixed-point numbers are serialized as
    strings showing the value when using human-readable formats. This feature
    requires the `serde` and the `std` optional features. With this feature,
    serialization is only supported for fixed-point numbers where the number of
    fractional bits is from zero to the total number of bits. **Warning:**
    numbers serialized when this feature is enabled cannot be deserialized when
    this feature is disabled, and vice versa.

To enable features, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.fixed]
features = ["serde"]
version = "2.0.0-alpha.28.0"
```

## Experimental optional features

It is not considered a breaking change if the following experimental features
are removed. The removal of experimental features would however require a minor
version bump. Similarly, on a minor version bump, optional dependencies can be
updated to an incompatible newer version.

 1. `num-traits`, disabled by default. This implements some traits from the
    [*num-traits* crate]. (The plan is to promote this to an optional feature
    once the [*num-traits* crate] reaches version 1.0.0.)

## Porting from version 1 to version 2

To port from version 1 to version 2, the following is required:

  * Temporary change required until the [`generic_const_exprs` feature] are
    stabilized: use the nightly compiler and enable the [`generic_const_exprs`
    feature] using

    ```rust
    #![feature(generic_const_exprs)]
    ```

  * Use integer literals instead of typenum integer constants, for example
    <code>[FixedI32][`FixedI32`]&lt;8></code> instead of
    <code>[FixedI32][`FixedI32`]&lt;[U8]></code>.

    [U8]: https://docs.rs/fixed/1/fixed/types/extra/type.U8.html

  * The [`Fixed`] trait constraints have been relaxed. Methods that required the
    number of fractional bits to be bounded have been moved to the new subtrait
    [`FixedBoundFrac`]. For code that uses these trait methods, [`Fixed`] should
    be replaced by [`FixedBoundFrac`].

    [`Fixed`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.Fixed.html
    [`FixedBoundFrac`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.FixedBoundFrac.html

  * The [`FRAC_NBITS`] and [`INT_NBITS`] associated constants of type [`u32`]
    were replaced by [`FRAC_BITS`] and [`INT_BITS`] of type [`i32`].

    [`FRAC_BITS`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#associatedconstant.FRAC_BITS
    [`FRAC_NBITS`]: https://docs.rs/fixed/1/fixed/struct.FixedI32.html#associatedconstant.FRAC_NBITS
    [`INT_BITS`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#associatedconstant.INT_BITS
    [`INT_NBITS`]: https://docs.rs/fixed/1/fixed/struct.FixedI32.html#associatedconstant.INT_NBITS

  * For the [`Unwrapped`] wrapper, the methods [`from_str_binary`][u-fsb],
    [`from_str_octal`][u-fso] and [`from_str_hex`][u-fsh] return the value
    directly instead of a [`Result`].

    [`Result`]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html
    [`Unwrapped`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html
    [u-fsb]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html#method.from_str_binary
    [u-fsh]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html#method.from_str_hex
    [u-fso]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.Unwrapped.html#method.from_str_octal

  * The deprecated [`F128Bits`] and [`F128`] structs have been removed. They
    were replaced by the [`f128`] primitive.

    [`F128Bits`]: https://docs.rs/fixed/1/fixed/struct.F128Bits.html
    [`F128`]: https://docs.rs/fixed/1/fixed/struct.F128.html

  * The deprecated [`const_fixed_from_int`] macro has been removed. It was
    replaced by the [`const_from_int`][f-cfi] method in version 1.20.0.

    [`const_fixed_from_int`]: https://docs.rs/fixed/1/fixed/macro.const_fixed_from_int.html
    [f-cfi]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.const_from_int

  * The deprecated optional features `az` and `f16` were removed. These features
    had no effect, as their functionality has been unconditionally enabled since
    version 1.7.0.

  * All deprecated methods have been removed.

## License

This crate is free software: you can redistribute it and/or modify it under the
terms of either

  * the [Apache License, Version 2.0][LICENSE-APACHE] or
  * the [MIT License][LICENSE-MIT]

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache License, Version 2.0,
shall be dual licensed as above, without any additional terms or conditions.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*arbitrary* crate]: https://crates.io/crates/arbitrary
[*borsh* crate]: https://crates.io/crates/borsh
[*cordic* crate]: https://crates.io/crates/cordic
[*fixed* crate]: https://crates.io/crates/fixed
[*half* crate]: https://crates.io/crates/half
[*num-traits* crate]: https://crates.io/crates/num-traits
[*serde* crate]: https://crates.io/crates/serde
[CORDIC]: https://en.wikipedia.org/wiki/CORDIC
[FixedI32]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html
[FixedU32]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedU32.html
[LICENSE-APACHE]: https://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: https://opensource.org/licenses/MIT
[`Binary`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html
[`Display`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html
[`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
[`FixedI128`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI128.html
[`FixedI16`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI16.html
[`FixedI32`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html
[`FixedI64`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI64.html
[`FixedI8`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI8.html
[`FixedU128`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedU128.html
[`FixedU16`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedU16.html
[`FixedU32`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedU32.html
[`FixedU64`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedU64.html
[`FixedU8`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedU8.html
[`FromFixed`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.FromFixed.html
[`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`I20F12`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/types/type.I20F12.html
[`I4F12`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/types/type.I4F12.html
[`I4F4`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/types/type.I4F4.html
[`Into`]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html
[`LosslessTryFrom`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.LosslessTryFrom.html
[`LosslessTryInto`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.LosslessTryInto.html
[`LossyFrom`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.LossyFrom.html
[`LossyInto`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.LossyInto.html
[`LowerExp`]: https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html
[`LowerHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html
[`Octal`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html
[`ParseFixedError`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.ParseFixedError.html
[`ToFixed`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/traits/trait.ToFixed.html
[`U20F12`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/types/type.U20F12.html
[`UpperExp`]: https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html
[`UpperHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html
[`az`]: https://docs.rs/az/^1/az/index.html
[`bf16`]: https://docs.rs/half/^2/half/struct.bf16.html
[`bytemuck`]: https://docs.rs/bytemuck/^1/bytemuck/index.html
[`checked_from_num`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.checked_from_num
[`f128`]: https://doc.rust-lang.org/nightly/core/primitive.f128.html
[`from_num`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.from_num
[`from_str_binary`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.from_str_binary
[`from_str_hex`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.from_str_hex
[`from_str_octal`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.from_str_octal
[`i32`]: https://doc.rust-lang.org/nightly/core/primitive.i32.html
[`lit`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.lit
[`to_num`]: https://docs.rs/fixed/2.0.0-alpha.28.0/fixed/struct.FixedI32.html#method.to_num
[`u32`]: https://doc.rust-lang.org/nightly/core/primitive.u32.html
