// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::{VarZeroVec, ZeroMap};

use icu_pattern::DoublePlaceholderPattern;

use crate::dimension::currency::CurrencyCode;
use crate::dimension::currency::options::Width;

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
    /// Essential currency data needed for currency formatting. For example, currency patterns.
    CurrencyEssentialsV1,
    CurrencyEssentials<'static>
);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
// TODO: Pack these 8 distinct index fields into a single 32-bit integer bitfield (3 bits per index) to minimize stack size.
pub struct PatternIndices {
    /// Standard pattern.
    pub standard: u8,
    /// Standard negative pattern. Falls back to formatting negative number with standard pattern at runtime if missing.
    pub standard_negative: Option<u8>,
    /// Standard alpha next to number pattern. Falls back to `standard` at datagen time if missing.
    pub standard_alpha_next_to_number: u8,
    /// Standard alpha next to number negative pattern. Falls back to `standard_negative` at runtime if missing.
    pub standard_alpha_next_to_number_negative: Option<u8>,
    /// Positive accounting pattern. Falls back to `standard` at datagen time if missing.
    pub accounting_positive: u8,
    /// Negative accounting pattern. Falls back to `standard_negative` at runtime if missing.
    pub accounting_negative: Option<u8>,
    /// Positive accounting alpha next to number pattern. Falls back to `accounting_positive` at datagen time if missing.
    pub accounting_alpha_next_to_number_positive: u8,
    /// Negative accounting alpha next to number pattern. Falls back to `accounting_negative` at runtime if missing.
    pub accounting_alpha_next_to_number_negative: Option<u8>,
}

/// This type contains all of the essential data for currency formatting.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, PartialEq, Debug, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path =  icu_experimental::dimension::provider::currency::essentials))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentials<'data> {
    /// A mapping from 3-letter currency ISO codes to their [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern_config_map: ZeroMap<'data, UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>,

    /// A packed list of distinct currency patterns referenced by [`PatternIndices`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, DoublePlaceholderPattern>,

    /// Indices into `patterns` for each formatting variant.
    pub indices: PatternIndices,

    /// A list of placeholders (strings), such as currency symbols, referenced by index.
    ///
    /// These values are retrieved using [`PlaceholderValue::Index`] stored in [`CurrencyPatternConfig`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub placeholders: VarZeroVec<'data, str>,

    /// The fallback currency pattern configuration used
    /// when a specific currency's pattern is not found in the currency patterns map.
    pub default_pattern_config: CurrencyPatternConfig,
}

icu_provider::data_struct!(CurrencyEssentials<'_>, #[cfg(feature = "datagen")]);

#[zerovec::make_ule(PatternSelectionULE)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
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
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
#[derive(Copy, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u16)]
pub enum PlaceholderValue {
    /// The index of the placeholder in the placeholders list.
    /// NOTE: the maximum value is `MAX_PLACEHOLDER_INDEX` which is 2045 (`0b0111_1111_1101`).
    Index(u16),

    /// The placeholder is the ISO code.
    ISO,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
#[derive(Copy, Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct CurrencyPatternConfig {
    /// Indicates which pattern to use for short currency formatting.
    pub short_pattern_selection: PatternSelection,

    /// Indicates which pattern to use for narrow currency formatting.
    pub narrow_pattern_selection: PatternSelection,

    /// The placeholder value for short currency formatting.
    /// If the value is `None`, this means that the short pattern does not have a placeholder.
    pub short_placeholder_value: Option<PlaceholderValue>,

    /// The placeholder value for narrow currency formatting.
    /// If the value is `None`, this means that the narrow pattern does not have a placeholder.
    pub narrow_placeholder_value: Option<PlaceholderValue>,
}

// Fallback pattern: "{1}{0}" (currency followed by number, e.g. "$10")
//
// Even though the baked data handles the fallback at data generation time,
// we have the fallback here for users feeding their own data without
// handling the fallback logic in their data generation.
const FALLBACK_PATTERN: &DoublePlaceholderPattern =
    DoublePlaceholderPattern::from_ref_store_unchecked("\x03\x02");

impl<'a> CurrencyEssentials<'a> {
    /// Returns the formatted currency name/symbol,
    /// the currency pattern for the given width and currency,
    /// and the pattern selection.
    pub(crate) fn name_and_pattern(
        &'a self,
        width: Width,
        currency: &'a CurrencyCode,
    ) -> (&'a str, &'a DoublePlaceholderPattern, PatternSelection) {
        let config = self
            .pattern_config_map
            .get_copied(&currency.0.to_unvalidated())
            .unwrap_or(self.default_pattern_config);

        let placeholder_val = match width {
            Width::Short => config.short_placeholder_value,
            Width::Narrow => config.narrow_placeholder_value,
        };

        let currency = match placeholder_val {
            Some(PlaceholderValue::Index(index)) => self.placeholders.get(index.into()),
            Some(PlaceholderValue::ISO) | None => None,
        }
        .unwrap_or(currency.0.as_str());

        let pattern_selection = match width {
            Width::Short => config.short_pattern_selection,
            Width::Narrow => config.narrow_pattern_selection,
        };

        let pattern = match pattern_selection {
            PatternSelection::Standard => self.standard_pattern(),
            PatternSelection::StandardAlphaNextToNumber => {
                self.standard_alpha_next_to_number_pattern()
            }
        };

        (currency, pattern, pattern_selection)
    }

    /// Returns the standard pattern.
    ///
    /// If the standard pattern is missing (which should only happen in corrupt
    /// or incomplete custom data), this returns a default safety pattern `"{1}{0}"`.
    /// Note that this is a safety default, not a CLDR-defined fallback.
    pub fn standard_pattern(&self) -> &DoublePlaceholderPattern {
        debug_assert!(
            (self.indices.standard as usize) < self.patterns.len(),
            "Standard pattern index {} is out of bounds for patterns of length {}",
            self.indices.standard,
            self.patterns.len()
        );
        self.patterns
            .get(self.indices.standard as usize)
            .unwrap_or(FALLBACK_PATTERN)
    }

    /// Returns the standard negative pattern if specified.
    pub fn standard_negative_pattern(&self) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .standard_negative
            .and_then(|idx| self.patterns.get(idx as usize))
    }

    /// Returns the `standard_alpha_next_to_number` pattern.
    ///
    /// Fallback hierarchy:
    /// `standard_alpha_next_to_number` -> `standard`
    ///
    /// Even though the baked data handles the fallback at data generation time,
    /// we have the fallback here for users feeding their own data without
    /// handling the fallback logic in their data generation.
    pub fn standard_alpha_next_to_number_pattern(&self) -> &DoublePlaceholderPattern {
        self.patterns
            .get(self.indices.standard_alpha_next_to_number as usize)
            .unwrap_or_else(|| self.standard_pattern())
    }

    /// Returns the `standard_alpha_next_to_number` negative pattern if specified, falling back to standard negative.
    ///
    /// Fallback hierarchy:
    /// `standard_alpha_next_to_number_negative` -> `standard_negative`
    pub fn standard_alpha_next_to_number_negative_pattern(
        &self,
    ) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .standard_alpha_next_to_number_negative
            .and_then(|idx| self.patterns.get(idx as usize))
            .or_else(|| self.standard_negative_pattern())
    }

    /// Returns the positive accounting pattern.
    ///
    /// Fallback hierarchy:
    /// `accounting_positive` -> `standard`
    ///
    /// Even though the baked data handles the fallback at data generation time,
    /// we have the fallback here for users feeding their own data without
    /// handling the fallback logic in their data generation.
    pub fn accounting_positive_pattern(&self) -> &DoublePlaceholderPattern {
        self.patterns
            .get(self.indices.accounting_positive as usize)
            .unwrap_or_else(|| self.standard_pattern())
    }

    /// Returns the negative accounting pattern if present, falling back to standard negative.
    ///
    /// Fallback hierarchy:
    /// `accounting_negative` -> `standard_negative`
    pub fn accounting_negative_pattern(&self) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .accounting_negative
            .and_then(|idx| self.patterns.get(idx as usize))
            .or_else(|| self.standard_negative_pattern())
    }

    /// Returns the positive `accounting_alpha_next_to_number` pattern.
    ///
    /// Fallback hierarchy:
    /// `accounting_alpha_next_to_number_positive` -> `standard_alpha_next_to_number` -> `standard`
    ///
    /// Even though the baked data handles the fallback at data generation time,
    /// we have the fallback here for users feeding their own data without
    /// handling the fallback logic in their data generation.
    pub fn accounting_alpha_next_to_number_positive_pattern(&self) -> &DoublePlaceholderPattern {
        self.patterns
            .get(self.indices.accounting_alpha_next_to_number_positive as usize)
            .unwrap_or_else(|| self.standard_alpha_next_to_number_pattern())
    }

    /// Returns the negative `accounting_alpha_next_to_number` pattern, falling back to `accounting_negative_pattern`.
    ///
    /// Fallback hierarchy:
    /// `accounting_alpha_next_to_number_negative` -> `accounting_negative` -> `standard_negative`
    pub fn accounting_alpha_next_to_number_negative_pattern(
        &self,
    ) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .accounting_alpha_next_to_number_negative
            .and_then(|idx| self.patterns.get(idx as usize))
            .or_else(|| self.accounting_negative_pattern())
    }
}
