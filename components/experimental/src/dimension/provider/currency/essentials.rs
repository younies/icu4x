// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerovec::VarZeroVec;

use icu_pattern::DoublePlaceholderPattern;

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
    pub standard: u8,
    pub standard_negative: Option<u8>,
    pub standard_alpha_next_to_number: u8,
    pub standard_alpha_next_to_number_negative: Option<u8>,
    pub accounting_positive: u8,
    pub accounting_negative: Option<u8>,
    pub accounting_alpha_next_to_number_positive: u8,
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
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::dimension::provider::currency::essentials))]
#[yoke(prove_covariance_manually)]
pub struct CurrencyEssentials<'data> {
    /// A packed list of distinct currency patterns referenced by [`PatternIndices`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, DoublePlaceholderPattern>,

    /// Indices into `patterns` for each formatting variant.
    pub indices: PatternIndices,
}

icu_provider::data_struct!(CurrencyEssentials<'_>, #[cfg(feature = "datagen")]);

impl<'a> CurrencyEssentials<'a> {
    /// Returns the standard pattern.
    pub fn standard_pattern(&self) -> Option<&DoublePlaceholderPattern> {
        self.patterns
            .get(self.indices.standard as usize)
            .or_else(|| self.patterns.get(0))
    }

    /// Returns the standard negative pattern if specified.
    pub fn standard_negative_pattern(&self) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .standard_negative
            .and_then(|idx| self.patterns.get(idx as usize))
    }

    /// Returns the `standard_alpha_next_to_number` pattern, falling back to `standard_pattern` if not present.
    pub fn standard_alpha_next_to_number_pattern(&self) -> Option<&DoublePlaceholderPattern> {
        self.patterns
            .get(self.indices.standard_alpha_next_to_number as usize)
            .or_else(|| self.standard_pattern())
    }

    /// Returns the `standard_alpha_next_to_number` negative pattern if specified, falling back to standard negative.
    pub fn standard_alpha_next_to_number_negative_pattern(
        &self,
    ) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .standard_alpha_next_to_number_negative
            .and_then(|idx| self.patterns.get(idx as usize))
            .or_else(|| self.standard_negative_pattern())
    }

    /// Returns the positive accounting pattern, falling back to `standard_pattern` if not present.
    pub fn accounting_positive_pattern(&self) -> Option<&DoublePlaceholderPattern> {
        self.patterns
            .get(self.indices.accounting_positive as usize)
            .or_else(|| self.standard_pattern())
    }

    /// Returns the negative accounting pattern if present.
    pub fn accounting_negative_pattern(&self) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .accounting_negative
            .and_then(|idx| self.patterns.get(idx as usize))
    }

    /// Returns the positive `accounting_alpha_next_to_number` pattern, falling back to accounting or standard.
    pub fn accounting_alpha_next_to_number_positive_pattern(
        &self,
    ) -> Option<&DoublePlaceholderPattern> {
        self.patterns
            .get(self.indices.accounting_alpha_next_to_number_positive as usize)
            .or_else(|| self.accounting_positive_pattern())
    }

    /// Returns the negative `accounting_alpha_next_to_number` pattern, falling back to `accounting_negative_pattern`.
    pub fn accounting_alpha_next_to_number_negative_pattern(
        &self,
    ) -> Option<&DoublePlaceholderPattern> {
        self.indices
            .accounting_alpha_next_to_number_negative
            .and_then(|idx| self.patterns.get(idx as usize))
            .or_else(|| self.accounting_negative_pattern())
    }
}
