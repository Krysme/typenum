//! This crate provides type-level numbers evaluated at compile time. It depends only on libcore.
//!
//! The traits defined or used in this crate are used in a typical manner. They can be divided into
//! two categories: **marker traits** and **type operators**.
//!
//! Many of the marker traits have functions defined, but they all do essentially the same thing:
//! convert a type into its runtime counterpart, and are really just there for debugging. For
//! example,
//!
//! ```rust
//! use typenum::{N4, Integer};
//!
//! assert_eq!(N4::to_i32(), -4);
//! ```
//!
//! **Type operators** are traits that behave as functions at the type level. These are the meat of
//! this library. Where possible, traits defined in libcore have been used, but their attached
//! functions have not been implemented.
//!
//! For example, the `Add` trait is implemented for both unsigned and signed integers, but the
//! `add` function is not. As there are never any objects of the types defined here, it wouldn't
//! make sense to implement it. What is important is its associated type `Output`, which is where
//! the addition happens.
//!
//! ```rust
//! use std::ops::Add;
//! use typenum::{Integer, P3, P4};
//!
//! type X = <P3 as Add<P4>>::Output;
//! assert_eq!(<X as Integer>::to_i32(), 7);
//! ```
//!
//! In addition, helper aliases are defined for type operators. For example, the above snippet
//! could be replaced with
//!
//! ```rust
//! use typenum::{Sum, Integer, P3, P4};
//!
//! type X = Sum<P3, P4>;
//! assert_eq!(<X as Integer>::to_i32(), 7);
//! ```
//!
//! Documented in each module is the full list of type operators implemented.
//!

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(feature = "strict", deny(missing_docs))]
#![cfg_attr(feature = "strict", deny(warnings))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::type_complexity,
        clippy::len_without_is_empty,
        clippy::new_without_default,
        clippy::many_single_char_names
    )
)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::missing_inline_in_public_items))]

// For debugging macros:
// #![feature(trace_macros)]
// trace_macros!(true);

use core::cmp::Ordering;

#[cfg(feature = "force_unix_path_separator")]
mod generated {

    /**
    Convenient type operations.

    Any types representing values must be able to be expressed as `ident`s. That means they need to be
    in scope.

    For example, `P5` is okay, but `typenum::P5` is not.

    You may combine operators arbitrarily, although doing so excessively may require raising the
    recursion limit.

    # Example
    ```rust
    #![recursion_limit="128"]
    #[macro_use] extern crate typenum;
    use typenum::consts::*;

    fn main() {
        assert_type!(
            op!(min((P1 - P2) * (N3 + N7), P5 * (P3 + P4)) == P10)
        );
    }
    ```
    Operators are evaluated based on the operator precedence outlined
    [here](https://doc.rust-lang.org/reference.html#operator-precedence).

    The full list of supported operators and functions is as follows:

    `*`, `/`, `%`, `+`, `-`, `<<`, `>>`, `&`, `^`, `|`, `==`, `!=`, `<=`, `>=`, `<`, `>`, `cmp`, `sqr`, `sqrt`, `abs`, `cube`, `pow`, `min`, `max`, `log2`, `gcd`

    They all expand to type aliases defined in the `operator_aliases` module. Here is an expanded list,
    including examples:

    ---
    Operator `*`. Expands to `Prod`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P2 * P3), P6);
    # }
    ```

    ---
    Operator `/`. Expands to `Quot`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P6 / P2), P3);
    # }
    ```

    ---
    Operator `%`. Expands to `Mod`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P5 % P3), P2);
    # }
    ```

    ---
    Operator `+`. Expands to `Sum`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P2 + P3), P5);
    # }
    ```

    ---
    Operator `-`. Expands to `Diff`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P2 - P3), N1);
    # }
    ```

    ---
    Operator `<<`. Expands to `Shleft`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(U1 << U5), U32);
    # }
    ```

    ---
    Operator `>>`. Expands to `Shright`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(U32 >> U5), U1);
    # }
    ```

    ---
    Operator `&`. Expands to `And`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(U5 & U3), U1);
    # }
    ```

    ---
    Operator `^`. Expands to `Xor`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(U5 ^ U3), U6);
    # }
    ```

    ---
    Operator `|`. Expands to `Or`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(U5 | U3), U7);
    # }
    ```

    ---
    Operator `==`. Expands to `Eq`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P5 == P3 + P2), True);
    # }
    ```

    ---
    Operator `!=`. Expands to `NotEq`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P5 != P3 + P2), False);
    # }
    ```

    ---
    Operator `<=`. Expands to `LeEq`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P6 <= P3 + P2), False);
    # }
    ```

    ---
    Operator `>=`. Expands to `GrEq`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P6 >= P3 + P2), True);
    # }
    ```

    ---
    Operator `<`. Expands to `Le`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P4 < P3 + P2), True);
    # }
    ```

    ---
    Operator `>`. Expands to `Gr`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(P5 < P3 + P2), False);
    # }
    ```

    ---
    Operator `cmp`. Expands to `Compare`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(cmp(P2, P3)), Less);
    # }
    ```

    ---
    Operator `sqr`. Expands to `Square`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(sqr(P2)), P4);
    # }
    ```

    ---
    Operator `sqrt`. Expands to `Sqrt`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(sqrt(U9)), U3);
    # }
    ```

    ---
    Operator `abs`. Expands to `AbsVal`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(abs(N2)), P2);
    # }
    ```

    ---
    Operator `cube`. Expands to `Cube`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(cube(P2)), P8);
    # }
    ```

    ---
    Operator `pow`. Expands to `Exp`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(pow(P2, P3)), P8);
    # }
    ```

    ---
    Operator `min`. Expands to `Minimum`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(min(P2, P3)), P2);
    # }
    ```

    ---
    Operator `max`. Expands to `Maximum`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(max(P2, P3)), P3);
    # }
    ```

    ---
    Operator `log2`. Expands to `Log2`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(log2(U9)), U3);
    # }
    ```

    ---
    Operator `gcd`. Expands to `Gcf`.

    ```rust
    # #[macro_use] extern crate typenum;
    # use typenum::*;
    # fn main() {
    assert_type_eq!(op!(gcd(U9, U21)), U3);
    # }
    ```

    */
    #[macro_export]
    macro_rules! op {
    ($($tail:tt)*) => ( __op_internal__!($($tail)*) );
}

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __op_internal__ {

(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: cmp $($tail:tt)*) => (
    __op_internal__!(@stack[Compare, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: sqr $($tail:tt)*) => (
    __op_internal__!(@stack[Square, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: sqrt $($tail:tt)*) => (
    __op_internal__!(@stack[Sqrt, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: abs $($tail:tt)*) => (
    __op_internal__!(@stack[AbsVal, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: cube $($tail:tt)*) => (
    __op_internal__!(@stack[Cube, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: pow $($tail:tt)*) => (
    __op_internal__!(@stack[Exp, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: min $($tail:tt)*) => (
    __op_internal__!(@stack[Minimum, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: max $($tail:tt)*) => (
    __op_internal__!(@stack[Maximum, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: log2 $($tail:tt)*) => (
    __op_internal__!(@stack[Log2, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: gcd $($tail:tt)*) => (
    __op_internal__!(@stack[Gcf, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
    __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: , $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: , $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: * $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: * $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: * $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: * $($tail:tt)*) => (
    __op_internal__!(@stack[Prod, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: / $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: / $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: / $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: / $($tail:tt)*) => (
    __op_internal__!(@stack[Quot, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: % $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: % $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: % $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: % $($tail:tt)*) => (
    __op_internal__!(@stack[Mod, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: + $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: + $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: + $($tail:tt)*) => (
    __op_internal__!(@stack[Sum, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: - $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: - $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: - $($tail:tt)*) => (
    __op_internal__!(@stack[Diff, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: << $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: << $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: << $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: << $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: << $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: << $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: << $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: << $($tail:tt)*) => (
    __op_internal__!(@stack[Shleft, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: >> $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: >> $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: >> $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: >> $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: >> $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: >> $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: >> $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >> $($tail:tt)*) => (
    __op_internal__!(@stack[Shright, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: & $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: & $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: & $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: & $($tail:tt)*) => (
    __op_internal__!(@stack[And, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: ^ $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ^ $($tail:tt)*) => (
    __op_internal__!(@stack[Xor, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: | $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: | $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: | $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: | $($tail:tt)*) => (
    __op_internal__!(@stack[Or, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: == $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Eq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Eq, $($queue,)*] @tail: == $($tail)*)
);
(@stack[NotEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[NotEq, $($queue,)*] @tail: == $($tail)*)
);
(@stack[LeEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[LeEq, $($queue,)*] @tail: == $($tail)*)
);
(@stack[GrEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[GrEq, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Le, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Le, $($queue,)*] @tail: == $($tail)*)
);
(@stack[Gr, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Gr, $($queue,)*] @tail: == $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: == $($tail:tt)*) => (
    __op_internal__!(@stack[Eq, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: != $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Eq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Eq, $($queue,)*] @tail: != $($tail)*)
);
(@stack[NotEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[NotEq, $($queue,)*] @tail: != $($tail)*)
);
(@stack[LeEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[LeEq, $($queue,)*] @tail: != $($tail)*)
);
(@stack[GrEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[GrEq, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Le, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Le, $($queue,)*] @tail: != $($tail)*)
);
(@stack[Gr, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Gr, $($queue,)*] @tail: != $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: != $($tail:tt)*) => (
    __op_internal__!(@stack[NotEq, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Eq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Eq, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[NotEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[NotEq, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[LeEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[LeEq, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[GrEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[GrEq, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Le, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Le, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[Gr, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Gr, $($queue,)*] @tail: <= $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: <= $($tail:tt)*) => (
    __op_internal__!(@stack[LeEq, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Eq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Eq, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[NotEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[NotEq, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[LeEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[LeEq, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[GrEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[GrEq, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Le, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Le, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[Gr, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Gr, $($queue,)*] @tail: >= $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: >= $($tail:tt)*) => (
    __op_internal__!(@stack[GrEq, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: < $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Eq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Eq, $($queue,)*] @tail: < $($tail)*)
);
(@stack[NotEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[NotEq, $($queue,)*] @tail: < $($tail)*)
);
(@stack[LeEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[LeEq, $($queue,)*] @tail: < $($tail)*)
);
(@stack[GrEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[GrEq, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Le, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Le, $($queue,)*] @tail: < $($tail)*)
);
(@stack[Gr, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Gr, $($queue,)*] @tail: < $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: < $($tail:tt)*) => (
    __op_internal__!(@stack[Le, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[Prod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Prod, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Quot, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Quot, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Mod, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Mod, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Sum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sum, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Diff, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Diff, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Shleft, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shleft, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Shright, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Shright, $($queue,)*] @tail: > $($tail)*)
);
(@stack[And, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[And, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Xor, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Xor, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Or, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Or, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Eq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Eq, $($queue,)*] @tail: > $($tail)*)
);
(@stack[NotEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[NotEq, $($queue,)*] @tail: > $($tail)*)
);
(@stack[LeEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[LeEq, $($queue,)*] @tail: > $($tail)*)
);
(@stack[GrEq, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[GrEq, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Le, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Le, $($queue,)*] @tail: > $($tail)*)
);
(@stack[Gr, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Gr, $($queue,)*] @tail: > $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: > $($tail:tt)*) => (
    __op_internal__!(@stack[Gr, $($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: ( $($stuff:tt)* ) $($tail:tt)* )
 => (
    __op_internal__!(@stack[LParen, $($stack,)*] @queue[$($queue,)*]
                     @tail: $($stuff)* RParen $($tail)*)
);
(@stack[LParen, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*) => (
    __op_internal__!(@rp3 @stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: RParen $($tail:tt)*)
 => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: RParen $($tail)*)
);
(@rp3 @stack[Compare, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Compare, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Square, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Square, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Sqrt, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Sqrt, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[AbsVal, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[AbsVal, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Cube, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Cube, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Exp, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Exp, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Minimum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Minimum, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Maximum, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Maximum, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Log2, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Log2, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[Gcf, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[Gcf, $($queue,)*] @tail: $($tail)*)
);
(@rp3 @stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$($queue,)*] @tail: $($tail)*)
);
(@stack[$($stack:ident,)*] @queue[$($queue:ident,)*] @tail: $num:ident $($tail:tt)*) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$num, $($queue,)*] @tail: $($tail)*)
);
(@stack[] @queue[$($queue:ident,)*] @tail: ) => (
    __op_internal__!(@reverse[] @input: $($queue,)*)
);
(@stack[$stack_top:ident, $($stack:ident,)*] @queue[$($queue:ident,)*] @tail:) => (
    __op_internal__!(@stack[$($stack,)*] @queue[$stack_top, $($queue,)*] @tail: )
);
(@reverse[$($revved:ident,)*] @input: $head:ident, $($tail:ident,)* ) => (
    __op_internal__!(@reverse[$head, $($revved,)*] @input: $($tail,)*)
);
(@reverse[$($revved:ident,)*] @input: ) => (
    __op_internal__!(@eval @stack[] @input[$($revved,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Prod, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Prod<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Quot, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Quot<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Mod, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Mod<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Sum, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Sum<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Diff, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Diff<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Shleft, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Shleft<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Shright, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Shright<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[And, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::And<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Xor, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Xor<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Or, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Or<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Eq, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Eq<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[NotEq, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::NotEq<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[LeEq, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::LeEq<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[GrEq, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::GrEq<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Le, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Le<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Gr, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Gr<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Compare, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Compare<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Exp, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Exp<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Minimum, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Minimum<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Maximum, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Maximum<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $b:ty, $($stack:ty,)*] @input[Gcf, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Gcf<$b, $a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $($stack:ty,)*] @input[Square, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Square<$a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $($stack:ty,)*] @input[Sqrt, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Sqrt<$a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $($stack:ty,)*] @input[AbsVal, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::AbsVal<$a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $($stack:ty,)*] @input[Cube, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Cube<$a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$a:ty, $($stack:ty,)*] @input[Log2, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$crate::Log2<$a>, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$($stack:ty,)*] @input[$head:ident, $($tail:ident,)*]) => (
    __op_internal__!(@eval @stack[$head, $($stack,)*] @input[$($tail,)*])
);
(@eval @stack[$stack:ty,] @input[]) => (
    $stack
);
($($tail:tt)* ) => (
    __op_internal__!(@stack[] @queue[] @tail: $($tail)*)
);
}
 
/**
Type aliases for many constants.

This file is generated by typenum's build script.

For unsigned integers, the format is `U` followed by the number. We define aliases for

- Numbers 0 through 1024
- Powers of 2 below `u64::MAX`
- Powers of 10 below `u64::MAX`

These alias definitions look like this:

```rust
use typenum::{B0, B1, UInt, UTerm};

# #[allow(dead_code)]
type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
```

For positive signed integers, the format is `P` followed by the number and for negative
signed integers it is `N` followed by the number. For the signed integer zero, we use
`Z0`. We define aliases for

- Numbers -1024 through 1024
- Powers of 2 between `i64::MIN` and `i64::MAX`
- Powers of 10 between `i64::MIN` and `i64::MAX`

These alias definitions look like this:

```rust
use typenum::{B0, B1, UInt, UTerm, PInt, NInt};

# #[allow(dead_code)]
type P6 = PInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>>;
# #[allow(dead_code)]
type N6 = NInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>>;
```

# Example
```rust
# #[allow(unused_imports)]
use typenum::{U0, U1, U2, U3, U4, U5, U6};
# #[allow(unused_imports)]
use typenum::{N3, N2, N1, Z0, P1, P2, P3};
# #[allow(unused_imports)]
use typenum::{U774, N17, N10000, P1024, P4096};
```

We also define the aliases `False` and `True` for `B0` and `B1`, respectively.
*/
#[allow(missing_docs)]
pub mod consts {
    use uint::{UInt, UTerm};
    use int::{PInt, NInt};

    pub use bit::{B0, B1};
    pub use int::Z0;

    pub type True = B1;
    pub type False = B0;
    pub type U0 = UTerm;
    pub type U1 = UInt<UTerm, B1>;
    pub type P1 = PInt<U1>; pub type N1 = NInt<U1>;
    pub type U2 = UInt<UInt<UTerm, B1>, B0>;
    pub type P2 = PInt<U2>; pub type N2 = NInt<U2>;
    pub type U3 = UInt<UInt<UTerm, B1>, B1>;
    pub type P3 = PInt<U3>; pub type N3 = NInt<U3>;
    pub type U4 = UInt<UInt<UInt<UTerm, B1>, B0>, B0>;
    pub type P4 = PInt<U4>; pub type N4 = NInt<U4>;
    pub type U5 = UInt<UInt<UInt<UTerm, B1>, B0>, B1>;
    pub type P5 = PInt<U5>; pub type N5 = NInt<U5>;
    pub type U6 = UInt<UInt<UInt<UTerm, B1>, B1>, B0>;
    pub type P6 = PInt<U6>; pub type N6 = NInt<U6>;
    pub type U7 = UInt<UInt<UInt<UTerm, B1>, B1>, B1>;
    pub type P7 = PInt<U7>; pub type N7 = NInt<U7>;
    pub type U8 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>;
    pub type P8 = PInt<U8>; pub type N8 = NInt<U8>;
    pub type U9 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>;
    pub type P9 = PInt<U9>; pub type N9 = NInt<U9>;
    pub type U10 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>;
    pub type P10 = PInt<U10>; pub type N10 = NInt<U10>;
    pub type U11 = UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>;
    pub type P11 = PInt<U11>; pub type N11 = NInt<U11>;
    pub type U12 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>;
    pub type P12 = PInt<U12>; pub type N12 = NInt<U12>;
    pub type U13 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>;
    pub type P13 = PInt<U13>; pub type N13 = NInt<U13>;
    pub type U14 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>;
    pub type P14 = PInt<U14>; pub type N14 = NInt<U14>;
    pub type U15 = UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>;
    pub type P15 = PInt<U15>; pub type N15 = NInt<U15>;
    pub type U16 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>;
    pub type P16 = PInt<U16>; pub type N16 = NInt<U16>;
    pub type U17 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>;
    pub type P17 = PInt<U17>; pub type N17 = NInt<U17>;
    pub type U18 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>;
    pub type P18 = PInt<U18>; pub type N18 = NInt<U18>;
    pub type U19 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>;
    pub type P19 = PInt<U19>; pub type N19 = NInt<U19>;
    pub type U20 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>;
    pub type P20 = PInt<U20>; pub type N20 = NInt<U20>;
    pub type U21 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>;
    pub type P21 = PInt<U21>; pub type N21 = NInt<U21>;
    pub type U22 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>;
    pub type P22 = PInt<U22>; pub type N22 = NInt<U22>;
    pub type U23 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>;
    pub type P23 = PInt<U23>; pub type N23 = NInt<U23>;
    pub type U24 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>;
    pub type P24 = PInt<U24>; pub type N24 = NInt<U24>;
    pub type U25 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>;
    pub type P25 = PInt<U25>; pub type N25 = NInt<U25>;
    pub type U26 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>;
    pub type P26 = PInt<U26>; pub type N26 = NInt<U26>;
    pub type U27 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>;
    pub type P27 = PInt<U27>; pub type N27 = NInt<U27>;
    pub type U28 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>;
    pub type P28 = PInt<U28>; pub type N28 = NInt<U28>;
    pub type U29 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>;
    pub type P29 = PInt<U29>; pub type N29 = NInt<U29>;
    pub type U30 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>;
    pub type P30 = PInt<U30>; pub type N30 = NInt<U30>;
    pub type U31 = UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>;
    pub type P31 = PInt<U31>; pub type N31 = NInt<U31>;
    pub type U32 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P32 = PInt<U32>; pub type N32 = NInt<U32>;
    pub type U33 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P33 = PInt<U33>; pub type N33 = NInt<U33>;
    pub type U34 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P34 = PInt<U34>; pub type N34 = NInt<U34>;
    pub type U35 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P35 = PInt<U35>; pub type N35 = NInt<U35>;
    pub type U36 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P36 = PInt<U36>; pub type N36 = NInt<U36>;
    pub type U37 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P37 = PInt<U37>; pub type N37 = NInt<U37>;
    pub type U38 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P38 = PInt<U38>; pub type N38 = NInt<U38>;
    pub type U39 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P39 = PInt<U39>; pub type N39 = NInt<U39>;
    pub type U40 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P40 = PInt<U40>; pub type N40 = NInt<U40>;
    pub type U41 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P41 = PInt<U41>; pub type N41 = NInt<U41>;
    pub type U42 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P42 = PInt<U42>; pub type N42 = NInt<U42>;
    pub type U43 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P43 = PInt<U43>; pub type N43 = NInt<U43>;
    pub type U44 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P44 = PInt<U44>; pub type N44 = NInt<U44>;
    pub type U45 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P45 = PInt<U45>; pub type N45 = NInt<U45>;
    pub type U46 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P46 = PInt<U46>; pub type N46 = NInt<U46>;
    pub type U47 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P47 = PInt<U47>; pub type N47 = NInt<U47>;
    pub type U48 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P48 = PInt<U48>; pub type N48 = NInt<U48>;
    pub type U49 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P49 = PInt<U49>; pub type N49 = NInt<U49>;
    pub type U50 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P50 = PInt<U50>; pub type N50 = NInt<U50>;
    pub type U51 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P51 = PInt<U51>; pub type N51 = NInt<U51>;
    pub type U52 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P52 = PInt<U52>; pub type N52 = NInt<U52>;
    pub type U53 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P53 = PInt<U53>; pub type N53 = NInt<U53>;
    pub type U54 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P54 = PInt<U54>; pub type N54 = NInt<U54>;
    pub type U55 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P55 = PInt<U55>; pub type N55 = NInt<U55>;
    pub type U56 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P56 = PInt<U56>; pub type N56 = NInt<U56>;
    pub type U57 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P57 = PInt<U57>; pub type N57 = NInt<U57>;
    pub type U58 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P58 = PInt<U58>; pub type N58 = NInt<U58>;
    pub type U59 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P59 = PInt<U59>; pub type N59 = NInt<U59>;
    pub type U60 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P60 = PInt<U60>; pub type N60 = NInt<U60>;
    pub type U61 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P61 = PInt<U61>; pub type N61 = NInt<U61>;
    pub type U62 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P62 = PInt<U62>; pub type N62 = NInt<U62>;
    pub type U63 = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P63 = PInt<U63>; pub type N63 = NInt<U63>;
    pub type U64 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P64 = PInt<U64>; pub type N64 = NInt<U64>;
    pub type U65 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P65 = PInt<U65>; pub type N65 = NInt<U65>;
    pub type U66 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P66 = PInt<U66>; pub type N66 = NInt<U66>;
    pub type U67 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P67 = PInt<U67>; pub type N67 = NInt<U67>;
    pub type U68 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P68 = PInt<U68>; pub type N68 = NInt<U68>;
    pub type U69 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P69 = PInt<U69>; pub type N69 = NInt<U69>;
    pub type U70 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P70 = PInt<U70>; pub type N70 = NInt<U70>;
    pub type U71 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P71 = PInt<U71>; pub type N71 = NInt<U71>;
    pub type U72 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P72 = PInt<U72>; pub type N72 = NInt<U72>;
    pub type U73 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P73 = PInt<U73>; pub type N73 = NInt<U73>;
    pub type U74 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P74 = PInt<U74>; pub type N74 = NInt<U74>;
    pub type U75 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P75 = PInt<U75>; pub type N75 = NInt<U75>;
    pub type U76 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P76 = PInt<U76>; pub type N76 = NInt<U76>;
    pub type U77 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P77 = PInt<U77>; pub type N77 = NInt<U77>;
    pub type U78 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P78 = PInt<U78>; pub type N78 = NInt<U78>;
    pub type U79 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P79 = PInt<U79>; pub type N79 = NInt<U79>;
    pub type U80 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P80 = PInt<U80>; pub type N80 = NInt<U80>;
    pub type U81 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P81 = PInt<U81>; pub type N81 = NInt<U81>;
    pub type U82 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P82 = PInt<U82>; pub type N82 = NInt<U82>;
    pub type U83 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P83 = PInt<U83>; pub type N83 = NInt<U83>;
    pub type U84 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P84 = PInt<U84>; pub type N84 = NInt<U84>;
    pub type U85 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P85 = PInt<U85>; pub type N85 = NInt<U85>;
    pub type U86 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P86 = PInt<U86>; pub type N86 = NInt<U86>;
    pub type U87 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P87 = PInt<U87>; pub type N87 = NInt<U87>;
    pub type U88 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P88 = PInt<U88>; pub type N88 = NInt<U88>;
    pub type U89 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P89 = PInt<U89>; pub type N89 = NInt<U89>;
    pub type U90 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P90 = PInt<U90>; pub type N90 = NInt<U90>;
    pub type U91 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P91 = PInt<U91>; pub type N91 = NInt<U91>;
    pub type U92 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P92 = PInt<U92>; pub type N92 = NInt<U92>;
    pub type U93 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P93 = PInt<U93>; pub type N93 = NInt<U93>;
    pub type U94 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P94 = PInt<U94>; pub type N94 = NInt<U94>;
    pub type U95 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P95 = PInt<U95>; pub type N95 = NInt<U95>;
    pub type U96 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P96 = PInt<U96>; pub type N96 = NInt<U96>;
    pub type U97 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P97 = PInt<U97>; pub type N97 = NInt<U97>;
    pub type U98 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P98 = PInt<U98>; pub type N98 = NInt<U98>;
    pub type U99 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P99 = PInt<U99>; pub type N99 = NInt<U99>;
    pub type U100 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P100 = PInt<U100>; pub type N100 = NInt<U100>;
    pub type U101 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P101 = PInt<U101>; pub type N101 = NInt<U101>;
    pub type U102 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P102 = PInt<U102>; pub type N102 = NInt<U102>;
    pub type U103 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P103 = PInt<U103>; pub type N103 = NInt<U103>;
    pub type U104 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P104 = PInt<U104>; pub type N104 = NInt<U104>;
    pub type U105 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P105 = PInt<U105>; pub type N105 = NInt<U105>;
    pub type U106 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P106 = PInt<U106>; pub type N106 = NInt<U106>;
    pub type U107 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P107 = PInt<U107>; pub type N107 = NInt<U107>;
    pub type U108 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P108 = PInt<U108>; pub type N108 = NInt<U108>;
    pub type U109 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P109 = PInt<U109>; pub type N109 = NInt<U109>;
    pub type U110 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P110 = PInt<U110>; pub type N110 = NInt<U110>;
    pub type U111 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P111 = PInt<U111>; pub type N111 = NInt<U111>;
    pub type U112 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P112 = PInt<U112>; pub type N112 = NInt<U112>;
    pub type U113 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P113 = PInt<U113>; pub type N113 = NInt<U113>;
    pub type U114 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P114 = PInt<U114>; pub type N114 = NInt<U114>;
    pub type U115 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P115 = PInt<U115>; pub type N115 = NInt<U115>;
    pub type U116 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P116 = PInt<U116>; pub type N116 = NInt<U116>;
    pub type U117 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P117 = PInt<U117>; pub type N117 = NInt<U117>;
    pub type U118 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P118 = PInt<U118>; pub type N118 = NInt<U118>;
    pub type U119 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P119 = PInt<U119>; pub type N119 = NInt<U119>;
    pub type U120 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P120 = PInt<U120>; pub type N120 = NInt<U120>;
    pub type U121 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P121 = PInt<U121>; pub type N121 = NInt<U121>;
    pub type U122 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P122 = PInt<U122>; pub type N122 = NInt<U122>;
    pub type U123 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P123 = PInt<U123>; pub type N123 = NInt<U123>;
    pub type U124 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P124 = PInt<U124>; pub type N124 = NInt<U124>;
    pub type U125 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P125 = PInt<U125>; pub type N125 = NInt<U125>;
    pub type U126 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P126 = PInt<U126>; pub type N126 = NInt<U126>;
    pub type U127 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P127 = PInt<U127>; pub type N127 = NInt<U127>;
    pub type U128 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P128 = PInt<U128>; pub type N128 = NInt<U128>;
    pub type U129 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P129 = PInt<U129>; pub type N129 = NInt<U129>;
    pub type U130 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P130 = PInt<U130>; pub type N130 = NInt<U130>;
    pub type U131 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P131 = PInt<U131>; pub type N131 = NInt<U131>;
    pub type U132 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P132 = PInt<U132>; pub type N132 = NInt<U132>;
    pub type U133 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P133 = PInt<U133>; pub type N133 = NInt<U133>;
    pub type U134 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P134 = PInt<U134>; pub type N134 = NInt<U134>;
    pub type U135 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P135 = PInt<U135>; pub type N135 = NInt<U135>;
    pub type U136 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P136 = PInt<U136>; pub type N136 = NInt<U136>;
    pub type U137 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P137 = PInt<U137>; pub type N137 = NInt<U137>;
    pub type U138 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P138 = PInt<U138>; pub type N138 = NInt<U138>;
    pub type U139 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P139 = PInt<U139>; pub type N139 = NInt<U139>;
    pub type U140 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P140 = PInt<U140>; pub type N140 = NInt<U140>;
    pub type U141 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P141 = PInt<U141>; pub type N141 = NInt<U141>;
    pub type U142 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P142 = PInt<U142>; pub type N142 = NInt<U142>;
    pub type U143 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P143 = PInt<U143>; pub type N143 = NInt<U143>;
    pub type U144 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P144 = PInt<U144>; pub type N144 = NInt<U144>;
    pub type U145 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P145 = PInt<U145>; pub type N145 = NInt<U145>;
    pub type U146 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P146 = PInt<U146>; pub type N146 = NInt<U146>;
    pub type U147 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P147 = PInt<U147>; pub type N147 = NInt<U147>;
    pub type U148 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P148 = PInt<U148>; pub type N148 = NInt<U148>;
    pub type U149 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P149 = PInt<U149>; pub type N149 = NInt<U149>;
    pub type U150 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P150 = PInt<U150>; pub type N150 = NInt<U150>;
    pub type U151 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P151 = PInt<U151>; pub type N151 = NInt<U151>;
    pub type U152 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P152 = PInt<U152>; pub type N152 = NInt<U152>;
    pub type U153 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P153 = PInt<U153>; pub type N153 = NInt<U153>;
    pub type U154 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P154 = PInt<U154>; pub type N154 = NInt<U154>;
    pub type U155 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P155 = PInt<U155>; pub type N155 = NInt<U155>;
    pub type U156 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P156 = PInt<U156>; pub type N156 = NInt<U156>;
    pub type U157 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P157 = PInt<U157>; pub type N157 = NInt<U157>;
    pub type U158 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P158 = PInt<U158>; pub type N158 = NInt<U158>;
    pub type U159 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P159 = PInt<U159>; pub type N159 = NInt<U159>;
    pub type U160 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P160 = PInt<U160>; pub type N160 = NInt<U160>;
    pub type U161 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P161 = PInt<U161>; pub type N161 = NInt<U161>;
    pub type U162 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P162 = PInt<U162>; pub type N162 = NInt<U162>;
    pub type U163 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P163 = PInt<U163>; pub type N163 = NInt<U163>;
    pub type U164 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P164 = PInt<U164>; pub type N164 = NInt<U164>;
    pub type U165 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P165 = PInt<U165>; pub type N165 = NInt<U165>;
    pub type U166 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P166 = PInt<U166>; pub type N166 = NInt<U166>;
    pub type U167 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P167 = PInt<U167>; pub type N167 = NInt<U167>;
    pub type U168 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P168 = PInt<U168>; pub type N168 = NInt<U168>;
    pub type U169 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P169 = PInt<U169>; pub type N169 = NInt<U169>;
    pub type U170 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P170 = PInt<U170>; pub type N170 = NInt<U170>;
    pub type U171 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P171 = PInt<U171>; pub type N171 = NInt<U171>;
    pub type U172 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P172 = PInt<U172>; pub type N172 = NInt<U172>;
    pub type U173 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P173 = PInt<U173>; pub type N173 = NInt<U173>;
    pub type U174 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P174 = PInt<U174>; pub type N174 = NInt<U174>;
    pub type U175 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P175 = PInt<U175>; pub type N175 = NInt<U175>;
    pub type U176 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P176 = PInt<U176>; pub type N176 = NInt<U176>;
    pub type U177 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P177 = PInt<U177>; pub type N177 = NInt<U177>;
    pub type U178 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P178 = PInt<U178>; pub type N178 = NInt<U178>;
    pub type U179 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P179 = PInt<U179>; pub type N179 = NInt<U179>;
    pub type U180 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P180 = PInt<U180>; pub type N180 = NInt<U180>;
    pub type U181 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P181 = PInt<U181>; pub type N181 = NInt<U181>;
    pub type U182 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P182 = PInt<U182>; pub type N182 = NInt<U182>;
    pub type U183 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P183 = PInt<U183>; pub type N183 = NInt<U183>;
    pub type U184 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P184 = PInt<U184>; pub type N184 = NInt<U184>;
    pub type U185 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P185 = PInt<U185>; pub type N185 = NInt<U185>;
    pub type U186 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P186 = PInt<U186>; pub type N186 = NInt<U186>;
    pub type U187 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P187 = PInt<U187>; pub type N187 = NInt<U187>;
    pub type U188 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P188 = PInt<U188>; pub type N188 = NInt<U188>;
    pub type U189 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P189 = PInt<U189>; pub type N189 = NInt<U189>;
    pub type U190 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P190 = PInt<U190>; pub type N190 = NInt<U190>;
    pub type U191 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P191 = PInt<U191>; pub type N191 = NInt<U191>;
    pub type U192 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P192 = PInt<U192>; pub type N192 = NInt<U192>;
    pub type U193 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P193 = PInt<U193>; pub type N193 = NInt<U193>;
    pub type U194 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P194 = PInt<U194>; pub type N194 = NInt<U194>;
    pub type U195 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P195 = PInt<U195>; pub type N195 = NInt<U195>;
    pub type U196 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P196 = PInt<U196>; pub type N196 = NInt<U196>;
    pub type U197 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P197 = PInt<U197>; pub type N197 = NInt<U197>;
    pub type U198 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P198 = PInt<U198>; pub type N198 = NInt<U198>;
    pub type U199 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P199 = PInt<U199>; pub type N199 = NInt<U199>;
    pub type U200 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P200 = PInt<U200>; pub type N200 = NInt<U200>;
    pub type U201 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P201 = PInt<U201>; pub type N201 = NInt<U201>;
    pub type U202 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P202 = PInt<U202>; pub type N202 = NInt<U202>;
    pub type U203 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P203 = PInt<U203>; pub type N203 = NInt<U203>;
    pub type U204 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P204 = PInt<U204>; pub type N204 = NInt<U204>;
    pub type U205 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P205 = PInt<U205>; pub type N205 = NInt<U205>;
    pub type U206 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P206 = PInt<U206>; pub type N206 = NInt<U206>;
    pub type U207 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P207 = PInt<U207>; pub type N207 = NInt<U207>;
    pub type U208 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P208 = PInt<U208>; pub type N208 = NInt<U208>;
    pub type U209 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P209 = PInt<U209>; pub type N209 = NInt<U209>;
    pub type U210 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P210 = PInt<U210>; pub type N210 = NInt<U210>;
    pub type U211 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P211 = PInt<U211>; pub type N211 = NInt<U211>;
    pub type U212 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P212 = PInt<U212>; pub type N212 = NInt<U212>;
    pub type U213 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P213 = PInt<U213>; pub type N213 = NInt<U213>;
    pub type U214 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P214 = PInt<U214>; pub type N214 = NInt<U214>;
    pub type U215 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P215 = PInt<U215>; pub type N215 = NInt<U215>;
    pub type U216 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P216 = PInt<U216>; pub type N216 = NInt<U216>;
    pub type U217 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P217 = PInt<U217>; pub type N217 = NInt<U217>;
    pub type U218 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P218 = PInt<U218>; pub type N218 = NInt<U218>;
    pub type U219 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P219 = PInt<U219>; pub type N219 = NInt<U219>;
    pub type U220 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P220 = PInt<U220>; pub type N220 = NInt<U220>;
    pub type U221 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P221 = PInt<U221>; pub type N221 = NInt<U221>;
    pub type U222 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P222 = PInt<U222>; pub type N222 = NInt<U222>;
    pub type U223 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P223 = PInt<U223>; pub type N223 = NInt<U223>;
    pub type U224 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P224 = PInt<U224>; pub type N224 = NInt<U224>;
    pub type U225 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P225 = PInt<U225>; pub type N225 = NInt<U225>;
    pub type U226 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P226 = PInt<U226>; pub type N226 = NInt<U226>;
    pub type U227 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P227 = PInt<U227>; pub type N227 = NInt<U227>;
    pub type U228 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P228 = PInt<U228>; pub type N228 = NInt<U228>;
    pub type U229 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P229 = PInt<U229>; pub type N229 = NInt<U229>;
    pub type U230 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P230 = PInt<U230>; pub type N230 = NInt<U230>;
    pub type U231 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P231 = PInt<U231>; pub type N231 = NInt<U231>;
    pub type U232 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P232 = PInt<U232>; pub type N232 = NInt<U232>;
    pub type U233 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P233 = PInt<U233>; pub type N233 = NInt<U233>;
    pub type U234 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P234 = PInt<U234>; pub type N234 = NInt<U234>;
    pub type U235 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P235 = PInt<U235>; pub type N235 = NInt<U235>;
    pub type U236 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P236 = PInt<U236>; pub type N236 = NInt<U236>;
    pub type U237 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P237 = PInt<U237>; pub type N237 = NInt<U237>;
    pub type U238 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P238 = PInt<U238>; pub type N238 = NInt<U238>;
    pub type U239 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P239 = PInt<U239>; pub type N239 = NInt<U239>;
    pub type U240 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P240 = PInt<U240>; pub type N240 = NInt<U240>;
    pub type U241 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P241 = PInt<U241>; pub type N241 = NInt<U241>;
    pub type U242 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P242 = PInt<U242>; pub type N242 = NInt<U242>;
    pub type U243 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P243 = PInt<U243>; pub type N243 = NInt<U243>;
    pub type U244 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P244 = PInt<U244>; pub type N244 = NInt<U244>;
    pub type U245 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P245 = PInt<U245>; pub type N245 = NInt<U245>;
    pub type U246 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P246 = PInt<U246>; pub type N246 = NInt<U246>;
    pub type U247 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P247 = PInt<U247>; pub type N247 = NInt<U247>;
    pub type U248 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P248 = PInt<U248>; pub type N248 = NInt<U248>;
    pub type U249 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P249 = PInt<U249>; pub type N249 = NInt<U249>;
    pub type U250 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P250 = PInt<U250>; pub type N250 = NInt<U250>;
    pub type U251 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P251 = PInt<U251>; pub type N251 = NInt<U251>;
    pub type U252 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P252 = PInt<U252>; pub type N252 = NInt<U252>;
    pub type U253 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P253 = PInt<U253>; pub type N253 = NInt<U253>;
    pub type U254 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P254 = PInt<U254>; pub type N254 = NInt<U254>;
    pub type U255 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P255 = PInt<U255>; pub type N255 = NInt<U255>;
    pub type U256 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P256 = PInt<U256>; pub type N256 = NInt<U256>;
    pub type U257 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P257 = PInt<U257>; pub type N257 = NInt<U257>;
    pub type U258 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P258 = PInt<U258>; pub type N258 = NInt<U258>;
    pub type U259 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P259 = PInt<U259>; pub type N259 = NInt<U259>;
    pub type U260 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P260 = PInt<U260>; pub type N260 = NInt<U260>;
    pub type U261 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P261 = PInt<U261>; pub type N261 = NInt<U261>;
    pub type U262 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P262 = PInt<U262>; pub type N262 = NInt<U262>;
    pub type U263 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P263 = PInt<U263>; pub type N263 = NInt<U263>;
    pub type U264 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P264 = PInt<U264>; pub type N264 = NInt<U264>;
    pub type U265 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P265 = PInt<U265>; pub type N265 = NInt<U265>;
    pub type U266 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P266 = PInt<U266>; pub type N266 = NInt<U266>;
    pub type U267 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P267 = PInt<U267>; pub type N267 = NInt<U267>;
    pub type U268 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P268 = PInt<U268>; pub type N268 = NInt<U268>;
    pub type U269 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P269 = PInt<U269>; pub type N269 = NInt<U269>;
    pub type U270 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P270 = PInt<U270>; pub type N270 = NInt<U270>;
    pub type U271 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P271 = PInt<U271>; pub type N271 = NInt<U271>;
    pub type U272 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P272 = PInt<U272>; pub type N272 = NInt<U272>;
    pub type U273 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P273 = PInt<U273>; pub type N273 = NInt<U273>;
    pub type U274 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P274 = PInt<U274>; pub type N274 = NInt<U274>;
    pub type U275 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P275 = PInt<U275>; pub type N275 = NInt<U275>;
    pub type U276 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P276 = PInt<U276>; pub type N276 = NInt<U276>;
    pub type U277 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P277 = PInt<U277>; pub type N277 = NInt<U277>;
    pub type U278 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P278 = PInt<U278>; pub type N278 = NInt<U278>;
    pub type U279 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P279 = PInt<U279>; pub type N279 = NInt<U279>;
    pub type U280 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P280 = PInt<U280>; pub type N280 = NInt<U280>;
    pub type U281 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P281 = PInt<U281>; pub type N281 = NInt<U281>;
    pub type U282 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P282 = PInt<U282>; pub type N282 = NInt<U282>;
    pub type U283 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P283 = PInt<U283>; pub type N283 = NInt<U283>;
    pub type U284 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P284 = PInt<U284>; pub type N284 = NInt<U284>;
    pub type U285 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P285 = PInt<U285>; pub type N285 = NInt<U285>;
    pub type U286 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P286 = PInt<U286>; pub type N286 = NInt<U286>;
    pub type U287 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P287 = PInt<U287>; pub type N287 = NInt<U287>;
    pub type U288 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P288 = PInt<U288>; pub type N288 = NInt<U288>;
    pub type U289 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P289 = PInt<U289>; pub type N289 = NInt<U289>;
    pub type U290 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P290 = PInt<U290>; pub type N290 = NInt<U290>;
    pub type U291 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P291 = PInt<U291>; pub type N291 = NInt<U291>;
    pub type U292 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P292 = PInt<U292>; pub type N292 = NInt<U292>;
    pub type U293 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P293 = PInt<U293>; pub type N293 = NInt<U293>;
    pub type U294 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P294 = PInt<U294>; pub type N294 = NInt<U294>;
    pub type U295 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P295 = PInt<U295>; pub type N295 = NInt<U295>;
    pub type U296 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P296 = PInt<U296>; pub type N296 = NInt<U296>;
    pub type U297 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P297 = PInt<U297>; pub type N297 = NInt<U297>;
    pub type U298 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P298 = PInt<U298>; pub type N298 = NInt<U298>;
    pub type U299 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P299 = PInt<U299>; pub type N299 = NInt<U299>;
    pub type U300 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P300 = PInt<U300>; pub type N300 = NInt<U300>;
    pub type U301 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P301 = PInt<U301>; pub type N301 = NInt<U301>;
    pub type U302 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P302 = PInt<U302>; pub type N302 = NInt<U302>;
    pub type U303 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P303 = PInt<U303>; pub type N303 = NInt<U303>;
    pub type U304 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P304 = PInt<U304>; pub type N304 = NInt<U304>;
    pub type U305 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P305 = PInt<U305>; pub type N305 = NInt<U305>;
    pub type U306 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P306 = PInt<U306>; pub type N306 = NInt<U306>;
    pub type U307 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P307 = PInt<U307>; pub type N307 = NInt<U307>;
    pub type U308 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P308 = PInt<U308>; pub type N308 = NInt<U308>;
    pub type U309 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P309 = PInt<U309>; pub type N309 = NInt<U309>;
    pub type U310 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P310 = PInt<U310>; pub type N310 = NInt<U310>;
    pub type U311 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P311 = PInt<U311>; pub type N311 = NInt<U311>;
    pub type U312 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P312 = PInt<U312>; pub type N312 = NInt<U312>;
    pub type U313 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P313 = PInt<U313>; pub type N313 = NInt<U313>;
    pub type U314 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P314 = PInt<U314>; pub type N314 = NInt<U314>;
    pub type U315 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P315 = PInt<U315>; pub type N315 = NInt<U315>;
    pub type U316 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P316 = PInt<U316>; pub type N316 = NInt<U316>;
    pub type U317 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P317 = PInt<U317>; pub type N317 = NInt<U317>;
    pub type U318 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P318 = PInt<U318>; pub type N318 = NInt<U318>;
    pub type U319 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P319 = PInt<U319>; pub type N319 = NInt<U319>;
    pub type U320 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P320 = PInt<U320>; pub type N320 = NInt<U320>;
    pub type U321 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P321 = PInt<U321>; pub type N321 = NInt<U321>;
    pub type U322 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P322 = PInt<U322>; pub type N322 = NInt<U322>;
    pub type U323 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P323 = PInt<U323>; pub type N323 = NInt<U323>;
    pub type U324 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P324 = PInt<U324>; pub type N324 = NInt<U324>;
    pub type U325 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P325 = PInt<U325>; pub type N325 = NInt<U325>;
    pub type U326 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P326 = PInt<U326>; pub type N326 = NInt<U326>;
    pub type U327 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P327 = PInt<U327>; pub type N327 = NInt<U327>;
    pub type U328 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P328 = PInt<U328>; pub type N328 = NInt<U328>;
    pub type U329 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P329 = PInt<U329>; pub type N329 = NInt<U329>;
    pub type U330 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P330 = PInt<U330>; pub type N330 = NInt<U330>;
    pub type U331 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P331 = PInt<U331>; pub type N331 = NInt<U331>;
    pub type U332 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P332 = PInt<U332>; pub type N332 = NInt<U332>;
    pub type U333 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P333 = PInt<U333>; pub type N333 = NInt<U333>;
    pub type U334 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P334 = PInt<U334>; pub type N334 = NInt<U334>;
    pub type U335 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P335 = PInt<U335>; pub type N335 = NInt<U335>;
    pub type U336 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P336 = PInt<U336>; pub type N336 = NInt<U336>;
    pub type U337 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P337 = PInt<U337>; pub type N337 = NInt<U337>;
    pub type U338 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P338 = PInt<U338>; pub type N338 = NInt<U338>;
    pub type U339 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P339 = PInt<U339>; pub type N339 = NInt<U339>;
    pub type U340 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P340 = PInt<U340>; pub type N340 = NInt<U340>;
    pub type U341 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P341 = PInt<U341>; pub type N341 = NInt<U341>;
    pub type U342 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P342 = PInt<U342>; pub type N342 = NInt<U342>;
    pub type U343 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P343 = PInt<U343>; pub type N343 = NInt<U343>;
    pub type U344 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P344 = PInt<U344>; pub type N344 = NInt<U344>;
    pub type U345 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P345 = PInt<U345>; pub type N345 = NInt<U345>;
    pub type U346 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P346 = PInt<U346>; pub type N346 = NInt<U346>;
    pub type U347 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P347 = PInt<U347>; pub type N347 = NInt<U347>;
    pub type U348 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P348 = PInt<U348>; pub type N348 = NInt<U348>;
    pub type U349 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P349 = PInt<U349>; pub type N349 = NInt<U349>;
    pub type U350 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P350 = PInt<U350>; pub type N350 = NInt<U350>;
    pub type U351 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P351 = PInt<U351>; pub type N351 = NInt<U351>;
    pub type U352 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P352 = PInt<U352>; pub type N352 = NInt<U352>;
    pub type U353 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P353 = PInt<U353>; pub type N353 = NInt<U353>;
    pub type U354 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P354 = PInt<U354>; pub type N354 = NInt<U354>;
    pub type U355 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P355 = PInt<U355>; pub type N355 = NInt<U355>;
    pub type U356 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P356 = PInt<U356>; pub type N356 = NInt<U356>;
    pub type U357 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P357 = PInt<U357>; pub type N357 = NInt<U357>;
    pub type U358 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P358 = PInt<U358>; pub type N358 = NInt<U358>;
    pub type U359 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P359 = PInt<U359>; pub type N359 = NInt<U359>;
    pub type U360 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P360 = PInt<U360>; pub type N360 = NInt<U360>;
    pub type U361 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P361 = PInt<U361>; pub type N361 = NInt<U361>;
    pub type U362 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P362 = PInt<U362>; pub type N362 = NInt<U362>;
    pub type U363 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P363 = PInt<U363>; pub type N363 = NInt<U363>;
    pub type U364 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P364 = PInt<U364>; pub type N364 = NInt<U364>;
    pub type U365 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P365 = PInt<U365>; pub type N365 = NInt<U365>;
    pub type U366 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P366 = PInt<U366>; pub type N366 = NInt<U366>;
    pub type U367 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P367 = PInt<U367>; pub type N367 = NInt<U367>;
    pub type U368 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P368 = PInt<U368>; pub type N368 = NInt<U368>;
    pub type U369 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P369 = PInt<U369>; pub type N369 = NInt<U369>;
    pub type U370 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P370 = PInt<U370>; pub type N370 = NInt<U370>;
    pub type U371 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P371 = PInt<U371>; pub type N371 = NInt<U371>;
    pub type U372 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P372 = PInt<U372>; pub type N372 = NInt<U372>;
    pub type U373 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P373 = PInt<U373>; pub type N373 = NInt<U373>;
    pub type U374 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P374 = PInt<U374>; pub type N374 = NInt<U374>;
    pub type U375 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P375 = PInt<U375>; pub type N375 = NInt<U375>;
    pub type U376 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P376 = PInt<U376>; pub type N376 = NInt<U376>;
    pub type U377 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P377 = PInt<U377>; pub type N377 = NInt<U377>;
    pub type U378 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P378 = PInt<U378>; pub type N378 = NInt<U378>;
    pub type U379 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P379 = PInt<U379>; pub type N379 = NInt<U379>;
    pub type U380 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P380 = PInt<U380>; pub type N380 = NInt<U380>;
    pub type U381 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P381 = PInt<U381>; pub type N381 = NInt<U381>;
    pub type U382 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P382 = PInt<U382>; pub type N382 = NInt<U382>;
    pub type U383 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P383 = PInt<U383>; pub type N383 = NInt<U383>;
    pub type U384 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P384 = PInt<U384>; pub type N384 = NInt<U384>;
    pub type U385 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P385 = PInt<U385>; pub type N385 = NInt<U385>;
    pub type U386 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P386 = PInt<U386>; pub type N386 = NInt<U386>;
    pub type U387 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P387 = PInt<U387>; pub type N387 = NInt<U387>;
    pub type U388 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P388 = PInt<U388>; pub type N388 = NInt<U388>;
    pub type U389 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P389 = PInt<U389>; pub type N389 = NInt<U389>;
    pub type U390 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P390 = PInt<U390>; pub type N390 = NInt<U390>;
    pub type U391 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P391 = PInt<U391>; pub type N391 = NInt<U391>;
    pub type U392 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P392 = PInt<U392>; pub type N392 = NInt<U392>;
    pub type U393 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P393 = PInt<U393>; pub type N393 = NInt<U393>;
    pub type U394 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P394 = PInt<U394>; pub type N394 = NInt<U394>;
    pub type U395 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P395 = PInt<U395>; pub type N395 = NInt<U395>;
    pub type U396 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P396 = PInt<U396>; pub type N396 = NInt<U396>;
    pub type U397 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P397 = PInt<U397>; pub type N397 = NInt<U397>;
    pub type U398 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P398 = PInt<U398>; pub type N398 = NInt<U398>;
    pub type U399 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P399 = PInt<U399>; pub type N399 = NInt<U399>;
    pub type U400 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P400 = PInt<U400>; pub type N400 = NInt<U400>;
    pub type U401 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P401 = PInt<U401>; pub type N401 = NInt<U401>;
    pub type U402 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P402 = PInt<U402>; pub type N402 = NInt<U402>;
    pub type U403 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P403 = PInt<U403>; pub type N403 = NInt<U403>;
    pub type U404 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P404 = PInt<U404>; pub type N404 = NInt<U404>;
    pub type U405 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P405 = PInt<U405>; pub type N405 = NInt<U405>;
    pub type U406 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P406 = PInt<U406>; pub type N406 = NInt<U406>;
    pub type U407 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P407 = PInt<U407>; pub type N407 = NInt<U407>;
    pub type U408 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P408 = PInt<U408>; pub type N408 = NInt<U408>;
    pub type U409 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P409 = PInt<U409>; pub type N409 = NInt<U409>;
    pub type U410 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P410 = PInt<U410>; pub type N410 = NInt<U410>;
    pub type U411 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P411 = PInt<U411>; pub type N411 = NInt<U411>;
    pub type U412 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P412 = PInt<U412>; pub type N412 = NInt<U412>;
    pub type U413 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P413 = PInt<U413>; pub type N413 = NInt<U413>;
    pub type U414 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P414 = PInt<U414>; pub type N414 = NInt<U414>;
    pub type U415 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P415 = PInt<U415>; pub type N415 = NInt<U415>;
    pub type U416 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P416 = PInt<U416>; pub type N416 = NInt<U416>;
    pub type U417 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P417 = PInt<U417>; pub type N417 = NInt<U417>;
    pub type U418 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P418 = PInt<U418>; pub type N418 = NInt<U418>;
    pub type U419 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P419 = PInt<U419>; pub type N419 = NInt<U419>;
    pub type U420 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P420 = PInt<U420>; pub type N420 = NInt<U420>;
    pub type U421 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P421 = PInt<U421>; pub type N421 = NInt<U421>;
    pub type U422 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P422 = PInt<U422>; pub type N422 = NInt<U422>;
    pub type U423 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P423 = PInt<U423>; pub type N423 = NInt<U423>;
    pub type U424 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P424 = PInt<U424>; pub type N424 = NInt<U424>;
    pub type U425 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P425 = PInt<U425>; pub type N425 = NInt<U425>;
    pub type U426 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P426 = PInt<U426>; pub type N426 = NInt<U426>;
    pub type U427 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P427 = PInt<U427>; pub type N427 = NInt<U427>;
    pub type U428 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P428 = PInt<U428>; pub type N428 = NInt<U428>;
    pub type U429 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P429 = PInt<U429>; pub type N429 = NInt<U429>;
    pub type U430 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P430 = PInt<U430>; pub type N430 = NInt<U430>;
    pub type U431 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P431 = PInt<U431>; pub type N431 = NInt<U431>;
    pub type U432 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P432 = PInt<U432>; pub type N432 = NInt<U432>;
    pub type U433 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P433 = PInt<U433>; pub type N433 = NInt<U433>;
    pub type U434 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P434 = PInt<U434>; pub type N434 = NInt<U434>;
    pub type U435 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P435 = PInt<U435>; pub type N435 = NInt<U435>;
    pub type U436 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P436 = PInt<U436>; pub type N436 = NInt<U436>;
    pub type U437 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P437 = PInt<U437>; pub type N437 = NInt<U437>;
    pub type U438 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P438 = PInt<U438>; pub type N438 = NInt<U438>;
    pub type U439 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P439 = PInt<U439>; pub type N439 = NInt<U439>;
    pub type U440 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P440 = PInt<U440>; pub type N440 = NInt<U440>;
    pub type U441 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P441 = PInt<U441>; pub type N441 = NInt<U441>;
    pub type U442 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P442 = PInt<U442>; pub type N442 = NInt<U442>;
    pub type U443 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P443 = PInt<U443>; pub type N443 = NInt<U443>;
    pub type U444 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P444 = PInt<U444>; pub type N444 = NInt<U444>;
    pub type U445 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P445 = PInt<U445>; pub type N445 = NInt<U445>;
    pub type U446 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P446 = PInt<U446>; pub type N446 = NInt<U446>;
    pub type U447 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P447 = PInt<U447>; pub type N447 = NInt<U447>;
    pub type U448 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P448 = PInt<U448>; pub type N448 = NInt<U448>;
    pub type U449 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P449 = PInt<U449>; pub type N449 = NInt<U449>;
    pub type U450 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P450 = PInt<U450>; pub type N450 = NInt<U450>;
    pub type U451 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P451 = PInt<U451>; pub type N451 = NInt<U451>;
    pub type U452 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P452 = PInt<U452>; pub type N452 = NInt<U452>;
    pub type U453 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P453 = PInt<U453>; pub type N453 = NInt<U453>;
    pub type U454 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P454 = PInt<U454>; pub type N454 = NInt<U454>;
    pub type U455 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P455 = PInt<U455>; pub type N455 = NInt<U455>;
    pub type U456 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P456 = PInt<U456>; pub type N456 = NInt<U456>;
    pub type U457 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P457 = PInt<U457>; pub type N457 = NInt<U457>;
    pub type U458 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P458 = PInt<U458>; pub type N458 = NInt<U458>;
    pub type U459 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P459 = PInt<U459>; pub type N459 = NInt<U459>;
    pub type U460 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P460 = PInt<U460>; pub type N460 = NInt<U460>;
    pub type U461 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P461 = PInt<U461>; pub type N461 = NInt<U461>;
    pub type U462 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P462 = PInt<U462>; pub type N462 = NInt<U462>;
    pub type U463 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P463 = PInt<U463>; pub type N463 = NInt<U463>;
    pub type U464 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P464 = PInt<U464>; pub type N464 = NInt<U464>;
    pub type U465 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P465 = PInt<U465>; pub type N465 = NInt<U465>;
    pub type U466 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P466 = PInt<U466>; pub type N466 = NInt<U466>;
    pub type U467 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P467 = PInt<U467>; pub type N467 = NInt<U467>;
    pub type U468 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P468 = PInt<U468>; pub type N468 = NInt<U468>;
    pub type U469 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P469 = PInt<U469>; pub type N469 = NInt<U469>;
    pub type U470 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P470 = PInt<U470>; pub type N470 = NInt<U470>;
    pub type U471 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P471 = PInt<U471>; pub type N471 = NInt<U471>;
    pub type U472 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P472 = PInt<U472>; pub type N472 = NInt<U472>;
    pub type U473 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P473 = PInt<U473>; pub type N473 = NInt<U473>;
    pub type U474 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P474 = PInt<U474>; pub type N474 = NInt<U474>;
    pub type U475 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P475 = PInt<U475>; pub type N475 = NInt<U475>;
    pub type U476 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P476 = PInt<U476>; pub type N476 = NInt<U476>;
    pub type U477 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P477 = PInt<U477>; pub type N477 = NInt<U477>;
    pub type U478 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P478 = PInt<U478>; pub type N478 = NInt<U478>;
    pub type U479 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P479 = PInt<U479>; pub type N479 = NInt<U479>;
    pub type U480 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P480 = PInt<U480>; pub type N480 = NInt<U480>;
    pub type U481 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P481 = PInt<U481>; pub type N481 = NInt<U481>;
    pub type U482 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P482 = PInt<U482>; pub type N482 = NInt<U482>;
    pub type U483 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P483 = PInt<U483>; pub type N483 = NInt<U483>;
    pub type U484 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P484 = PInt<U484>; pub type N484 = NInt<U484>;
    pub type U485 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P485 = PInt<U485>; pub type N485 = NInt<U485>;
    pub type U486 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P486 = PInt<U486>; pub type N486 = NInt<U486>;
    pub type U487 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P487 = PInt<U487>; pub type N487 = NInt<U487>;
    pub type U488 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P488 = PInt<U488>; pub type N488 = NInt<U488>;
    pub type U489 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P489 = PInt<U489>; pub type N489 = NInt<U489>;
    pub type U490 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P490 = PInt<U490>; pub type N490 = NInt<U490>;
    pub type U491 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P491 = PInt<U491>; pub type N491 = NInt<U491>;
    pub type U492 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P492 = PInt<U492>; pub type N492 = NInt<U492>;
    pub type U493 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P493 = PInt<U493>; pub type N493 = NInt<U493>;
    pub type U494 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P494 = PInt<U494>; pub type N494 = NInt<U494>;
    pub type U495 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P495 = PInt<U495>; pub type N495 = NInt<U495>;
    pub type U496 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P496 = PInt<U496>; pub type N496 = NInt<U496>;
    pub type U497 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P497 = PInt<U497>; pub type N497 = NInt<U497>;
    pub type U498 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P498 = PInt<U498>; pub type N498 = NInt<U498>;
    pub type U499 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P499 = PInt<U499>; pub type N499 = NInt<U499>;
    pub type U500 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P500 = PInt<U500>; pub type N500 = NInt<U500>;
    pub type U501 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P501 = PInt<U501>; pub type N501 = NInt<U501>;
    pub type U502 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P502 = PInt<U502>; pub type N502 = NInt<U502>;
    pub type U503 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P503 = PInt<U503>; pub type N503 = NInt<U503>;
    pub type U504 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P504 = PInt<U504>; pub type N504 = NInt<U504>;
    pub type U505 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P505 = PInt<U505>; pub type N505 = NInt<U505>;
    pub type U506 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P506 = PInt<U506>; pub type N506 = NInt<U506>;
    pub type U507 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P507 = PInt<U507>; pub type N507 = NInt<U507>;
    pub type U508 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P508 = PInt<U508>; pub type N508 = NInt<U508>;
    pub type U509 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P509 = PInt<U509>; pub type N509 = NInt<U509>;
    pub type U510 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P510 = PInt<U510>; pub type N510 = NInt<U510>;
    pub type U511 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P511 = PInt<U511>; pub type N511 = NInt<U511>;
    pub type U512 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P512 = PInt<U512>; pub type N512 = NInt<U512>;
    pub type U513 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P513 = PInt<U513>; pub type N513 = NInt<U513>;
    pub type U514 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P514 = PInt<U514>; pub type N514 = NInt<U514>;
    pub type U515 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P515 = PInt<U515>; pub type N515 = NInt<U515>;
    pub type U516 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P516 = PInt<U516>; pub type N516 = NInt<U516>;
    pub type U517 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P517 = PInt<U517>; pub type N517 = NInt<U517>;
    pub type U518 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P518 = PInt<U518>; pub type N518 = NInt<U518>;
    pub type U519 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P519 = PInt<U519>; pub type N519 = NInt<U519>;
    pub type U520 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P520 = PInt<U520>; pub type N520 = NInt<U520>;
    pub type U521 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P521 = PInt<U521>; pub type N521 = NInt<U521>;
    pub type U522 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P522 = PInt<U522>; pub type N522 = NInt<U522>;
    pub type U523 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P523 = PInt<U523>; pub type N523 = NInt<U523>;
    pub type U524 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P524 = PInt<U524>; pub type N524 = NInt<U524>;
    pub type U525 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P525 = PInt<U525>; pub type N525 = NInt<U525>;
    pub type U526 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P526 = PInt<U526>; pub type N526 = NInt<U526>;
    pub type U527 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P527 = PInt<U527>; pub type N527 = NInt<U527>;
    pub type U528 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P528 = PInt<U528>; pub type N528 = NInt<U528>;
    pub type U529 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P529 = PInt<U529>; pub type N529 = NInt<U529>;
    pub type U530 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P530 = PInt<U530>; pub type N530 = NInt<U530>;
    pub type U531 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P531 = PInt<U531>; pub type N531 = NInt<U531>;
    pub type U532 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P532 = PInt<U532>; pub type N532 = NInt<U532>;
    pub type U533 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P533 = PInt<U533>; pub type N533 = NInt<U533>;
    pub type U534 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P534 = PInt<U534>; pub type N534 = NInt<U534>;
    pub type U535 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P535 = PInt<U535>; pub type N535 = NInt<U535>;
    pub type U536 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P536 = PInt<U536>; pub type N536 = NInt<U536>;
    pub type U537 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P537 = PInt<U537>; pub type N537 = NInt<U537>;
    pub type U538 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P538 = PInt<U538>; pub type N538 = NInt<U538>;
    pub type U539 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P539 = PInt<U539>; pub type N539 = NInt<U539>;
    pub type U540 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P540 = PInt<U540>; pub type N540 = NInt<U540>;
    pub type U541 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P541 = PInt<U541>; pub type N541 = NInt<U541>;
    pub type U542 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P542 = PInt<U542>; pub type N542 = NInt<U542>;
    pub type U543 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P543 = PInt<U543>; pub type N543 = NInt<U543>;
    pub type U544 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P544 = PInt<U544>; pub type N544 = NInt<U544>;
    pub type U545 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P545 = PInt<U545>; pub type N545 = NInt<U545>;
    pub type U546 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P546 = PInt<U546>; pub type N546 = NInt<U546>;
    pub type U547 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P547 = PInt<U547>; pub type N547 = NInt<U547>;
    pub type U548 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P548 = PInt<U548>; pub type N548 = NInt<U548>;
    pub type U549 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P549 = PInt<U549>; pub type N549 = NInt<U549>;
    pub type U550 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P550 = PInt<U550>; pub type N550 = NInt<U550>;
    pub type U551 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P551 = PInt<U551>; pub type N551 = NInt<U551>;
    pub type U552 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P552 = PInt<U552>; pub type N552 = NInt<U552>;
    pub type U553 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P553 = PInt<U553>; pub type N553 = NInt<U553>;
    pub type U554 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P554 = PInt<U554>; pub type N554 = NInt<U554>;
    pub type U555 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P555 = PInt<U555>; pub type N555 = NInt<U555>;
    pub type U556 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P556 = PInt<U556>; pub type N556 = NInt<U556>;
    pub type U557 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P557 = PInt<U557>; pub type N557 = NInt<U557>;
    pub type U558 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P558 = PInt<U558>; pub type N558 = NInt<U558>;
    pub type U559 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P559 = PInt<U559>; pub type N559 = NInt<U559>;
    pub type U560 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P560 = PInt<U560>; pub type N560 = NInt<U560>;
    pub type U561 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P561 = PInt<U561>; pub type N561 = NInt<U561>;
    pub type U562 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P562 = PInt<U562>; pub type N562 = NInt<U562>;
    pub type U563 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P563 = PInt<U563>; pub type N563 = NInt<U563>;
    pub type U564 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P564 = PInt<U564>; pub type N564 = NInt<U564>;
    pub type U565 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P565 = PInt<U565>; pub type N565 = NInt<U565>;
    pub type U566 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P566 = PInt<U566>; pub type N566 = NInt<U566>;
    pub type U567 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P567 = PInt<U567>; pub type N567 = NInt<U567>;
    pub type U568 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P568 = PInt<U568>; pub type N568 = NInt<U568>;
    pub type U569 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P569 = PInt<U569>; pub type N569 = NInt<U569>;
    pub type U570 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P570 = PInt<U570>; pub type N570 = NInt<U570>;
    pub type U571 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P571 = PInt<U571>; pub type N571 = NInt<U571>;
    pub type U572 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P572 = PInt<U572>; pub type N572 = NInt<U572>;
    pub type U573 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P573 = PInt<U573>; pub type N573 = NInt<U573>;
    pub type U574 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P574 = PInt<U574>; pub type N574 = NInt<U574>;
    pub type U575 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P575 = PInt<U575>; pub type N575 = NInt<U575>;
    pub type U576 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P576 = PInt<U576>; pub type N576 = NInt<U576>;
    pub type U577 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P577 = PInt<U577>; pub type N577 = NInt<U577>;
    pub type U578 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P578 = PInt<U578>; pub type N578 = NInt<U578>;
    pub type U579 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P579 = PInt<U579>; pub type N579 = NInt<U579>;
    pub type U580 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P580 = PInt<U580>; pub type N580 = NInt<U580>;
    pub type U581 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P581 = PInt<U581>; pub type N581 = NInt<U581>;
    pub type U582 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P582 = PInt<U582>; pub type N582 = NInt<U582>;
    pub type U583 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P583 = PInt<U583>; pub type N583 = NInt<U583>;
    pub type U584 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P584 = PInt<U584>; pub type N584 = NInt<U584>;
    pub type U585 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P585 = PInt<U585>; pub type N585 = NInt<U585>;
    pub type U586 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P586 = PInt<U586>; pub type N586 = NInt<U586>;
    pub type U587 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P587 = PInt<U587>; pub type N587 = NInt<U587>;
    pub type U588 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P588 = PInt<U588>; pub type N588 = NInt<U588>;
    pub type U589 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P589 = PInt<U589>; pub type N589 = NInt<U589>;
    pub type U590 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P590 = PInt<U590>; pub type N590 = NInt<U590>;
    pub type U591 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P591 = PInt<U591>; pub type N591 = NInt<U591>;
    pub type U592 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P592 = PInt<U592>; pub type N592 = NInt<U592>;
    pub type U593 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P593 = PInt<U593>; pub type N593 = NInt<U593>;
    pub type U594 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P594 = PInt<U594>; pub type N594 = NInt<U594>;
    pub type U595 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P595 = PInt<U595>; pub type N595 = NInt<U595>;
    pub type U596 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P596 = PInt<U596>; pub type N596 = NInt<U596>;
    pub type U597 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P597 = PInt<U597>; pub type N597 = NInt<U597>;
    pub type U598 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P598 = PInt<U598>; pub type N598 = NInt<U598>;
    pub type U599 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P599 = PInt<U599>; pub type N599 = NInt<U599>;
    pub type U600 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P600 = PInt<U600>; pub type N600 = NInt<U600>;
    pub type U601 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P601 = PInt<U601>; pub type N601 = NInt<U601>;
    pub type U602 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P602 = PInt<U602>; pub type N602 = NInt<U602>;
    pub type U603 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P603 = PInt<U603>; pub type N603 = NInt<U603>;
    pub type U604 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P604 = PInt<U604>; pub type N604 = NInt<U604>;
    pub type U605 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P605 = PInt<U605>; pub type N605 = NInt<U605>;
    pub type U606 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P606 = PInt<U606>; pub type N606 = NInt<U606>;
    pub type U607 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P607 = PInt<U607>; pub type N607 = NInt<U607>;
    pub type U608 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P608 = PInt<U608>; pub type N608 = NInt<U608>;
    pub type U609 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P609 = PInt<U609>; pub type N609 = NInt<U609>;
    pub type U610 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P610 = PInt<U610>; pub type N610 = NInt<U610>;
    pub type U611 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P611 = PInt<U611>; pub type N611 = NInt<U611>;
    pub type U612 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P612 = PInt<U612>; pub type N612 = NInt<U612>;
    pub type U613 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P613 = PInt<U613>; pub type N613 = NInt<U613>;
    pub type U614 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P614 = PInt<U614>; pub type N614 = NInt<U614>;
    pub type U615 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P615 = PInt<U615>; pub type N615 = NInt<U615>;
    pub type U616 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P616 = PInt<U616>; pub type N616 = NInt<U616>;
    pub type U617 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P617 = PInt<U617>; pub type N617 = NInt<U617>;
    pub type U618 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P618 = PInt<U618>; pub type N618 = NInt<U618>;
    pub type U619 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P619 = PInt<U619>; pub type N619 = NInt<U619>;
    pub type U620 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P620 = PInt<U620>; pub type N620 = NInt<U620>;
    pub type U621 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P621 = PInt<U621>; pub type N621 = NInt<U621>;
    pub type U622 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P622 = PInt<U622>; pub type N622 = NInt<U622>;
    pub type U623 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P623 = PInt<U623>; pub type N623 = NInt<U623>;
    pub type U624 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P624 = PInt<U624>; pub type N624 = NInt<U624>;
    pub type U625 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P625 = PInt<U625>; pub type N625 = NInt<U625>;
    pub type U626 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P626 = PInt<U626>; pub type N626 = NInt<U626>;
    pub type U627 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P627 = PInt<U627>; pub type N627 = NInt<U627>;
    pub type U628 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P628 = PInt<U628>; pub type N628 = NInt<U628>;
    pub type U629 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P629 = PInt<U629>; pub type N629 = NInt<U629>;
    pub type U630 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P630 = PInt<U630>; pub type N630 = NInt<U630>;
    pub type U631 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P631 = PInt<U631>; pub type N631 = NInt<U631>;
    pub type U632 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P632 = PInt<U632>; pub type N632 = NInt<U632>;
    pub type U633 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P633 = PInt<U633>; pub type N633 = NInt<U633>;
    pub type U634 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P634 = PInt<U634>; pub type N634 = NInt<U634>;
    pub type U635 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P635 = PInt<U635>; pub type N635 = NInt<U635>;
    pub type U636 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P636 = PInt<U636>; pub type N636 = NInt<U636>;
    pub type U637 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P637 = PInt<U637>; pub type N637 = NInt<U637>;
    pub type U638 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P638 = PInt<U638>; pub type N638 = NInt<U638>;
    pub type U639 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P639 = PInt<U639>; pub type N639 = NInt<U639>;
    pub type U640 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P640 = PInt<U640>; pub type N640 = NInt<U640>;
    pub type U641 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P641 = PInt<U641>; pub type N641 = NInt<U641>;
    pub type U642 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P642 = PInt<U642>; pub type N642 = NInt<U642>;
    pub type U643 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P643 = PInt<U643>; pub type N643 = NInt<U643>;
    pub type U644 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P644 = PInt<U644>; pub type N644 = NInt<U644>;
    pub type U645 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P645 = PInt<U645>; pub type N645 = NInt<U645>;
    pub type U646 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P646 = PInt<U646>; pub type N646 = NInt<U646>;
    pub type U647 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P647 = PInt<U647>; pub type N647 = NInt<U647>;
    pub type U648 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P648 = PInt<U648>; pub type N648 = NInt<U648>;
    pub type U649 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P649 = PInt<U649>; pub type N649 = NInt<U649>;
    pub type U650 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P650 = PInt<U650>; pub type N650 = NInt<U650>;
    pub type U651 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P651 = PInt<U651>; pub type N651 = NInt<U651>;
    pub type U652 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P652 = PInt<U652>; pub type N652 = NInt<U652>;
    pub type U653 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P653 = PInt<U653>; pub type N653 = NInt<U653>;
    pub type U654 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P654 = PInt<U654>; pub type N654 = NInt<U654>;
    pub type U655 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P655 = PInt<U655>; pub type N655 = NInt<U655>;
    pub type U656 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P656 = PInt<U656>; pub type N656 = NInt<U656>;
    pub type U657 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P657 = PInt<U657>; pub type N657 = NInt<U657>;
    pub type U658 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P658 = PInt<U658>; pub type N658 = NInt<U658>;
    pub type U659 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P659 = PInt<U659>; pub type N659 = NInt<U659>;
    pub type U660 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P660 = PInt<U660>; pub type N660 = NInt<U660>;
    pub type U661 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P661 = PInt<U661>; pub type N661 = NInt<U661>;
    pub type U662 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P662 = PInt<U662>; pub type N662 = NInt<U662>;
    pub type U663 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P663 = PInt<U663>; pub type N663 = NInt<U663>;
    pub type U664 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P664 = PInt<U664>; pub type N664 = NInt<U664>;
    pub type U665 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P665 = PInt<U665>; pub type N665 = NInt<U665>;
    pub type U666 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P666 = PInt<U666>; pub type N666 = NInt<U666>;
    pub type U667 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P667 = PInt<U667>; pub type N667 = NInt<U667>;
    pub type U668 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P668 = PInt<U668>; pub type N668 = NInt<U668>;
    pub type U669 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P669 = PInt<U669>; pub type N669 = NInt<U669>;
    pub type U670 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P670 = PInt<U670>; pub type N670 = NInt<U670>;
    pub type U671 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P671 = PInt<U671>; pub type N671 = NInt<U671>;
    pub type U672 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P672 = PInt<U672>; pub type N672 = NInt<U672>;
    pub type U673 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P673 = PInt<U673>; pub type N673 = NInt<U673>;
    pub type U674 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P674 = PInt<U674>; pub type N674 = NInt<U674>;
    pub type U675 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P675 = PInt<U675>; pub type N675 = NInt<U675>;
    pub type U676 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P676 = PInt<U676>; pub type N676 = NInt<U676>;
    pub type U677 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P677 = PInt<U677>; pub type N677 = NInt<U677>;
    pub type U678 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P678 = PInt<U678>; pub type N678 = NInt<U678>;
    pub type U679 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P679 = PInt<U679>; pub type N679 = NInt<U679>;
    pub type U680 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P680 = PInt<U680>; pub type N680 = NInt<U680>;
    pub type U681 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P681 = PInt<U681>; pub type N681 = NInt<U681>;
    pub type U682 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P682 = PInt<U682>; pub type N682 = NInt<U682>;
    pub type U683 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P683 = PInt<U683>; pub type N683 = NInt<U683>;
    pub type U684 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P684 = PInt<U684>; pub type N684 = NInt<U684>;
    pub type U685 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P685 = PInt<U685>; pub type N685 = NInt<U685>;
    pub type U686 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P686 = PInt<U686>; pub type N686 = NInt<U686>;
    pub type U687 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P687 = PInt<U687>; pub type N687 = NInt<U687>;
    pub type U688 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P688 = PInt<U688>; pub type N688 = NInt<U688>;
    pub type U689 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P689 = PInt<U689>; pub type N689 = NInt<U689>;
    pub type U690 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P690 = PInt<U690>; pub type N690 = NInt<U690>;
    pub type U691 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P691 = PInt<U691>; pub type N691 = NInt<U691>;
    pub type U692 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P692 = PInt<U692>; pub type N692 = NInt<U692>;
    pub type U693 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P693 = PInt<U693>; pub type N693 = NInt<U693>;
    pub type U694 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P694 = PInt<U694>; pub type N694 = NInt<U694>;
    pub type U695 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P695 = PInt<U695>; pub type N695 = NInt<U695>;
    pub type U696 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P696 = PInt<U696>; pub type N696 = NInt<U696>;
    pub type U697 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P697 = PInt<U697>; pub type N697 = NInt<U697>;
    pub type U698 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P698 = PInt<U698>; pub type N698 = NInt<U698>;
    pub type U699 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P699 = PInt<U699>; pub type N699 = NInt<U699>;
    pub type U700 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P700 = PInt<U700>; pub type N700 = NInt<U700>;
    pub type U701 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P701 = PInt<U701>; pub type N701 = NInt<U701>;
    pub type U702 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P702 = PInt<U702>; pub type N702 = NInt<U702>;
    pub type U703 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P703 = PInt<U703>; pub type N703 = NInt<U703>;
    pub type U704 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P704 = PInt<U704>; pub type N704 = NInt<U704>;
    pub type U705 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P705 = PInt<U705>; pub type N705 = NInt<U705>;
    pub type U706 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P706 = PInt<U706>; pub type N706 = NInt<U706>;
    pub type U707 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P707 = PInt<U707>; pub type N707 = NInt<U707>;
    pub type U708 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P708 = PInt<U708>; pub type N708 = NInt<U708>;
    pub type U709 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P709 = PInt<U709>; pub type N709 = NInt<U709>;
    pub type U710 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P710 = PInt<U710>; pub type N710 = NInt<U710>;
    pub type U711 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P711 = PInt<U711>; pub type N711 = NInt<U711>;
    pub type U712 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P712 = PInt<U712>; pub type N712 = NInt<U712>;
    pub type U713 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P713 = PInt<U713>; pub type N713 = NInt<U713>;
    pub type U714 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P714 = PInt<U714>; pub type N714 = NInt<U714>;
    pub type U715 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P715 = PInt<U715>; pub type N715 = NInt<U715>;
    pub type U716 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P716 = PInt<U716>; pub type N716 = NInt<U716>;
    pub type U717 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P717 = PInt<U717>; pub type N717 = NInt<U717>;
    pub type U718 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P718 = PInt<U718>; pub type N718 = NInt<U718>;
    pub type U719 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P719 = PInt<U719>; pub type N719 = NInt<U719>;
    pub type U720 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P720 = PInt<U720>; pub type N720 = NInt<U720>;
    pub type U721 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P721 = PInt<U721>; pub type N721 = NInt<U721>;
    pub type U722 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P722 = PInt<U722>; pub type N722 = NInt<U722>;
    pub type U723 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P723 = PInt<U723>; pub type N723 = NInt<U723>;
    pub type U724 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P724 = PInt<U724>; pub type N724 = NInt<U724>;
    pub type U725 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P725 = PInt<U725>; pub type N725 = NInt<U725>;
    pub type U726 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P726 = PInt<U726>; pub type N726 = NInt<U726>;
    pub type U727 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P727 = PInt<U727>; pub type N727 = NInt<U727>;
    pub type U728 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P728 = PInt<U728>; pub type N728 = NInt<U728>;
    pub type U729 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P729 = PInt<U729>; pub type N729 = NInt<U729>;
    pub type U730 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P730 = PInt<U730>; pub type N730 = NInt<U730>;
    pub type U731 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P731 = PInt<U731>; pub type N731 = NInt<U731>;
    pub type U732 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P732 = PInt<U732>; pub type N732 = NInt<U732>;
    pub type U733 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P733 = PInt<U733>; pub type N733 = NInt<U733>;
    pub type U734 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P734 = PInt<U734>; pub type N734 = NInt<U734>;
    pub type U735 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P735 = PInt<U735>; pub type N735 = NInt<U735>;
    pub type U736 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P736 = PInt<U736>; pub type N736 = NInt<U736>;
    pub type U737 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P737 = PInt<U737>; pub type N737 = NInt<U737>;
    pub type U738 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P738 = PInt<U738>; pub type N738 = NInt<U738>;
    pub type U739 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P739 = PInt<U739>; pub type N739 = NInt<U739>;
    pub type U740 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P740 = PInt<U740>; pub type N740 = NInt<U740>;
    pub type U741 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P741 = PInt<U741>; pub type N741 = NInt<U741>;
    pub type U742 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P742 = PInt<U742>; pub type N742 = NInt<U742>;
    pub type U743 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P743 = PInt<U743>; pub type N743 = NInt<U743>;
    pub type U744 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P744 = PInt<U744>; pub type N744 = NInt<U744>;
    pub type U745 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P745 = PInt<U745>; pub type N745 = NInt<U745>;
    pub type U746 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P746 = PInt<U746>; pub type N746 = NInt<U746>;
    pub type U747 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P747 = PInt<U747>; pub type N747 = NInt<U747>;
    pub type U748 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P748 = PInt<U748>; pub type N748 = NInt<U748>;
    pub type U749 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P749 = PInt<U749>; pub type N749 = NInt<U749>;
    pub type U750 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P750 = PInt<U750>; pub type N750 = NInt<U750>;
    pub type U751 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P751 = PInt<U751>; pub type N751 = NInt<U751>;
    pub type U752 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P752 = PInt<U752>; pub type N752 = NInt<U752>;
    pub type U753 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P753 = PInt<U753>; pub type N753 = NInt<U753>;
    pub type U754 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P754 = PInt<U754>; pub type N754 = NInt<U754>;
    pub type U755 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P755 = PInt<U755>; pub type N755 = NInt<U755>;
    pub type U756 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P756 = PInt<U756>; pub type N756 = NInt<U756>;
    pub type U757 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P757 = PInt<U757>; pub type N757 = NInt<U757>;
    pub type U758 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P758 = PInt<U758>; pub type N758 = NInt<U758>;
    pub type U759 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P759 = PInt<U759>; pub type N759 = NInt<U759>;
    pub type U760 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P760 = PInt<U760>; pub type N760 = NInt<U760>;
    pub type U761 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P761 = PInt<U761>; pub type N761 = NInt<U761>;
    pub type U762 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P762 = PInt<U762>; pub type N762 = NInt<U762>;
    pub type U763 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P763 = PInt<U763>; pub type N763 = NInt<U763>;
    pub type U764 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P764 = PInt<U764>; pub type N764 = NInt<U764>;
    pub type U765 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P765 = PInt<U765>; pub type N765 = NInt<U765>;
    pub type U766 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P766 = PInt<U766>; pub type N766 = NInt<U766>;
    pub type U767 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P767 = PInt<U767>; pub type N767 = NInt<U767>;
    pub type U768 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P768 = PInt<U768>; pub type N768 = NInt<U768>;
    pub type U769 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P769 = PInt<U769>; pub type N769 = NInt<U769>;
    pub type U770 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P770 = PInt<U770>; pub type N770 = NInt<U770>;
    pub type U771 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P771 = PInt<U771>; pub type N771 = NInt<U771>;
    pub type U772 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P772 = PInt<U772>; pub type N772 = NInt<U772>;
    pub type U773 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P773 = PInt<U773>; pub type N773 = NInt<U773>;
    pub type U774 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P774 = PInt<U774>; pub type N774 = NInt<U774>;
    pub type U775 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P775 = PInt<U775>; pub type N775 = NInt<U775>;
    pub type U776 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P776 = PInt<U776>; pub type N776 = NInt<U776>;
    pub type U777 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P777 = PInt<U777>; pub type N777 = NInt<U777>;
    pub type U778 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P778 = PInt<U778>; pub type N778 = NInt<U778>;
    pub type U779 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P779 = PInt<U779>; pub type N779 = NInt<U779>;
    pub type U780 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P780 = PInt<U780>; pub type N780 = NInt<U780>;
    pub type U781 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P781 = PInt<U781>; pub type N781 = NInt<U781>;
    pub type U782 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P782 = PInt<U782>; pub type N782 = NInt<U782>;
    pub type U783 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P783 = PInt<U783>; pub type N783 = NInt<U783>;
    pub type U784 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P784 = PInt<U784>; pub type N784 = NInt<U784>;
    pub type U785 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P785 = PInt<U785>; pub type N785 = NInt<U785>;
    pub type U786 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P786 = PInt<U786>; pub type N786 = NInt<U786>;
    pub type U787 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P787 = PInt<U787>; pub type N787 = NInt<U787>;
    pub type U788 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P788 = PInt<U788>; pub type N788 = NInt<U788>;
    pub type U789 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P789 = PInt<U789>; pub type N789 = NInt<U789>;
    pub type U790 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P790 = PInt<U790>; pub type N790 = NInt<U790>;
    pub type U791 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P791 = PInt<U791>; pub type N791 = NInt<U791>;
    pub type U792 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P792 = PInt<U792>; pub type N792 = NInt<U792>;
    pub type U793 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P793 = PInt<U793>; pub type N793 = NInt<U793>;
    pub type U794 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P794 = PInt<U794>; pub type N794 = NInt<U794>;
    pub type U795 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P795 = PInt<U795>; pub type N795 = NInt<U795>;
    pub type U796 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P796 = PInt<U796>; pub type N796 = NInt<U796>;
    pub type U797 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P797 = PInt<U797>; pub type N797 = NInt<U797>;
    pub type U798 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P798 = PInt<U798>; pub type N798 = NInt<U798>;
    pub type U799 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P799 = PInt<U799>; pub type N799 = NInt<U799>;
    pub type U800 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P800 = PInt<U800>; pub type N800 = NInt<U800>;
    pub type U801 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P801 = PInt<U801>; pub type N801 = NInt<U801>;
    pub type U802 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P802 = PInt<U802>; pub type N802 = NInt<U802>;
    pub type U803 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P803 = PInt<U803>; pub type N803 = NInt<U803>;
    pub type U804 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P804 = PInt<U804>; pub type N804 = NInt<U804>;
    pub type U805 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P805 = PInt<U805>; pub type N805 = NInt<U805>;
    pub type U806 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P806 = PInt<U806>; pub type N806 = NInt<U806>;
    pub type U807 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P807 = PInt<U807>; pub type N807 = NInt<U807>;
    pub type U808 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P808 = PInt<U808>; pub type N808 = NInt<U808>;
    pub type U809 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P809 = PInt<U809>; pub type N809 = NInt<U809>;
    pub type U810 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P810 = PInt<U810>; pub type N810 = NInt<U810>;
    pub type U811 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P811 = PInt<U811>; pub type N811 = NInt<U811>;
    pub type U812 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P812 = PInt<U812>; pub type N812 = NInt<U812>;
    pub type U813 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P813 = PInt<U813>; pub type N813 = NInt<U813>;
    pub type U814 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P814 = PInt<U814>; pub type N814 = NInt<U814>;
    pub type U815 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P815 = PInt<U815>; pub type N815 = NInt<U815>;
    pub type U816 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P816 = PInt<U816>; pub type N816 = NInt<U816>;
    pub type U817 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P817 = PInt<U817>; pub type N817 = NInt<U817>;
    pub type U818 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P818 = PInt<U818>; pub type N818 = NInt<U818>;
    pub type U819 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P819 = PInt<U819>; pub type N819 = NInt<U819>;
    pub type U820 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P820 = PInt<U820>; pub type N820 = NInt<U820>;
    pub type U821 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P821 = PInt<U821>; pub type N821 = NInt<U821>;
    pub type U822 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P822 = PInt<U822>; pub type N822 = NInt<U822>;
    pub type U823 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P823 = PInt<U823>; pub type N823 = NInt<U823>;
    pub type U824 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P824 = PInt<U824>; pub type N824 = NInt<U824>;
    pub type U825 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P825 = PInt<U825>; pub type N825 = NInt<U825>;
    pub type U826 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P826 = PInt<U826>; pub type N826 = NInt<U826>;
    pub type U827 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P827 = PInt<U827>; pub type N827 = NInt<U827>;
    pub type U828 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P828 = PInt<U828>; pub type N828 = NInt<U828>;
    pub type U829 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P829 = PInt<U829>; pub type N829 = NInt<U829>;
    pub type U830 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P830 = PInt<U830>; pub type N830 = NInt<U830>;
    pub type U831 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P831 = PInt<U831>; pub type N831 = NInt<U831>;
    pub type U832 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P832 = PInt<U832>; pub type N832 = NInt<U832>;
    pub type U833 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P833 = PInt<U833>; pub type N833 = NInt<U833>;
    pub type U834 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P834 = PInt<U834>; pub type N834 = NInt<U834>;
    pub type U835 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P835 = PInt<U835>; pub type N835 = NInt<U835>;
    pub type U836 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P836 = PInt<U836>; pub type N836 = NInt<U836>;
    pub type U837 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P837 = PInt<U837>; pub type N837 = NInt<U837>;
    pub type U838 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P838 = PInt<U838>; pub type N838 = NInt<U838>;
    pub type U839 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P839 = PInt<U839>; pub type N839 = NInt<U839>;
    pub type U840 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P840 = PInt<U840>; pub type N840 = NInt<U840>;
    pub type U841 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P841 = PInt<U841>; pub type N841 = NInt<U841>;
    pub type U842 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P842 = PInt<U842>; pub type N842 = NInt<U842>;
    pub type U843 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P843 = PInt<U843>; pub type N843 = NInt<U843>;
    pub type U844 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P844 = PInt<U844>; pub type N844 = NInt<U844>;
    pub type U845 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P845 = PInt<U845>; pub type N845 = NInt<U845>;
    pub type U846 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P846 = PInt<U846>; pub type N846 = NInt<U846>;
    pub type U847 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P847 = PInt<U847>; pub type N847 = NInt<U847>;
    pub type U848 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P848 = PInt<U848>; pub type N848 = NInt<U848>;
    pub type U849 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P849 = PInt<U849>; pub type N849 = NInt<U849>;
    pub type U850 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P850 = PInt<U850>; pub type N850 = NInt<U850>;
    pub type U851 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P851 = PInt<U851>; pub type N851 = NInt<U851>;
    pub type U852 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P852 = PInt<U852>; pub type N852 = NInt<U852>;
    pub type U853 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P853 = PInt<U853>; pub type N853 = NInt<U853>;
    pub type U854 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P854 = PInt<U854>; pub type N854 = NInt<U854>;
    pub type U855 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P855 = PInt<U855>; pub type N855 = NInt<U855>;
    pub type U856 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P856 = PInt<U856>; pub type N856 = NInt<U856>;
    pub type U857 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P857 = PInt<U857>; pub type N857 = NInt<U857>;
    pub type U858 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P858 = PInt<U858>; pub type N858 = NInt<U858>;
    pub type U859 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P859 = PInt<U859>; pub type N859 = NInt<U859>;
    pub type U860 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P860 = PInt<U860>; pub type N860 = NInt<U860>;
    pub type U861 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P861 = PInt<U861>; pub type N861 = NInt<U861>;
    pub type U862 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P862 = PInt<U862>; pub type N862 = NInt<U862>;
    pub type U863 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P863 = PInt<U863>; pub type N863 = NInt<U863>;
    pub type U864 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P864 = PInt<U864>; pub type N864 = NInt<U864>;
    pub type U865 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P865 = PInt<U865>; pub type N865 = NInt<U865>;
    pub type U866 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P866 = PInt<U866>; pub type N866 = NInt<U866>;
    pub type U867 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P867 = PInt<U867>; pub type N867 = NInt<U867>;
    pub type U868 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P868 = PInt<U868>; pub type N868 = NInt<U868>;
    pub type U869 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P869 = PInt<U869>; pub type N869 = NInt<U869>;
    pub type U870 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P870 = PInt<U870>; pub type N870 = NInt<U870>;
    pub type U871 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P871 = PInt<U871>; pub type N871 = NInt<U871>;
    pub type U872 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P872 = PInt<U872>; pub type N872 = NInt<U872>;
    pub type U873 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P873 = PInt<U873>; pub type N873 = NInt<U873>;
    pub type U874 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P874 = PInt<U874>; pub type N874 = NInt<U874>;
    pub type U875 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P875 = PInt<U875>; pub type N875 = NInt<U875>;
    pub type U876 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P876 = PInt<U876>; pub type N876 = NInt<U876>;
    pub type U877 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P877 = PInt<U877>; pub type N877 = NInt<U877>;
    pub type U878 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P878 = PInt<U878>; pub type N878 = NInt<U878>;
    pub type U879 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P879 = PInt<U879>; pub type N879 = NInt<U879>;
    pub type U880 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P880 = PInt<U880>; pub type N880 = NInt<U880>;
    pub type U881 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P881 = PInt<U881>; pub type N881 = NInt<U881>;
    pub type U882 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P882 = PInt<U882>; pub type N882 = NInt<U882>;
    pub type U883 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P883 = PInt<U883>; pub type N883 = NInt<U883>;
    pub type U884 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P884 = PInt<U884>; pub type N884 = NInt<U884>;
    pub type U885 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P885 = PInt<U885>; pub type N885 = NInt<U885>;
    pub type U886 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P886 = PInt<U886>; pub type N886 = NInt<U886>;
    pub type U887 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P887 = PInt<U887>; pub type N887 = NInt<U887>;
    pub type U888 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P888 = PInt<U888>; pub type N888 = NInt<U888>;
    pub type U889 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P889 = PInt<U889>; pub type N889 = NInt<U889>;
    pub type U890 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P890 = PInt<U890>; pub type N890 = NInt<U890>;
    pub type U891 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P891 = PInt<U891>; pub type N891 = NInt<U891>;
    pub type U892 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P892 = PInt<U892>; pub type N892 = NInt<U892>;
    pub type U893 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P893 = PInt<U893>; pub type N893 = NInt<U893>;
    pub type U894 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P894 = PInt<U894>; pub type N894 = NInt<U894>;
    pub type U895 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P895 = PInt<U895>; pub type N895 = NInt<U895>;
    pub type U896 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P896 = PInt<U896>; pub type N896 = NInt<U896>;
    pub type U897 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P897 = PInt<U897>; pub type N897 = NInt<U897>;
    pub type U898 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P898 = PInt<U898>; pub type N898 = NInt<U898>;
    pub type U899 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P899 = PInt<U899>; pub type N899 = NInt<U899>;
    pub type U900 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P900 = PInt<U900>; pub type N900 = NInt<U900>;
    pub type U901 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P901 = PInt<U901>; pub type N901 = NInt<U901>;
    pub type U902 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P902 = PInt<U902>; pub type N902 = NInt<U902>;
    pub type U903 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P903 = PInt<U903>; pub type N903 = NInt<U903>;
    pub type U904 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P904 = PInt<U904>; pub type N904 = NInt<U904>;
    pub type U905 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P905 = PInt<U905>; pub type N905 = NInt<U905>;
    pub type U906 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P906 = PInt<U906>; pub type N906 = NInt<U906>;
    pub type U907 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P907 = PInt<U907>; pub type N907 = NInt<U907>;
    pub type U908 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P908 = PInt<U908>; pub type N908 = NInt<U908>;
    pub type U909 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P909 = PInt<U909>; pub type N909 = NInt<U909>;
    pub type U910 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P910 = PInt<U910>; pub type N910 = NInt<U910>;
    pub type U911 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P911 = PInt<U911>; pub type N911 = NInt<U911>;
    pub type U912 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P912 = PInt<U912>; pub type N912 = NInt<U912>;
    pub type U913 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P913 = PInt<U913>; pub type N913 = NInt<U913>;
    pub type U914 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P914 = PInt<U914>; pub type N914 = NInt<U914>;
    pub type U915 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P915 = PInt<U915>; pub type N915 = NInt<U915>;
    pub type U916 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P916 = PInt<U916>; pub type N916 = NInt<U916>;
    pub type U917 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P917 = PInt<U917>; pub type N917 = NInt<U917>;
    pub type U918 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P918 = PInt<U918>; pub type N918 = NInt<U918>;
    pub type U919 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P919 = PInt<U919>; pub type N919 = NInt<U919>;
    pub type U920 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P920 = PInt<U920>; pub type N920 = NInt<U920>;
    pub type U921 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P921 = PInt<U921>; pub type N921 = NInt<U921>;
    pub type U922 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P922 = PInt<U922>; pub type N922 = NInt<U922>;
    pub type U923 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P923 = PInt<U923>; pub type N923 = NInt<U923>;
    pub type U924 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P924 = PInt<U924>; pub type N924 = NInt<U924>;
    pub type U925 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P925 = PInt<U925>; pub type N925 = NInt<U925>;
    pub type U926 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P926 = PInt<U926>; pub type N926 = NInt<U926>;
    pub type U927 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P927 = PInt<U927>; pub type N927 = NInt<U927>;
    pub type U928 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P928 = PInt<U928>; pub type N928 = NInt<U928>;
    pub type U929 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P929 = PInt<U929>; pub type N929 = NInt<U929>;
    pub type U930 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P930 = PInt<U930>; pub type N930 = NInt<U930>;
    pub type U931 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P931 = PInt<U931>; pub type N931 = NInt<U931>;
    pub type U932 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P932 = PInt<U932>; pub type N932 = NInt<U932>;
    pub type U933 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P933 = PInt<U933>; pub type N933 = NInt<U933>;
    pub type U934 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P934 = PInt<U934>; pub type N934 = NInt<U934>;
    pub type U935 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P935 = PInt<U935>; pub type N935 = NInt<U935>;
    pub type U936 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P936 = PInt<U936>; pub type N936 = NInt<U936>;
    pub type U937 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P937 = PInt<U937>; pub type N937 = NInt<U937>;
    pub type U938 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P938 = PInt<U938>; pub type N938 = NInt<U938>;
    pub type U939 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P939 = PInt<U939>; pub type N939 = NInt<U939>;
    pub type U940 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P940 = PInt<U940>; pub type N940 = NInt<U940>;
    pub type U941 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P941 = PInt<U941>; pub type N941 = NInt<U941>;
    pub type U942 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P942 = PInt<U942>; pub type N942 = NInt<U942>;
    pub type U943 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P943 = PInt<U943>; pub type N943 = NInt<U943>;
    pub type U944 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P944 = PInt<U944>; pub type N944 = NInt<U944>;
    pub type U945 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P945 = PInt<U945>; pub type N945 = NInt<U945>;
    pub type U946 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P946 = PInt<U946>; pub type N946 = NInt<U946>;
    pub type U947 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P947 = PInt<U947>; pub type N947 = NInt<U947>;
    pub type U948 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P948 = PInt<U948>; pub type N948 = NInt<U948>;
    pub type U949 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P949 = PInt<U949>; pub type N949 = NInt<U949>;
    pub type U950 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P950 = PInt<U950>; pub type N950 = NInt<U950>;
    pub type U951 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P951 = PInt<U951>; pub type N951 = NInt<U951>;
    pub type U952 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P952 = PInt<U952>; pub type N952 = NInt<U952>;
    pub type U953 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P953 = PInt<U953>; pub type N953 = NInt<U953>;
    pub type U954 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P954 = PInt<U954>; pub type N954 = NInt<U954>;
    pub type U955 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P955 = PInt<U955>; pub type N955 = NInt<U955>;
    pub type U956 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P956 = PInt<U956>; pub type N956 = NInt<U956>;
    pub type U957 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P957 = PInt<U957>; pub type N957 = NInt<U957>;
    pub type U958 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P958 = PInt<U958>; pub type N958 = NInt<U958>;
    pub type U959 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P959 = PInt<U959>; pub type N959 = NInt<U959>;
    pub type U960 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P960 = PInt<U960>; pub type N960 = NInt<U960>;
    pub type U961 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>;
    pub type P961 = PInt<U961>; pub type N961 = NInt<U961>;
    pub type U962 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>;
    pub type P962 = PInt<U962>; pub type N962 = NInt<U962>;
    pub type U963 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>;
    pub type P963 = PInt<U963>; pub type N963 = NInt<U963>;
    pub type U964 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>;
    pub type P964 = PInt<U964>; pub type N964 = NInt<U964>;
    pub type U965 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>;
    pub type P965 = PInt<U965>; pub type N965 = NInt<U965>;
    pub type U966 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>;
    pub type P966 = PInt<U966>; pub type N966 = NInt<U966>;
    pub type U967 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>;
    pub type P967 = PInt<U967>; pub type N967 = NInt<U967>;
    pub type U968 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>;
    pub type P968 = PInt<U968>; pub type N968 = NInt<U968>;
    pub type U969 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>;
    pub type P969 = PInt<U969>; pub type N969 = NInt<U969>;
    pub type U970 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>;
    pub type P970 = PInt<U970>; pub type N970 = NInt<U970>;
    pub type U971 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B1>;
    pub type P971 = PInt<U971>; pub type N971 = NInt<U971>;
    pub type U972 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>;
    pub type P972 = PInt<U972>; pub type N972 = NInt<U972>;
    pub type U973 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>;
    pub type P973 = PInt<U973>; pub type N973 = NInt<U973>;
    pub type U974 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>;
    pub type P974 = PInt<U974>; pub type N974 = NInt<U974>;
    pub type U975 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B1>;
    pub type P975 = PInt<U975>; pub type N975 = NInt<U975>;
    pub type U976 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P976 = PInt<U976>; pub type N976 = NInt<U976>;
    pub type U977 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>;
    pub type P977 = PInt<U977>; pub type N977 = NInt<U977>;
    pub type U978 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>;
    pub type P978 = PInt<U978>; pub type N978 = NInt<U978>;
    pub type U979 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>;
    pub type P979 = PInt<U979>; pub type N979 = NInt<U979>;
    pub type U980 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>;
    pub type P980 = PInt<U980>; pub type N980 = NInt<U980>;
    pub type U981 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B1>;
    pub type P981 = PInt<U981>; pub type N981 = NInt<U981>;
    pub type U982 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>;
    pub type P982 = PInt<U982>; pub type N982 = NInt<U982>;
    pub type U983 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>;
    pub type P983 = PInt<U983>; pub type N983 = NInt<U983>;
    pub type U984 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>;
    pub type P984 = PInt<U984>; pub type N984 = NInt<U984>;
    pub type U985 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>;
    pub type P985 = PInt<U985>; pub type N985 = NInt<U985>;
    pub type U986 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>;
    pub type P986 = PInt<U986>; pub type N986 = NInt<U986>;
    pub type U987 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>;
    pub type P987 = PInt<U987>; pub type N987 = NInt<U987>;
    pub type U988 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>;
    pub type P988 = PInt<U988>; pub type N988 = NInt<U988>;
    pub type U989 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>;
    pub type P989 = PInt<U989>; pub type N989 = NInt<U989>;
    pub type U990 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>;
    pub type P990 = PInt<U990>; pub type N990 = NInt<U990>;
    pub type U991 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>;
    pub type P991 = PInt<U991>; pub type N991 = NInt<U991>;
    pub type U992 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P992 = PInt<U992>; pub type N992 = NInt<U992>;
    pub type U993 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>;
    pub type P993 = PInt<U993>; pub type N993 = NInt<U993>;
    pub type U994 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>;
    pub type P994 = PInt<U994>; pub type N994 = NInt<U994>;
    pub type U995 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>;
    pub type P995 = PInt<U995>; pub type N995 = NInt<U995>;
    pub type U996 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>;
    pub type P996 = PInt<U996>; pub type N996 = NInt<U996>;
    pub type U997 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>;
    pub type P997 = PInt<U997>; pub type N997 = NInt<U997>;
    pub type U998 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>;
    pub type P998 = PInt<U998>; pub type N998 = NInt<U998>;
    pub type U999 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>;
    pub type P999 = PInt<U999>; pub type N999 = NInt<U999>;
    pub type U1000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>;
    pub type P1000 = PInt<U1000>; pub type N1000 = NInt<U1000>;
    pub type U1001 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>;
    pub type P1001 = PInt<U1001>; pub type N1001 = NInt<U1001>;
    pub type U1002 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>;
    pub type P1002 = PInt<U1002>; pub type N1002 = NInt<U1002>;
    pub type U1003 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>;
    pub type P1003 = PInt<U1003>; pub type N1003 = NInt<U1003>;
    pub type U1004 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>;
    pub type P1004 = PInt<U1004>; pub type N1004 = NInt<U1004>;
    pub type U1005 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>;
    pub type P1005 = PInt<U1005>; pub type N1005 = NInt<U1005>;
    pub type U1006 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>;
    pub type P1006 = PInt<U1006>; pub type N1006 = NInt<U1006>;
    pub type U1007 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B1>;
    pub type P1007 = PInt<U1007>; pub type N1007 = NInt<U1007>;
    pub type U1008 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>;
    pub type P1008 = PInt<U1008>; pub type N1008 = NInt<U1008>;
    pub type U1009 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B1>;
    pub type P1009 = PInt<U1009>; pub type N1009 = NInt<U1009>;
    pub type U1010 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>;
    pub type P1010 = PInt<U1010>; pub type N1010 = NInt<U1010>;
    pub type U1011 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>;
    pub type P1011 = PInt<U1011>; pub type N1011 = NInt<U1011>;
    pub type U1012 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>;
    pub type P1012 = PInt<U1012>; pub type N1012 = NInt<U1012>;
    pub type U1013 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>;
    pub type P1013 = PInt<U1013>; pub type N1013 = NInt<U1013>;
    pub type U1014 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B0>;
    pub type P1014 = PInt<U1014>; pub type N1014 = NInt<U1014>;
    pub type U1015 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>, B1>;
    pub type P1015 = PInt<U1015>; pub type N1015 = NInt<U1015>;
    pub type U1016 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>;
    pub type P1016 = PInt<U1016>; pub type N1016 = NInt<U1016>;
    pub type U1017 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>;
    pub type P1017 = PInt<U1017>; pub type N1017 = NInt<U1017>;
    pub type U1018 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>;
    pub type P1018 = PInt<U1018>; pub type N1018 = NInt<U1018>;
    pub type U1019 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B1>;
    pub type P1019 = PInt<U1019>; pub type N1019 = NInt<U1019>;
    pub type U1020 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>;
    pub type P1020 = PInt<U1020>; pub type N1020 = NInt<U1020>;
    pub type U1021 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>;
    pub type P1021 = PInt<U1021>; pub type N1021 = NInt<U1021>;
    pub type U1022 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B0>;
    pub type P1022 = PInt<U1022>; pub type N1022 = NInt<U1022>;
    pub type U1023 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>, B1>;
    pub type P1023 = PInt<U1023>; pub type N1023 = NInt<U1023>;
    pub type U1024 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1024 = PInt<U1024>; pub type N1024 = NInt<U1024>;
    pub type U2048 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P2048 = PInt<U2048>; pub type N2048 = NInt<U2048>;
    pub type U4096 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P4096 = PInt<U4096>; pub type N4096 = NInt<U4096>;
    pub type U8192 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P8192 = PInt<U8192>; pub type N8192 = NInt<U8192>;
    pub type U16384 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P16384 = PInt<U16384>; pub type N16384 = NInt<U16384>;
    pub type U32768 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P32768 = PInt<U32768>; pub type N32768 = NInt<U32768>;
    pub type U65536 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P65536 = PInt<U65536>; pub type N65536 = NInt<U65536>;
    pub type U131072 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P131072 = PInt<U131072>; pub type N131072 = NInt<U131072>;
    pub type U262144 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P262144 = PInt<U262144>; pub type N262144 = NInt<U262144>;
    pub type U524288 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P524288 = PInt<U524288>; pub type N524288 = NInt<U524288>;
    pub type U1048576 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1048576 = PInt<U1048576>; pub type N1048576 = NInt<U1048576>;
    pub type U2097152 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P2097152 = PInt<U2097152>; pub type N2097152 = NInt<U2097152>;
    pub type U4194304 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P4194304 = PInt<U4194304>; pub type N4194304 = NInt<U4194304>;
    pub type U8388608 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P8388608 = PInt<U8388608>; pub type N8388608 = NInt<U8388608>;
    pub type U16777216 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P16777216 = PInt<U16777216>; pub type N16777216 = NInt<U16777216>;
    pub type U33554432 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P33554432 = PInt<U33554432>; pub type N33554432 = NInt<U33554432>;
    pub type U67108864 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P67108864 = PInt<U67108864>; pub type N67108864 = NInt<U67108864>;
    pub type U134217728 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P134217728 = PInt<U134217728>; pub type N134217728 = NInt<U134217728>;
    pub type U268435456 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P268435456 = PInt<U268435456>; pub type N268435456 = NInt<U268435456>;
    pub type U536870912 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P536870912 = PInt<U536870912>; pub type N536870912 = NInt<U536870912>;
    pub type U1073741824 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1073741824 = PInt<U1073741824>; pub type N1073741824 = NInt<U1073741824>;
    pub type U2147483648 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P2147483648 = PInt<U2147483648>; pub type N2147483648 = NInt<U2147483648>;
    pub type U4294967296 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P4294967296 = PInt<U4294967296>; pub type N4294967296 = NInt<U4294967296>;
    pub type U8589934592 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P8589934592 = PInt<U8589934592>; pub type N8589934592 = NInt<U8589934592>;
    pub type U17179869184 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P17179869184 = PInt<U17179869184>; pub type N17179869184 = NInt<U17179869184>;
    pub type U34359738368 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P34359738368 = PInt<U34359738368>; pub type N34359738368 = NInt<U34359738368>;
    pub type U68719476736 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P68719476736 = PInt<U68719476736>; pub type N68719476736 = NInt<U68719476736>;
    pub type U137438953472 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P137438953472 = PInt<U137438953472>; pub type N137438953472 = NInt<U137438953472>;
    pub type U274877906944 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P274877906944 = PInt<U274877906944>; pub type N274877906944 = NInt<U274877906944>;
    pub type U549755813888 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P549755813888 = PInt<U549755813888>; pub type N549755813888 = NInt<U549755813888>;
    pub type U1099511627776 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1099511627776 = PInt<U1099511627776>; pub type N1099511627776 = NInt<U1099511627776>;
    pub type U2199023255552 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P2199023255552 = PInt<U2199023255552>; pub type N2199023255552 = NInt<U2199023255552>;
    pub type U4398046511104 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P4398046511104 = PInt<U4398046511104>; pub type N4398046511104 = NInt<U4398046511104>;
    pub type U8796093022208 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P8796093022208 = PInt<U8796093022208>; pub type N8796093022208 = NInt<U8796093022208>;
    pub type U17592186044416 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P17592186044416 = PInt<U17592186044416>; pub type N17592186044416 = NInt<U17592186044416>;
    pub type U35184372088832 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P35184372088832 = PInt<U35184372088832>; pub type N35184372088832 = NInt<U35184372088832>;
    pub type U70368744177664 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P70368744177664 = PInt<U70368744177664>; pub type N70368744177664 = NInt<U70368744177664>;
    pub type U140737488355328 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P140737488355328 = PInt<U140737488355328>; pub type N140737488355328 = NInt<U140737488355328>;
    pub type U281474976710656 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P281474976710656 = PInt<U281474976710656>; pub type N281474976710656 = NInt<U281474976710656>;
    pub type U562949953421312 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P562949953421312 = PInt<U562949953421312>; pub type N562949953421312 = NInt<U562949953421312>;
    pub type U1125899906842624 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1125899906842624 = PInt<U1125899906842624>; pub type N1125899906842624 = NInt<U1125899906842624>;
    pub type U2251799813685248 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P2251799813685248 = PInt<U2251799813685248>; pub type N2251799813685248 = NInt<U2251799813685248>;
    pub type U4503599627370496 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P4503599627370496 = PInt<U4503599627370496>; pub type N4503599627370496 = NInt<U4503599627370496>;
    pub type U9007199254740992 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P9007199254740992 = PInt<U9007199254740992>; pub type N9007199254740992 = NInt<U9007199254740992>;
    pub type U18014398509481984 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P18014398509481984 = PInt<U18014398509481984>; pub type N18014398509481984 = NInt<U18014398509481984>;
    pub type U36028797018963968 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P36028797018963968 = PInt<U36028797018963968>; pub type N36028797018963968 = NInt<U36028797018963968>;
    pub type U72057594037927936 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P72057594037927936 = PInt<U72057594037927936>; pub type N72057594037927936 = NInt<U72057594037927936>;
    pub type U144115188075855872 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P144115188075855872 = PInt<U144115188075855872>; pub type N144115188075855872 = NInt<U144115188075855872>;
    pub type U288230376151711744 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P288230376151711744 = PInt<U288230376151711744>; pub type N288230376151711744 = NInt<U288230376151711744>;
    pub type U576460752303423488 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P576460752303423488 = PInt<U576460752303423488>; pub type N576460752303423488 = NInt<U576460752303423488>;
    pub type U1152921504606846976 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1152921504606846976 = PInt<U1152921504606846976>; pub type N1152921504606846976 = NInt<U1152921504606846976>;
    pub type U2305843009213693952 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P2305843009213693952 = PInt<U2305843009213693952>; pub type N2305843009213693952 = NInt<U2305843009213693952>;
    pub type U4611686018427387904 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P4611686018427387904 = PInt<U4611686018427387904>; pub type N4611686018427387904 = NInt<U4611686018427387904>;
    pub type U9223372036854775808 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type U10000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>;
    pub type P10000 = PInt<U10000>; pub type N10000 = NInt<U10000>;
    pub type U100000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type P100000 = PInt<U100000>; pub type N100000 = NInt<U100000>;
    pub type U1000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1000000 = PInt<U1000000>; pub type N1000000 = NInt<U1000000>;
    pub type U10000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P10000000 = PInt<U10000000>; pub type N10000000 = NInt<U10000000>;
    pub type U100000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P100000000 = PInt<U100000000>; pub type N100000000 = NInt<U100000000>;
    pub type U1000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1000000000 = PInt<U1000000000>; pub type N1000000000 = NInt<U1000000000>;
    pub type U10000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P10000000000 = PInt<U10000000000>; pub type N10000000000 = NInt<U10000000000>;
    pub type U100000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P100000000000 = PInt<U100000000000>; pub type N100000000000 = NInt<U100000000000>;
    pub type U1000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1000000000000 = PInt<U1000000000000>; pub type N1000000000000 = NInt<U1000000000000>;
    pub type U10000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P10000000000000 = PInt<U10000000000000>; pub type N10000000000000 = NInt<U10000000000000>;
    pub type U100000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P100000000000000 = PInt<U100000000000000>; pub type N100000000000000 = NInt<U100000000000000>;
    pub type U1000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1000000000000000 = PInt<U1000000000000000>; pub type N1000000000000000 = NInt<U1000000000000000>;
    pub type U10000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P10000000000000000 = PInt<U10000000000000000>; pub type N10000000000000000 = NInt<U10000000000000000>;
    pub type U100000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P100000000000000000 = PInt<U100000000000000000>; pub type N100000000000000000 = NInt<U100000000000000000>;
    pub type U1000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B1>, B1>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B1>, B1>, B0>, B1>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B0>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
    pub type P1000000000000000000 = PInt<U1000000000000000000>; pub type N1000000000000000000 = NInt<U1000000000000000000>;
    pub type U10000000000000000000 = UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B1>, B0>, B1>, B0>, B1>, B1>, B0>, B0>, B0>, B1>, B1>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B1>, B0>, B0>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B0>, B0>, B0>, B1>, B0>, B0>, B1>, B1>, B1>, B1>, B0>, B1>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>, B0>;
}
}

#[cfg(not(feature = "force_unix_path_separator"))]
mod generated {
    include!(env!("TYPENUM_BUILD_OP"));
    include!(env!("TYPENUM_BUILD_CONSTS"));
}

pub mod bit;
pub mod int;
pub mod marker_traits;
pub mod operator_aliases;
pub mod private;
pub mod type_operators;
pub mod uint;

pub mod array;

pub use consts::*;
pub use generated::consts;
pub use marker_traits::*;
pub use operator_aliases::*;
pub use type_operators::*;

pub use array::{ATerm, TArr};
pub use int::{NInt, PInt};
pub use uint::{UInt, UTerm};

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Greater`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
pub struct Greater;

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Less`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
pub struct Less;

/// A potential output from `Cmp`, this is the type equivalent to the enum variant
/// `core::cmp::Ordering::Equal`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug, Default)]
pub struct Equal;

/// Returns `core::cmp::Ordering::Greater`
impl Ord for Greater {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Greater
    }
}

/// Returns `core::cmp::Ordering::Less`
impl Ord for Less {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Less
    }
}

/// Returns `core::cmp::Ordering::Equal`
impl Ord for Equal {
    #[inline]
    fn to_ordering() -> Ordering {
        Ordering::Equal
    }
}

/// Asserts that two types are the same.
#[macro_export]
macro_rules! assert_type_eq {
    ($a:ty, $b:ty) => {
        let _: <$a as $crate::Same<$b>>::Output;
    };
}

/// Asserts that a type is `True`, aka `B1`.
#[macro_export]
macro_rules! assert_type {
    ($a:ty) => {
        let _: <$a as $crate::Same<True>>::Output;
    };
}
