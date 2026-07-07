// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::Display;
use core::marker::PhantomData;

use crate::dimension::provider::currency::essentials::CurrencyEssentialsV1;
use fixed_decimal::Decimal as FixedDecimal;
use icu_decimal::{
    DecimalFormatter, DecimalFormatterPreferences, options::DecimalFormatterOptions,
};
use icu_plurals::PluralRules;
use icu_provider::prelude::*;
use writeable::Writeable;

use super::formatter::{
    Compact, CurrencyFormatter, CurrencyFormatterPreferences, load_with_fallback,
};
use super::{CurrencyCode, options::Width};
use icu_pattern::DoublePlaceholderPattern;

extern crate alloc;

/// Internal data for compact currency formatting.
#[derive(Debug)]
pub struct CompactData {
    pub(crate) compact_data: DataPayload<icu_decimal::provider::DecimalCompactShortV1>,
    pub(crate) plural_rules: PluralRules,
}

impl CurrencyFormatter<Compact> {
    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_short: skip,
            try_new_short_with_buffer_provider,
            try_new_short_unstable,
            Self
        ]
    );

    icu_provider::gen_buffer_data_constructors!(
        (prefs: CurrencyFormatterPreferences, currency_code: &CurrencyCode) -> error: DataError,
        functions: [
            try_new_narrow: skip,
            try_new_narrow_with_buffer_provider,
            try_new_narrow_unstable,
            Self
        ]
    );

    /// Creates a new [`CurrencyFormatter`] for short compact formatting from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_short(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter =
            DecimalFormatter::try_new(decimal_prefs, DecimalFormatterOptions::default())?;

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential =
            load_with_fallback::<CurrencyEssentialsV1>(&crate::provider::Baked, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

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

        Ok(Self {
            width: Width::Short,
            essential,
            decimal_formatter,
            bound_currency: *currency_code,
            rep_data: CompactData {
                compact_data,
                plural_rules,
            },
            _marker: PhantomData,
        })
    }

    /// Creates a new [`CurrencyFormatter`] for narrow compact formatting from compiled locale data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn try_new_narrow(
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError> {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter =
            DecimalFormatter::try_new(decimal_prefs, DecimalFormatterOptions::default())?;

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential =
            load_with_fallback::<CurrencyEssentialsV1>(&crate::provider::Baked, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

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

        Ok(Self {
            width: Width::Narrow,
            essential,
            decimal_formatter,
            bound_currency: *currency_code,
            rep_data: CompactData {
                compact_data,
                plural_rules,
            },
            _marker: PhantomData,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_short)]
    pub fn try_new_short_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<icu_decimal::provider::DecimalCompactShortV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter = DecimalFormatter::try_new_unstable(
            provider,
            decimal_prefs,
            DecimalFormatterOptions::default(),
        )?;

        let compact_data = DataProvider::<icu_decimal::provider::DecimalCompactShortV1>::load(
            provider,
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

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential = load_with_fallback::<CurrencyEssentialsV1>(provider, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

        Ok(Self {
            width: Width::Short,
            essential,
            decimal_formatter,
            bound_currency: *currency_code,
            rep_data: CompactData {
                compact_data,
                plural_rules,
            },
            _marker: PhantomData,
        })
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::try_new_narrow)]
    pub fn try_new_narrow_unstable<D>(
        provider: &D,
        prefs: CurrencyFormatterPreferences,
        currency_code: &CurrencyCode,
    ) -> Result<Self, DataError>
    where
        D: ?Sized
            + DataProvider<CurrencyEssentialsV1>
            + DataProvider<icu_decimal::provider::DecimalCompactShortV1>
            + DataProvider<icu_decimal::provider::DecimalSymbolsV1>
            + DataProvider<icu_decimal::provider::DecimalDigitsV1>
            + DataProvider<icu_plurals::provider::PluralsCardinalV1>,
    {
        let locale = CurrencyEssentialsV1::make_locale(prefs.locale_preferences);
        let decimal_prefs = DecimalFormatterPreferences::from(&prefs);
        let decimal_formatter = DecimalFormatter::try_new_unstable(
            provider,
            decimal_prefs,
            DecimalFormatterOptions::default(),
        )?;

        let compact_data = DataProvider::<icu_decimal::provider::DecimalCompactShortV1>::load(
            provider,
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

        let plural_rules = PluralRules::try_new_cardinal_unstable(provider, (&prefs).into())?;

        let req_id = decimal_prefs.nu_id(&locale);
        let default_id = DataIdentifierBorrowed::for_locale(&locale);
        let ids = req_id.into_iter().chain(core::iter::once(default_id));
        let essential = load_with_fallback::<CurrencyEssentialsV1>(provider, ids)?.payload;

        if essential.get().standard_pattern().is_none() {
            return Err(DataError::custom("missing standard pattern"));
        }

        Ok(Self {
            width: Width::Narrow,
            essential,
            decimal_formatter,
            bound_currency: *currency_code,
            rep_data: CompactData {
                compact_data,
                plural_rules,
            },
            _marker: PhantomData,
        })
    }

    /// Formats in the compact format a [`FixedDecimal`] value.
    ///
    /// # Examples
    /// ```
    /// use icu::experimental::dimension::currency::formatter::{Compact, CurrencyFormatter};
    /// use icu::experimental::dimension::currency::CurrencyCode;
    /// use icu::locale::locale;
    /// use tinystr::*;
    /// use writeable::assert_writeable_eq;
    ///
    /// let locale = locale!("en-US").into();
    /// let currency_code = CurrencyCode(tinystr!(3, "USD"));
    /// let fmt = CurrencyFormatter::<Compact>::try_new_short(locale, &currency_code).unwrap();
    /// let value = "12345.67".parse().unwrap();
    /// assert_writeable_eq!(fmt.format_fixed_decimal(&value), "$12K");
    /// ```
    pub fn format_fixed_decimal<'l>(
        &'l self,
        value: &'l FixedDecimal,
    ) -> impl Writeable + Display + 'l {
        let (currency_placeholder, pattern, _pattern_selection) = self
            .essential
            .get()
            .name_and_pattern(self.width, &self.bound_currency);

        let pattern = pattern.unwrap_or_else(|| <&DoublePlaceholderPattern>::default());

        // TODO(#8184): We currently use a "gluing" approach (combining standard currency patterns with
        // compact decimal output) instead of resolving double-placeholder patterns from ShortCurrencyCompactV1.
        // We are waiting for the conclusion of an ongoing CLDR/TR35 investigation regarding compact currency
        // pattern structure and usage. Once resolved, if CLDR decides to use explicit double-placeholder data,
        // we can introduce ShortCurrencyCompactV1; otherwise we continue with this gluing approach.

        let (compact_pattern, significand) = self
            .rep_data
            .compact_data
            .get()
            .get_pattern_and_significand(&value.absolute, &self.rep_data.plural_rules);

        // Per UTS #35 (LDML / TR35 Part 3: Numbers, Section 3.2.1), when a pattern does not specify an
        // explicit negative subpattern, the default negative format is formed by prepending the localized
        // minus sign to the entire positive pattern (e.g., `-¤#,##0` producing `-$12K`).
        // Therefore, `format_sign` is applied as the outermost wrapper around the glued currency string so
        // that the minus sign modifies the full monetary expression rather than just the numeric significand.
        self.decimal_formatter.format_sign(
            value.sign,
            pattern.interpolate((
                compact_pattern
                    .unwrap_or(icu_pattern::SinglePlaceholderPattern::PASS_THROUGH)
                    .interpolate([self
                        .decimal_formatter
                        .format_unsigned(icu_decimal::Cow::Owned(significand))]),
                currency_placeholder,
            )),
        )
    }
}
