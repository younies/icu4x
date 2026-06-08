// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;
use core::marker::PhantomData;
use fixed_decimal::Decimal as FixedDecimal;
use icu_decimal::{
    DecimalFormatter, DecimalFormatterPreferences, options::DecimalFormatterOptions,
    preferences::CompactDecimalFormatterPreferences,
};
use icu_locale_core::preferences::{define_preferences, prefs_convert};
use icu_plurals::{PluralRules, PluralRulesPreferences};
use icu_provider::prelude::*;
use writeable::Writeable;

use super::CurrencyCode;
use super::options::{CurrencyFormatterOptions, Width};
use crate::dimension::provider::currency::{
    compact::ShortCurrencyCompactV1, essentials::CurrencyEssentialsV1,
    extended::CurrencyExtendedDataV1, patterns::CurrencyPatternsDataV1,
};

extern crate alloc;

define_preferences!(
    /// The preferences for currency formatting.
    [Copy]
    CurrencyFormatterPreferences,
    {
        /// The user's preferred numbering system.
        numbering_system: crate::dimension::preferences::NumberingSystem
    }
);

prefs_convert!(CurrencyFormatterPreferences, DecimalFormatterPreferences, {
    numbering_system
});
prefs_convert!(CurrencyFormatterPreferences, PluralRulesPreferences);

define_preferences!(
    /// The preferences for compact currency formatting.
    [Copy]
    CompactCurrencyFormatterPreferences,
    {
        /// The user's preferred numbering system.
        numbering_system: crate::dimension::preferences::NumberingSystem
    }
);

prefs_convert!(
    CompactCurrencyFormatterPreferences,
    DecimalFormatterPreferences,
    { numbering_system }
);
prefs_convert!(
    CompactCurrencyFormatterPreferences,
    CompactDecimalFormatterPreferences
);
prefs_convert!(CompactCurrencyFormatterPreferences, PluralRulesPreferences);

/// Trait for value representations in currency formatting.
pub trait ValueRepresentation {
    type InternalData;
}

/// Standard decimal value representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal;
impl ValueRepresentation for Decimal {
    type InternalData = DecimalCurrencyData;
}

/// Compact value representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Compact;
impl ValueRepresentation for Compact {
    type InternalData = CompactCurrencyData;
}

/// Scientific value representation (stub for future implementation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scientific;
impl ValueRepresentation for Scientific {
    type InternalData = (); // Placeholder
}

#[derive(Debug)]
pub enum DecimalCurrencyData {
    Standard {
        essential: DataPayload<CurrencyEssentialsV1>,
        decimal_formatter: DecimalFormatter,
        options: CurrencyFormatterOptions,
    },
    Long {
        extended: DataPayload<CurrencyExtendedDataV1>,
        patterns: DataPayload<CurrencyPatternsDataV1>,
        decimal_formatter: DecimalFormatter,
        plural_rules: PluralRules,
    },
}

#[derive(Debug)]
pub enum CompactCurrencyData {
    Standard {
        _short_currency_compact: DataPayload<ShortCurrencyCompactV1>,
        essential: DataPayload<CurrencyEssentialsV1>,
        decimal_formatter: DecimalFormatter,
        compact_data: DataPayload<icu_decimal::provider::DecimalCompactShortV1>,
        plural_rules: PluralRules,
        options: CurrencyFormatterOptions,
    },
    Long {
        extended: DataPayload<CurrencyExtendedDataV1>,
        patterns: DataPayload<CurrencyPatternsDataV1>,
        decimal_formatter: DecimalFormatter,
        compact_data: DataPayload<icu_decimal::provider::DecimalCompactLongV1>,
        plural_rules: PluralRules,
    },
}

/// A generic currency formatter that can format monetary values using different representations.
///
/// This struct implements Option 1 exactly as designed in `design_docs/icu4x/number_formatter.md`.
#[derive(Debug)]
pub struct CurrencyFormatter<T: ValueRepresentation> {
    data: T::InternalData,
    _marker: PhantomData<T>,
}

impl CurrencyFormatter<Decimal> {
    /// Creates a currency formatter for short formatting.
    #[cfg(feature = "compiled_data")]
    pub fn try_new_short(prefs: CurrencyFormatterPreferences) -> Result<Self, DataError> {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_formatter =
            DecimalFormatter::try_new((&prefs).into(), DecimalFormatterOptions::default())?;
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        let options = CurrencyFormatterOptions {
            width: Width::Short,
        };
        Ok(Self {
            data: DecimalCurrencyData::Standard {
                essential,
                decimal_formatter,
                options,
            },
            _marker: PhantomData,
        })
    }

    /// Creates a currency formatter for narrow formatting.
    #[cfg(feature = "compiled_data")]
    pub fn try_new_narrow(prefs: CurrencyFormatterPreferences) -> Result<Self, DataError> {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_formatter =
            DecimalFormatter::try_new((&prefs).into(), DecimalFormatterOptions::default())?;
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        let options = CurrencyFormatterOptions {
            width: Width::Narrow,
        };
        Ok(Self {
            data: DecimalCurrencyData::Standard {
                essential,
                decimal_formatter,
                options,
            },
            _marker: PhantomData,
        })
    }

    /// Creates a currency formatter for long formatting.
    #[cfg(feature = "compiled_data")]
    pub fn try_new_long(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        let locale = CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);
        let decimal_formatter =
            DecimalFormatter::try_new((&prefs).into(), DecimalFormatterOptions::default())?;

        let marker_attributes = DataMarkerAttributes::try_from_str(currency_code.0.as_str())
            .map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;

        let extended = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    &locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = crate::provider::Baked.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        Ok(Self {
            data: DecimalCurrencyData::Long {
                extended,
                patterns,
                decimal_formatter,
                plural_rules,
            },
            _marker: PhantomData,
        })
    }

    /// Formats a [`FixedDecimal`] value for the given currency code.
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
        currency_code: &'l CurrencyCode,
    ) -> impl Writeable + Display + 'l {
        match &self.data {
            DecimalCurrencyData::Standard {
                essential,
                decimal_formatter,
                options,
            } => {
                let (currency_str, pattern, _pattern_selection) = essential
                    .get()
                    .name_and_pattern(options.width, currency_code);

                decimal_formatter.format_sign(
                    value.sign,
                    pattern.interpolate((
                        decimal_formatter
                            .format_unsigned(icu_decimal::Cow::Borrowed(&value.absolute)),
                        currency_str,
                    )),
                )
            }
            DecimalCurrencyData::Long {
                extended,
                patterns,
                decimal_formatter,
                plural_rules,
            } => {
                let operands = value.into();
                let display_name = extended.get().display_names.get(operands, plural_rules);
                let pattern = patterns.get().patterns.get(operands, plural_rules);

                decimal_formatter.format_sign(
                    value.sign,
                    pattern.interpolate((
                        decimal_formatter
                            .format_unsigned(icu_decimal::Cow::Borrowed(&value.absolute)),
                        display_name,
                    )),
                )
            }
        }
    }
}

impl CurrencyFormatter<Compact> {
    /// Creates a currency formatter for short compact formatting.
    #[cfg(feature = "compiled_data")]
    pub fn try_new_short(prefs: CompactCurrencyFormatterPreferences) -> Result<Self, DataError> {
        let short_locale = ShortCurrencyCompactV1::make_locale(prefs.locale_preferences);
        let _short_currency_compact = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&short_locale),
                ..Default::default()
            })?
            .payload;

        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        let decimal_formatter = DecimalFormatter::try_new((&prefs).into(), Default::default())?;

        let compact_data = DataProvider::<icu_decimal::provider::DecimalCompactShortV1>::load(
            &icu_decimal::provider::Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_locale(
                    &icu_decimal::provider::DecimalCompactShortV1::make_locale(
                        prefs.locale_preferences,
                    ),
                ),
                ..Default::default()
            },
        )?
        .payload
        .cast();

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;
        let options = CurrencyFormatterOptions {
            width: Width::Short,
        };

        Ok(Self {
            data: CompactCurrencyData::Standard {
                _short_currency_compact,
                essential,
                decimal_formatter,
                compact_data,
                plural_rules,
                options,
            },
            _marker: PhantomData,
        })
    }

    /// Creates a currency formatter for narrow compact formatting.
    #[cfg(feature = "compiled_data")]
    pub fn try_new_narrow(prefs: CompactCurrencyFormatterPreferences) -> Result<Self, DataError> {
        let short_locale = ShortCurrencyCompactV1::make_locale(prefs.locale_preferences);
        let _short_currency_compact = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&short_locale),
                ..Default::default()
            })?
            .payload;

        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let essential = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&locale),
                ..Default::default()
            })?
            .payload;

        let decimal_formatter = DecimalFormatter::try_new((&prefs).into(), Default::default())?;

        let compact_data = DataProvider::<icu_decimal::provider::DecimalCompactShortV1>::load(
            &icu_decimal::provider::Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_locale(
                    &icu_decimal::provider::DecimalCompactShortV1::make_locale(
                        prefs.locale_preferences,
                    ),
                ),
                ..Default::default()
            },
        )?
        .payload
        .cast();

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;
        let options = CurrencyFormatterOptions {
            width: Width::Narrow,
        };

        Ok(Self {
            data: CompactCurrencyData::Standard {
                _short_currency_compact,
                essential,
                decimal_formatter,
                compact_data,
                plural_rules,
                options,
            },
            _marker: PhantomData,
        })
    }

    /// Creates a currency formatter for long compact formatting.
    #[cfg(feature = "compiled_data")]
    pub fn try_new_long(
        prefs: CompactCurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        let decimal_formatter = DecimalFormatter::try_new((&prefs).into(), Default::default())?;

        let compact_data = DataProvider::<icu_decimal::provider::DecimalCompactLongV1>::load(
            &icu_decimal::provider::Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_locale(
                    &icu_decimal::provider::DecimalCompactLongV1::make_locale(
                        prefs.locale_preferences,
                    ),
                ),
                ..Default::default()
            },
        )?
        .payload
        .cast();

        let marker_attributes = DataMarkerAttributes::try_from_str(currency_code.0.as_str())
            .map_err(|_| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("failed to get data marker attribute from a `CurrencyCode`")
            })?;

        let locale = &CurrencyPatternsDataV1::make_locale(prefs.locale_preferences);

        let extended = crate::provider::Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    marker_attributes,
                    locale,
                ),
                ..Default::default()
            })?
            .payload;

        let patterns = crate::provider::Baked.load(Default::default())?.payload;

        let plural_rules = PluralRules::try_new_cardinal((&prefs).into())?;

        Ok(Self {
            data: CompactCurrencyData::Long {
                extended,
                patterns,
                decimal_formatter,
                compact_data,
                plural_rules,
            },
            _marker: PhantomData,
        })
    }

    /// Formats a [`FixedDecimal`] value for the given currency code.
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
        currency_code: &'l CurrencyCode,
    ) -> impl Writeable + Display + 'l {
        match &self.data {
            CompactCurrencyData::Standard {
                _short_currency_compact,
                essential,
                decimal_formatter,
                compact_data,
                plural_rules,
                options,
            } => {
                let (currency_placeholder, pattern, _pattern_selection) = essential
                    .get()
                    .name_and_pattern(options.width, currency_code);

                let (compact_pattern, significand) = compact_data
                    .get()
                    .get_pattern_and_significand(&value.absolute, plural_rules);

                decimal_formatter.format_sign(
                    value.sign,
                    pattern.interpolate((
                        compact_pattern
                            .unwrap_or(icu_pattern::SinglePlaceholderPattern::PASS_THROUGH)
                            .interpolate([decimal_formatter
                                .format_unsigned(icu_decimal::Cow::Owned(significand))]),
                        currency_placeholder,
                    )),
                )
            }
            CompactCurrencyData::Long {
                extended,
                patterns,
                decimal_formatter,
                compact_data,
                plural_rules,
            } => {
                let operands = value.into();
                let display_name = extended.get().display_names.get(operands, plural_rules);
                let pattern = patterns.get().patterns.get(operands, plural_rules);

                let (compact_pattern, significand) = compact_data
                    .get()
                    .get_pattern_and_significand(&value.absolute, plural_rules);

                decimal_formatter.format_sign(
                    value.sign,
                    pattern.interpolate((
                        compact_pattern
                            .unwrap_or(icu_pattern::SinglePlaceholderPattern::PASS_THROUGH)
                            .interpolate([decimal_formatter
                                .format_unsigned(icu_decimal::Cow::Owned(significand))]),
                        display_name,
                    )),
                )
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "compiled_data")]
mod tests {
    use super::*;
    use icu_locale::locale;
    use tinystr::tinystr;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_decimal_short() {
        let prefs = locale!("en-US").into();
        let fmt = CurrencyFormatter::<Decimal>::try_new_short(prefs).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        assert_writeable_eq!(
            fmt.format_fixed_decimal(&value, &currency_code),
            "$12,345.67"
        );
    }

    #[test]
    fn test_decimal_narrow() {
        let prefs = locale!("en-US").into();
        let fmt = CurrencyFormatter::<Decimal>::try_new_narrow(prefs).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        assert_writeable_eq!(
            fmt.format_fixed_decimal(&value, &currency_code),
            "$12,345.67"
        );
    }

    #[test]
    fn test_decimal_long() {
        let prefs = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::<Decimal>::try_new_long(prefs, &currency_code).unwrap();
        let value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt.format_fixed_decimal(&value, &currency_code),
            "12,345.67 US dollars"
        );
    }

    #[test]
    fn test_compact_short() {
        let prefs = locale!("en-US").into();
        let fmt = CurrencyFormatter::<Compact>::try_new_short(prefs).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        assert_writeable_eq!(fmt.format_fixed_decimal(&value, &currency_code), "$12K");
    }

    #[test]
    fn test_compact_narrow() {
        let prefs = locale!("en-US").into();
        let fmt = CurrencyFormatter::<Compact>::try_new_narrow(prefs).unwrap();
        let value = "12345.67".parse().unwrap();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        assert_writeable_eq!(fmt.format_fixed_decimal(&value, &currency_code), "$12K");
    }

    #[test]
    fn test_compact_long() {
        let prefs = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::<Compact>::try_new_long(prefs, &currency_code).unwrap();
        let value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt.format_fixed_decimal(&value, &currency_code),
            "12 thousand US dollars"
        );
    }
}
