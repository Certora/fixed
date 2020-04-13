// Copyright © 2018–2019 Trevor Spiteri

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

// self must be positive
pub trait IntFracLog10 {
    fn int_part_log10(self) -> i32;
    fn frac_part_log10(self) -> i32;
}

impl IntFracLog10 for u8 {
    #[inline]
    fn int_part_log10(self) -> i32 {
        if self >= 100 {
            2
        } else if self >= 10 {
            1
        } else {
            debug_assert!(self >= 1);
            0
        }
    }

    #[inline]
    fn frac_part_log10(self) -> i32 {
        if self > 25 {
            -1
        } else if self > 2 {
            -2
        } else {
            debug_assert!(self > 0);
            -3
        }
    }
}

impl IntFracLog10 for u16 {
    #[inline]
    fn int_part_log10(self) -> i32 {
        if self >= 10_000 {
            4
        } else if self >= 1000 {
            3
        } else if self >= 100 {
            2
        } else if self >= 10 {
            1
        } else {
            debug_assert!(self >= 1);
            0
        }
    }

    #[inline]
    fn frac_part_log10(self) -> i32 {
        if self > 6553 {
            -1
        } else if self > 655 {
            -2
        } else if self > 65 {
            -3
        } else if self > 6 {
            -4
        } else {
            debug_assert!(self > 0);
            -5
        }
    }
}

#[inline]
fn int_part_log10_less_than_8(mut i: u32) -> i32 {
    debug_assert!(i < 100_000_000);
    let mut log = 0;
    if i >= 10_000 {
        i /= 10_000;
        log += 4;
    }
    log + if i >= 1000 {
        3
    } else if i >= 100 {
        2
    } else if i >= 10 {
        1
    } else {
        debug_assert!(i >= 1);
        0
    }
}

impl IntFracLog10 for u32 {
    fn int_part_log10(mut self) -> i32 {
        let mut log = 0;
        if self >= 100_000_000 {
            self /= 100_000_000;
            log += 8;
        }
        log + int_part_log10_less_than_8(self)
    }

    fn frac_part_log10(mut self) -> i32 {
        const MAX: u32 = u32::max_value();
        let mut log = 0;
        if self <= MAX / 100_000_000 {
            self *= 100_000_000;
            log += -8;
        }
        if self <= MAX / 10_000 {
            self *= 10_000;
            log += -4;
        }
        log + if self > MAX / 10 {
            -1
        } else if self > MAX / 100 {
            -2
        } else if self > MAX / 1000 {
            -3
        } else {
            debug_assert!(self > MAX / 10_000);
            -4
        }
    }
}

#[inline]
fn int_part_log10_less_than_16(mut i: u64) -> i32 {
    debug_assert!(i < 10_000_000_000_000_000);
    let mut log = 0;
    if i >= 100_000_000 {
        i /= 100_000_000;
        log += 8;
    }
    debug_assert_eq!(i >> 32, 0);
    log + int_part_log10_less_than_8(i as u32)
}

impl IntFracLog10 for u64 {
    fn int_part_log10(mut self) -> i32 {
        let mut log = 0;
        if self >= 10_000_000_000_000_000 {
            self /= 10_000_000_000_000_000;
            log += 16;
        }
        log + int_part_log10_less_than_16(self)
    }

    fn frac_part_log10(mut self) -> i32 {
        const MAX: u64 = u64::max_value();
        let mut log = 0;
        if self <= MAX / 10_000_000_000_000_000 {
            log += -16;
            self *= 10_000_000_000_000_000;
        }
        if self <= MAX / 100_000_000 {
            log += -8;
            self *= 100_000_000;
        }
        if self <= MAX / 10_000 {
            log += -4;
            self *= 10_000;
        }
        log + if self > MAX / 10 {
            -1
        } else if self > MAX / 100 {
            -2
        } else if self > MAX / 1000 {
            -3
        } else {
            debug_assert!(self > MAX / 10_000);
            -4
        }
    }
}

impl IntFracLog10 for u128 {
    fn int_part_log10(mut self) -> i32 {
        let mut log = 0;
        if self >= 100_000_000_000_000_000_000_000_000_000_000 {
            self /= 100_000_000_000_000_000_000_000_000_000_000;
            log += 32;
        }
        if self >= 10_000_000_000_000_000 {
            self /= 10_000_000_000_000_000;
            log += 16;
        }
        debug_assert_eq!(self >> 64, 0);
        log + int_part_log10_less_than_16(self as u64)
    }

    fn frac_part_log10(mut self) -> i32 {
        const MAX: u128 = u128::max_value();
        let mut log = 0;
        if self <= MAX / 100_000_000_000_000_000_000_000_000_000_000 {
            log += -32;
            self *= 100_000_000_000_000_000_000_000_000_000_000;
        }
        if self <= MAX / 10_000_000_000_000_000 {
            log += -16;
            self *= 10_000_000_000_000_000;
        }
        if self <= MAX / 100_000_000 {
            log += -8;
            self *= 100_000_000;
        }
        if self <= MAX / 10_000 {
            log += -4;
            self *= 10_000;
        }
        log + if self > MAX / 10 {
            -1
        } else if self > MAX / 100 {
            -2
        } else if self > MAX / 1000 {
            -3
        } else {
            debug_assert!(self > MAX / 10_000);
            -4
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntFracLog10;

    #[test]
    fn int_part_log10_u8() {
        assert_eq!(u8::max_value().int_part_log10(), 2);
        for i in 0..=2 {
            let p = 10u8.pow(i as u32);
            if i > 0 {
                assert_eq!((p - 1).int_part_log10(), i - 1);
            }
            assert_eq!(p.int_part_log10(), i);
            assert_eq!((p + 1).int_part_log10(), i);
        }
    }

    #[test]
    fn frac_part_log10_u8() {
        assert_eq!(1u8.frac_part_log10(), -3);
        for i in 0..=2 {
            let p = u8::max_value() / 10u8.pow(i as u32);
            if p > 1 {
                assert_eq!((p - 1).frac_part_log10(), -1 - i);
            }
            assert_eq!(p.frac_part_log10(), -1 - i);
            if i > 0 {
                assert_eq!((p + 1).frac_part_log10(), -i);
            }
        }
    }

    #[test]
    fn int_part_log10_u16() {
        assert_eq!(u16::max_value().int_part_log10(), 4);
        for i in 0..=4 {
            let p = 10u16.pow(i as u32);
            if i > 0 {
                assert_eq!((p - 1).int_part_log10(), i - 1);
            }
            assert_eq!(p.int_part_log10(), i);
            assert_eq!((p + 1).int_part_log10(), i);
        }
    }

    #[test]
    fn frac_part_log10_u16() {
        assert_eq!(1u16.frac_part_log10(), -5);
        for i in 0..=4 {
            let p = u16::max_value() / 10u16.pow(i as u32);
            if p > 1 {
                assert_eq!((p - 1).frac_part_log10(), -1 - i);
            }
            assert_eq!(p.frac_part_log10(), -1 - i);
            if i > 0 {
                assert_eq!((p + 1).frac_part_log10(), -i);
            }
        }
    }

    #[test]
    fn int_part_log10_u32() {
        assert_eq!(u32::max_value().int_part_log10(), 9);
        for i in 0..=9 {
            let p = 10u32.pow(i as u32);
            if i > 0 {
                assert_eq!((p - 1).int_part_log10(), i - 1);
            }
            assert_eq!(p.int_part_log10(), i);
            assert_eq!((p + 1).int_part_log10(), i);
        }
    }

    #[test]
    fn frac_part_log10_u32() {
        assert_eq!(1u32.frac_part_log10(), -10);
        for i in 0..=9 {
            let p = u32::max_value() / 10u32.pow(i as u32);
            if p > 1 {
                assert_eq!((p - 1).frac_part_log10(), -1 - i);
            }
            assert_eq!(p.frac_part_log10(), -1 - i);
            if i > 0 {
                assert_eq!((p + 1).frac_part_log10(), -i);
            }
        }
    }

    #[test]
    fn int_part_log10_u64() {
        assert_eq!(u64::max_value().int_part_log10(), 19);
        for i in 0..=19 {
            let p = 10u64.pow(i as u32);
            if i > 0 {
                assert_eq!((p - 1).int_part_log10(), i - 1);
            }
            assert_eq!(p.int_part_log10(), i);
            assert_eq!((p + 1).int_part_log10(), i);
        }
    }

    #[test]
    fn frac_part_log10_u64() {
        assert_eq!(1u64.frac_part_log10(), -20);
        for i in 0..=19 {
            let p = u64::max_value() / 10u64.pow(i as u32);
            if p > 1 {
                assert_eq!((p - 1).frac_part_log10(), -1 - i);
            }
            assert_eq!(p.frac_part_log10(), -1 - i);
            if i > 0 {
                assert_eq!((p + 1).frac_part_log10(), -i);
            }
        }
    }

    #[test]
    fn int_part_log10_u128() {
        assert_eq!(u128::max_value().int_part_log10(), 38);
        for i in 0..=38 {
            let p = 10u128.pow(i as u32);
            if i > 0 {
                assert_eq!((p - 1).int_part_log10(), i - 1);
            }
            assert_eq!(p.int_part_log10(), i);
            assert_eq!((p + 1).int_part_log10(), i);
        }
    }

    #[test]
    fn frac_part_log10_u128() {
        assert_eq!(1u128.frac_part_log10(), -39);
        for i in 0..=38 {
            let p = u128::max_value() / 10u128.pow(i as u32);
            if p > 1 {
                assert_eq!((p - 1).frac_part_log10(), -1 - i);
            }
            assert_eq!(p.frac_part_log10(), -1 - i);
            if i > 0 {
                assert_eq!((p + 1).frac_part_log10(), -i);
            }
        }
    }
}
