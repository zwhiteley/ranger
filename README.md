# Ranger

<h1 style="text-align: center">
    <img src="https://github.com/zwhiteley/ranger/blob/main/logo.png?raw=true" alt="logo">
</h1>

## Summary

Ranger is a crate which provides a set of traits, types, and macros which restricts the values of pre-existing types (
both primitive and user-defined) with the following objectives:

* To simplify enum/struct composition.
* To reduce boilerplate code.
* To unify "ranged" types (including their conversions).

## Justification

The need for "ranged" types stems from ergonomics and boilerplate -- consider the following example:

```rust
pub enum DentalPatient {
    Child {
        // Assumes a person can neither have a negative age nor an age above `255`.
        age: u8
    },
    
    Adult {
        age: u8
    }
}
```

Whilst this may seem like a contrived example, it still has an issue: in most countries, a child is someone who has an
age below `18` -- in this example, it is perfectly possible to construct a child with an age of `18` or above,
violating this legal standard. The typical fix for such an issue is to create two structs and control the values of the
age field using getters and setters:

```rust 
const AGE_BOUNDARY: u8 = 18;

pub enum DentalPatient {
    Child(ChildPatient),
    Adult(AdultPatient)
}

pub struct ChildPatient {
    age: u8
}

impl ChildPatient {
    pub fn new(age: u8) -> Option<Self> { /* ... */ }
    pub fn age(&self) -> u8 { /* ... */ }
    pub fn set_age(&mut self, age: u8) -> Result<(), ()> { /* ... */ }
}

pub struct AdultPatient { /* ... */ }
impl AdultPatient { /* ... */ }
```

Using this design, it is now impossible to construct a `DentalPatient::Child` with an age equal to or above `18`.
However, not only does this approach make the API more complex, but it requires a lot more code (most of which is
boilerplate and merely pollutes the file).

To resolve this, ranged types can be used instead:

```rust
use ranged::RangedU8;

const AGE_BOUNDARY: u8 = 18;

pub enum DentalPatient {
    Child {
        age: RangedU8<0, { AGE_BOUNDARY - 1 }>,
    },
    
    Adult {
        age: RangedU8<AGE_BOUNDARY, { u8::MAX }>,
    }
}
```

This operates similarly to the previous example, except the boilerplate is contained within a pre-made crate which
prevents invalid values from being constructed and implements several convenience traits as well (which a regular
developer will likely forgo for the sake of brevity).

## Limitations

No crate is without its limitations and the ranger crate has plenty:

* Technical debt -- if used inappropriately, it can result in technical debt being unnecessarily accumulated (in the
  context of the previous example, the government may decide that `18` is too high for someone to be considered a legal
  adult and decides to lower it to `16`, forcing the developer to change all the types, as the range is part of the
  type's signature, and recompile the program). <br><br>

  This is somewhat analogous to the issue with inheritance, whereby a small mistake in inheritance (e.g., a class may
  be too broadly defined) can result in a quirky design, hacks, and ultimately technical debt. <br><br>

* Alternatives -- where possible, alternatives should be used -- for example, ranged types should **not** be used to
  represent days of the week: despite being unlikely to change, ranged types would be confusing (e.g., does `0`
  represent `Sunday` or `Monday`, what does `3` mean in this context, miscalculation, etc) -- in cases like these,
  enumerations should be preferred as they are more descriptive (only in cases where assigning variants meaningful
  names is not possible, or where it would be infeasible to provide each variant a descriptive name, should ranged
  types be used). <br><br>

* Half-baked -- unfortunately, generic constants aren't particularly flexible, especially with respect to bounds --
  this prevents a lot of convenience traits from being implemented (for example, it should be possible to implement
  the `From<U>` for all ranged types `T` where the range of `U` is larger than the range of `T`) as it is impossible
  to meaningfully compare to generic constants. It should be noted that there are plans to implement these traits when
  Rust stabilises a method to place bounds on generic constant.

## Licence

This crate is dual-licenced under the MIT licence and the Apache-2.0 licence.