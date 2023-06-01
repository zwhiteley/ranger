//! # Ranger
//!
//! Ranger is a no-std crate which offers a variety of types, traits, and macros for
//! value-restricted types (henceforth referred to as "ranged" types) which are restricted in
//! what values they are capable of holding (e.g., a [`RangedU8<0, 10>`](numeric::RangedU8) can
//! only hold `u8` values in the range `0..=10`).
//!
//! ## Purpose
//!
//! The purpose of this crate is to simplify struct and enum composition -- take the following
//! example:
//!
//! ```
//! struct LegalAdult {
//!     pub name: String,
//!
//!     // Assumes a person cannot have an age above u8::MAX
//!     pub age: u8,
//!
//!     /* other public fields */
//! }
//! ```
//!
//! For the sake of simplicity, I am going to assume that there is a universal legal age at which
//! a person becomes an adult (18).
//!
//! This example is _flawed_ -- it is possible to construct an adult with an age below the legal
//! age of an adult -- a naive approach to resolving this issue would be to use a constructor,
//! getters, and setters:
//!
//! ```
//! # /*
//! struct LegalAdult { ... }
//! # */
//! # struct LegalAdult { name: String, age: u8 }
//!
//! impl LegalAdult {
//!     pub const LEGAL_AGE: u8 = 18;
//!
//!     pub fn new(name: String, age: u8, /* ... */) -> Option<Self> {
//!         if age < Self::LEGAL_AGE { None }
//!         else { Some(Self { name, age, /* ... */ }) }
//!     }
//!
//!     pub fn age(&self) -> u8 { self.age }
//!
//!     pub fn set_age(&mut self, age: u8) {
//!         if age >= Self::LEGAL_AGE { self.age = age; }
//!     }
//! }
//! ```
//!
//! Unfortunately, this approach is verbose, requires a significant amount of boilerplate, and
//! is somewhat quirky to use. An alternative (and, in my opinion, better) approach would be
//! to use ranged types:
//!
//! ```
//! use ranger::numeric::RangedU8;
//!
//! const LEGAL_AGE: u8 = 18;
//!
//! struct LegalAdult {
//!     pub name: String,
//!     pub age: RangedU8<LEGAL_AGE, { u8::MAX }>,
//!
//!     /* Additional public fields */
//! }
//! ```
//!
//! The [`RangedU8`](numeric::RangedU8) ensures that the age provided is above the legal age,
//! circumventing the boilerplate entirely whilst preserving the field's invariants.
//!
//! ## Enums
//!
//! Ranged types are not a replacement for enums and should not be used to represent variants,
//! except in the following cases:
//!
//! * High variant number -- if there are a large number of variants, it may be impractical (or even
//!   infeasible) to provide every variant with a name -- in these cases, ranged types may be
//!   appropriate (it should be noted is very rare for an enum to have enough variants to warrant
//!   a ranged type, and it is usually indicative of a bad design).
//!
//!   Ranged types, in general, should not be used for bitflags and a specialised bitflag crate,
//!   like [`bitflags`](https://crates.io/crates/bitflags), should be used instead.
//!
//! * Unnameable variants -- if the variants cannot be assigned meaningful names (e.g., days of
//!   the month), then ranged types may be appropriate.
//!
//! * Deserialisation -- in cases where enum serialisation/deserialisation is required, ranged types
//!   may be invaluable (not only to they force the developer to protect against serialisation
//!   errors, but they convey meaning and information about the serialisation protocol).
//!
//!   It should be noted that, in general, specialised serialisation crates and macros should be
//!   used instead of creating an entirely new protocol from scratch.
//!
//! ## When to use
//!
//! Whilst ranged types offer desirable benefits, their use should be restricted
//! ([see the enums section](crate#enums)) to:
//!
//! * Unsafe code -- whilst this may seem counterproductive due to the overhead ranged types
//!   introduce (to ensure the type's invariants are upheld), they can be highly valuable in
//!   unsafe code as they more accurately convey meaning (e.g., it is more difficult to ignorantly
//!   violate a particular field's invariants if it is protected by a ranged type), automatically
//!   perform necessary checks, and offer unsafe methods to allow such checks to be skipped
//!   where appropriate.
//!
//! * Validation -- for example, a ranged string type could be used to force users to validate
//!   emails prior to use (not only enforcing a critical invariant at compile time, but possibly
//!   upholding a major security requirement).

#![no_std]

/// Contains ranged numeric types.
#[cfg(feature = "numeric")]
pub mod numeric;