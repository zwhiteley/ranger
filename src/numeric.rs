use core::fmt::{Display, Formatter};

/// An error for a ranged integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RangedError<IntType> {
    /// The provided value was too small.
    TooSmall {
        /// The value provided.
        value: IntType,

        /// The minimum value of the ranged type.
        minimum: IntType,
    },

    /// The provided value was too large.
    TooLarge {
        /// The value provided.
        value: IntType,

        /// The maximum value of the ranged type.
        maximum: IntType,
    },

    /// The range of the type is invalid (i.e., the minimum value exceeds the maximum value).
    ///
    /// # Remarks
    ///
    /// Unfortunately, at the time of writing, it is impossible to statically prevent this case
    /// from occurring, despite it being easily statically preventable -- the reason for this is
    /// bounds checking: Rust does not currently support it for constant generics (the static
    /// assertions crate will not work either as it is reliant on a "hack" that Rust actively
    /// guards against to prevent constant generic bounds checking).
    ///
    /// The check should be optimised out by the compiler when the functions are inlined.
    InvalidRange {
        /// The minimum value of the ranged type.
        minimum: IntType,

        /// The maximum value of the ranged type.
        maximum: IntType,
    },
}

impl<IntType: Display> Display for RangedError<IntType> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use RangedError::*;

        match self {
            TooSmall { value, minimum } =>
                write!(f, "value provided, {value}, is lesser than the minimum value, {minimum}, for the type"),
            TooLarge { value, maximum } =>
                write!(f, "value provided, {value}, is greater than the maximum value, {maximum}, for the type"),
            InvalidRange { minimum, maximum } => write!(f,
                                                        "the minimum value, {minimum}, is greater than the maximum value, {maximum}"
            ),
        }
    }
}

// core::error::Error is current unstable: https://github.com/rust-lang/rust/issues/103765 -- my
// 10 second skim suggests it is Error::backtrace which prevented the trait from being in core in
// the first place -- when the core error trait is stabilised, it will be implemented instead,
// in the mean time, std's will have to be used.
#[cfg(feature = "std")]
impl<IntType: Display> std::error::Error for RangedError<IntType> {}

macro_rules! _int_define {
    ($name:tt($int_ty:ty)) => {
        #[doc = concat!("A ranged [`", stringify!($int_ty), "`] type with a value between `MIN` \
        and `MAX`")]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name<const MIN: $int_ty, const MAX: $int_ty>($int_ty);

        impl<const MIN: $int_ty, const MAX: $int_ty> $name<MIN, MAX> {
            /// Create a new instance of the type.
            ///
            /// # Parameters
            ///
            /// The value must be in the range `MIN..=MAX`.
            #[inline]
            pub const fn new(
                value: $int_ty,
            ) -> Result<Self, $crate::numeric::RangedError<$int_ty>> {
                if MIN > MAX {
                    return Err($crate::numeric::RangedError::InvalidRange {
                        minimum: MIN,
                        maximum: MAX,
                    });
                }

                if value < MIN {
                    Err($crate::numeric::RangedError::TooSmall {
                        value,
                        minimum: MIN,
                    })
                } else if value > MAX {
                    Err($crate::numeric::RangedError::TooLarge {
                        value,
                        maximum: MAX,
                    })
                } else {
                    Ok(Self(value))
                }
            }

            /// Create a new instance of the type without a bounds check.
            ///
            /// # Safety
            ///
            /// The value **MUST** be within the range `MIN..=MAX` (if `MIN` is greater than `MAX`,
            /// this function is **ALWAYS** unsafe to call).
            #[inline]
            pub const unsafe fn new_unchecked(value: $int_ty) -> Self {
                Self(value)
            }

            /// Retrieve the inner value of the type.
            ///
            /// # Remarks
            ///
            /// This is guaranteed to return a value in the range `MIN..=MAX`.
            #[inline]
            pub const fn inner(self) -> $int_ty {
                self.0
            }
        }

        impl<const MIN: $int_ty, const MAX: $int_ty> ::core::fmt::Display for $name<MIN, MAX> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.0, f)
            }
        }

        impl<const MIN: $int_ty, const MAX: $int_ty> ::core::convert::TryFrom<$int_ty>
            for $name<MIN, MAX>
        {
            type Error = $crate::numeric::RangedError<$int_ty>;

            #[inline]
            fn try_from(value: $int_ty) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl<const MIN: $int_ty, const MAX: $int_ty> ::core::convert::From<$name<MIN, MAX>>
            for $int_ty
        {
            #[inline]
            fn from(value: $name<MIN, MAX>) -> Self {
                value.inner()
            }
        }

        impl<const MIN: $int_ty, const MAX: $int_ty> ::core::convert::AsRef<$int_ty>
            for $name<MIN, MAX>
        {
            #[inline]
            fn as_ref(&self) -> &$int_ty {
                &self.0
            }
        }

        // Convenience trait implementation
        impl<const MIN: $int_ty, const MAX: $int_ty> ::core::ops::Deref for $name<MIN, MAX> {
            type Target = $int_ty;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

        /* Considered trait impls
         *
         * I did consider implementing traits like ::core::ops::Add, ::core::ops::Multiply, etc,
         * however, there were two major concerns I had:
         *
         * - Should the result of adding two ranged integers together output a ranged integer or
         *   the base type?
         *
         *   - If the former option was implemented, the range of the resultant would have to be
         *     the range of one of the operands (as only the identity expression is supported for
         *     polymorphic constants), which could make the API unwieldy to use (as a Result would
         *     need to be returned to ensure the number was still in range).
         *
         *   - If the latter option was implemented, all methods and traits on the base type would
         *     need to be implemented for the ranged integers as well; furthermore, this
         *     behaviour could be unexpected (i.e., the consumer may think that adding two ranged
         *     integers should produce a ranged integer as well).
         *
         * As a result, I have decided to put more thought into it before making the decision (the
         * crate is still in the development phase so backwards compatibility isn't a major issue
         * at the moment, it is just a lot of wasted effort if I choose to go the other way).
         */
    };
}

// Unsigned ranged types
_int_define!(RangedU8(u8));
_int_define!(RangedU16(u16));
_int_define!(RangedU32(u32));
_int_define!(RangedU64(u64));
_int_define!(RangedU128(u128));

// Signed ranged types
_int_define!(RangedI8(i8));
_int_define!(RangedI16(i16));
_int_define!(RangedI32(i32));
_int_define!(RangedI64(i64));
_int_define!(RangedI128(i128));

#[cfg(test)]
mod tests {
    // This test is here for convenience (i.e., my IDE doesn't recognise generated tests and
    // doesn't offer test running -- I am sure there's a way around it but I do not care)
    #[test]
    fn _ignore() {}

    macro_rules! _test_ranged {
        ($module:tt, $name:tt($int_ty:ty)) => {
            mod $module {
                use $crate::numeric::*;

                // Convenience constants
                const MIN: $int_ty = <$int_ty>::MIN;
                const MAX: $int_ty = <$int_ty>::MAX;

                #[test]
                fn ctor_valid() {
                    let ranged = $name::<MIN, MAX>::new(10 as $int_ty);
                    assert!(ranged.is_ok());
                }

                #[test]
                fn ctor_boundary() {
                    let ranged = $name::<MIN, MIN>::new(MIN);
                    assert!(ranged.is_ok());
                }

                #[test]
                fn ctor_small() {
                    let ranged = $name::<MAX, MAX>::new(MAX - 1);
                    assert!(matches!(ranged.unwrap_err(), RangedError::TooSmall { .. }));
                }

                #[test]
                fn ctor_large() {
                    let ranged = $name::<MIN, MIN>::new(MIN + 1);
                    assert!(matches!(ranged.unwrap_err(), RangedError::TooLarge { .. }));
                }

                #[test]
                fn ctor_invalid() {
                    let ranged = $name::<MAX, MIN>::new(MIN);
                    assert!(matches!(ranged.unwrap_err(), RangedError::InvalidRange { .. }));
                }

                #[test]
                fn conv_valid() {
                    TryInto::<$name<MIN, MAX>>::try_into(10 as $int_ty).unwrap();
                }

                #[test]
                fn conv_invalid() {
                    TryInto::<$name<MIN, MIN>>::try_into(MIN + 1).unwrap_err();
                }

                #[test]
                fn inner() {
                    let number = MIN;
                    let ranged = $name::<MIN, MAX>::new(number).unwrap();
                    assert_eq!(number, ranged.inner());
                }

                #[test]
                fn deref() {
                    let number = MIN;
                    let ranged = $name::<MIN, MAX>::new(number).unwrap();
                    assert_eq!(number, *ranged);
                }
            }
        };
    }

    // Unsigned tests
    _test_ranged!(ranged_u8, RangedU8(u8));
    _test_ranged!(ranged_u16, RangedU16(u16));
    _test_ranged!(ranged_u32, RangedU32(u32));
    _test_ranged!(ranged_u64, RangedU64(u64));
    _test_ranged!(ranged_u128, RangedU128(u128));

    // Signed tests
    _test_ranged!(ranged_i8, RangedI8(i8));
    _test_ranged!(ranged_i16, RangedI16(i16));
    _test_ranged!(ranged_i32, RangedI32(i32));
    _test_ranged!(ranged_i64, RangedI64(i64));
    _test_ranged!(ranged_i128, RangedI128(i128));
}
