// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: Legacy code. Should be removed after implementing the new implementation of https://hackmd.io/@younies/number_formatter_4x

pub mod compact_format;
pub mod compact_formatter;
pub mod format;
pub mod formatter;
pub mod long_compact_format;
pub mod long_compact_formatter;
pub mod long_format;
pub mod long_formatter;
pub mod options;

use crate::dimension::currency::CurrencyCode;
use crate::dimension::currency::legacy::options::Width;
use crate::dimension::provider::currency::essentials::{
    CurrencyEssentials, PatternSelection, PlaceholderValue,
};
use icu_pattern::DoublePlaceholderPattern;

impl<'a> CurrencyEssentials<'a> {
    pub(crate) fn name_and_pattern(
        &'a self,
        width: Width,
        currency: &'a CurrencyCode,
    ) -> (
        &'a str,
        Option<&'a DoublePlaceholderPattern>,
        PatternSelection,
    ) {
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
}
