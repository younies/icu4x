// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for currency symbols.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap};

use crate::dimension::currency::CurrencyCode;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

icu_provider::data_marker!(
    /// Currency symbol data needed for short and narrow currency formatting.
    CurrencySymbolsV1,
    CurrencySymbols<'static>
);

/// This type contains the symbol mappings for short and narrow currency formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, PartialEq, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[yoke(prove_covariance_manually)]
pub struct CurrencySymbols<'data> {
    /// A mapping from 3-letter currency ISO codes to their [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern_config_map: ZeroMap<'data, UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>,

    /// A list of symbols, including short (`symbol`) and narrow (`symbol-narrow`)
    /// currency symbols (such as `$`, `€`, `US$`), referenced by index.
    ///
    /// These values are retrieved using [`CurrencySymbol::Index`] stored in [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub symbols: VarZeroVec<'data, str>,

    /// The fallback currency pattern configuration used
    /// when a specific currency's pattern is not found in the currency patterns map.
    pub default_pattern_config: CurrencyPatternConfig,
}

icu_provider::data_struct!(CurrencySymbols<'_>, #[cfg(feature = "datagen")]);

#[zerovec::make_ule(PatternSelectionULE)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum PatternSelection {
    /// Use the standard pattern.
    #[default]
    Standard = 0,

    /// Use the `standard_alpha_next_to_number` pattern.
    StandardAlphaNextToNumber = 1,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[derive(Copy, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u16)]
pub enum CurrencySymbol {
    /// The index of the symbol in the symbols list.
    /// NOTE: the maximum value is `MAX_SYMBOL_INDEX` which is 2045 (`0b0111_1111_1101`).
    Index(u16),

    /// The symbol is the ISO code.
    ISO,
}

#[derive(Debug, Clone, Copy)]
pub enum Width {
    /// Format the currency with the standard (short) currency symbol.
    ///
    /// For example, 1 USD formats as "$1.00" in en-US and "US$1" in most other locales.
    Short,

    /// Format the currency with the narrow currency symbol.
    ///
    /// The narrow symbol may be ambiguous, so it should be evident from context which
    /// currency is being represented.
    ///
    /// For example, 1 USD formats as "$1.00" in most locales.
    Narrow,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::symbols))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct CurrencyPatternConfig {
    /// Indicates which pattern to use for short currency formatting.
    pub short_pattern_selection: PatternSelection,

    /// Indicates which pattern to use for narrow currency formatting.
    pub narrow_pattern_selection: PatternSelection,

    /// The symbol for short currency formatting.
    /// If the value is `None`, this means that the short pattern does not have a symbol.
    pub short_symbol: Option<CurrencySymbol>,

    /// The symbol for narrow currency formatting.
    /// If the value is `None`, this means that the narrow pattern does not have a symbol.
    pub narrow_symbol: Option<CurrencySymbol>,
}

impl<'a> CurrencySymbols<'a> {
    /// Returns the formatted currency name/symbol,
    /// the currency pattern for the given width and currency,
    /// and the pattern selection.
    pub fn get(&'a self, width: Width, currency: &'a CurrencyCode) -> (&'a str, PatternSelection) {
        let config = self
            .pattern_config_map
            .get_copied(&currency.0.to_unvalidated())
            .unwrap_or(self.default_pattern_config);

        let symbol = match width {
            Width::Short => config.short_symbol,
            Width::Narrow => config.narrow_symbol,
        };

        let symbol = match symbol {
            Some(CurrencySymbol::Index(index)) => self.symbols.get(index.into()),
            Some(CurrencySymbol::ISO) | None => None,
        }
        .unwrap_or(currency.0.as_str());

        let pattern_selection = match width {
            Width::Short => config.short_pattern_selection,
            Width::Narrow => config.narrow_pattern_selection,
        };

        (symbol, pattern_selection)
    }
}
