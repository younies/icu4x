// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing, manipulating, and serializing Unicode Language and Locale Identifiers.
//!
//! This module is published as its own crate ([`icu_locale_core`](https://docs.rs/icu_locale_core/latest/icu_locale_core/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! The module provides algorithms for parsing a string into a well-formed language or locale identifier
//! as defined by [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]. Additionally
//! the module provides [`preferences`] interface for operations on locale preferences and conversions
//! from and to locale unicode extensions.
//!
//! [`Locale`] is the most common structure to use for storing information about a language,
//! script, region, variants and extensions. In almost all cases, this struct should be used as the
//! base unit for all locale management operations.
//!
//! [`LanguageIdentifier`] is a strict subset of [`Locale`] which can be useful in a narrow range of
//! cases where [`Unicode Extensions`] are not relevant.
//!
//! If in doubt, use [`Locale`].
//!
//! # Examples
//!
//! ```
//! use icu::locale::Locale;
//! use icu::locale::{
//!     locale,
//!     subtags::{language, region},
//! };
//!
//! let mut loc: Locale = locale!("en-US");
//!
//! assert_eq!(loc.id.language, language!("en"));
//! assert_eq!(loc.id.script, None);
//! assert_eq!(loc.id.region, Some(region!("US")));
//! assert_eq!(loc.id.variants.len(), 0);
//!
//! loc.id.region = Some(region!("GB"));
//!
//! assert_eq!(loc, locale!("en-GB"));
//! ```
//!
//! For more details, see [`Locale`] and [`LanguageIdentifier`].
//!
//! [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]: https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers
//! [`ICU4X`]: ../icu/index.html
//! [`Unicode Extensions`]: extensions

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod helpers;

mod data;
mod langid;
mod locale;
mod macros;
mod parser;
mod shortvec;

pub use data::DataLocale;
pub use langid::LanguageIdentifier;
pub use locale::Locale;
pub use parser::ParseError;

pub mod extensions;
#[macro_use]
pub mod subtags;
pub mod preferences;
pub mod zerovec;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "databake")]
mod databake;
