// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{
        CurrencyCode,
        formatter::{Compact, CurrencyFormatter, CurrencyFormatterPreferences},
    };

    #[test]
    pub fn test_en_us() {
        let prefs: CurrencyFormatterPreferences = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let fmt = CurrencyFormatter::<Compact>::try_new_short(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "$12K");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-$12K");
    }

    #[test]
    pub fn test_fr_fr() {
        let prefs: CurrencyFormatterPreferences = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));
        let fmt = CurrencyFormatter::<Compact>::try_new_short(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "12\u{a0}k\u{a0}€");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-12\u{a0}k\u{a0}€");
    }

    #[test]
    pub fn test_zh_cn() {
        let prefs: CurrencyFormatterPreferences = locale!("zh-CN").into();
        let currency_code = CurrencyCode(tinystr!(3, "CNY"));
        let fmt = CurrencyFormatter::<Compact>::try_new_short(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        assert_writeable_eq!(formatted_currency, "¥1.2万");

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        assert_writeable_eq!(formatted_currency, "-¥1.2万");
    }

    #[test]
    pub fn test_ar_eg() {
        let prefs: CurrencyFormatterPreferences = locale!("ar-EG").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let fmt = CurrencyFormatter::<Compact>::try_new_short(prefs, &currency_code).unwrap();

        // Positive case
        let positive_value = "12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&positive_value);
        // TODO(#6064)
        assert_writeable_eq!(formatted_currency, "\u{200f}١٢\u{a0}ألف\u{a0}ج.م.\u{200f}"); //  "ج.م.١٢ألف"

        // Negative case
        let negative_value = "-12345.67".parse().unwrap();
        let formatted_currency = fmt.format_fixed_decimal(&negative_value);
        // TODO(#6064)
        assert_writeable_eq!(
            formatted_currency,
            "\u{61c}-\u{200f}١٢\u{a0}ألف\u{a0}ج.م.\u{200f}"
        );
    }

    #[test]
    pub fn test_alpha_next_to_number_and_small_numbers() {
        let prefs: CurrencyFormatterPreferences = locale!("en-US").into();
        let usd = CurrencyCode(tinystr!(3, "USD"));
        let sek = CurrencyCode(tinystr!(3, "SEK"));

        let fmt_usd = CurrencyFormatter::<Compact>::try_new_short(prefs, &usd).unwrap();
        let fmt_sek = CurrencyFormatter::<Compact>::try_new_short(prefs, &sek).unwrap();

        // Small number (magnitude < 3, no compact suffix): should fall back cleanly
        let small_value = "123".parse().unwrap();
        assert_writeable_eq!(fmt_usd.format_fixed_decimal(&small_value), "$123");
        assert_writeable_eq!(fmt_sek.format_fixed_decimal(&small_value), "SEK\u{a0}123");

        // Compact number with alphabetical currency code: should use alpha_next_to_number pattern with non-breaking space
        let compact_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(fmt_sek.format_fixed_decimal(&compact_value), "SEK\u{a0}12K");
    }
}
