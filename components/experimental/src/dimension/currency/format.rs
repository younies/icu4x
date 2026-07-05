// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: add more tests for this module to cover more locales & currencies.
#[cfg(test)]
mod tests {
    use icu_locale_core::locale;
    use tinystr::*;
    use writeable::assert_writeable_eq;

    use crate::dimension::currency::{
        CurrencyCode,
        formatter::{CurrencyFormatter, CurrencyFormatterPreferences},
    };

    #[test]
    pub fn test_en_us() {
        let prefs = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));

        // Short
        let fmt_short = CurrencyFormatter::try_new_short(prefs, &currency_code).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&positive_value),
            "$12,345.67"
        );
        let negative_value = "-12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&negative_value),
            "-$12,345.67"
        );

        let value_no_decimals = "123".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&value_no_decimals),
            "$123.00"
        );
        let value_4_decimals = "123.4567".parse().unwrap();
        assert_writeable_eq!(fmt_short.format_fixed_decimal(&value_4_decimals), "$123.46");

        // Narrow
        let fmt_narrow = CurrencyFormatter::try_new_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&positive_value),
            "$12,345.67"
        );
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&negative_value),
            "-$12,345.67"
        );

        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&value_no_decimals),
            "$123.00"
        );
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&value_4_decimals),
            "$123.46"
        );

        // Long
        let fmt_long = CurrencyFormatter::try_new_long(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&positive_value),
            "12,345.67 US dollars"
        );
        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&negative_value),
            "-12,345.67 US dollars"
        );

        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&value_no_decimals),
            "123.00 US dollars"
        );
        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&value_4_decimals),
            "123.46 US dollars"
        );
    }

    #[test]
    pub fn test_fr_fr() {
        let prefs = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "EUR"));

        // Short
        let fmt_short = CurrencyFormatter::try_new_short(prefs, &currency_code).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&positive_value),
            "12\u{202f}345,67\u{a0}€"
        );
        let negative_value = "-12345.67".parse().unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&negative_value),
            "-12\u{202f}345,67\u{a0}€"
        );

        // Narrow
        let fmt_narrow = CurrencyFormatter::try_new_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&positive_value),
            "12\u{202f}345,67\u{a0}€"
        );
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&negative_value),
            "-12\u{202f}345,67\u{a0}€"
        );

        // Long
        let fmt_long = CurrencyFormatter::try_new_long(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&positive_value),
            "12\u{202f}345,67 euros"
        );
        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&negative_value),
            "-12\u{202f}345,67 euros"
        );
    }

    #[test]
    pub fn test_ar_eg() {
        let prefs = locale!("ar-EG").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));

        // Short
        let fmt_short = CurrencyFormatter::try_new_short(prefs, &currency_code).unwrap();
        let positive_value = "12345.67".parse().unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&positive_value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );
        let negative_value = "-12345.67".parse().unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&negative_value),
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );

        // Narrow
        let fmt_narrow = CurrencyFormatter::try_new_narrow(prefs, &currency_code).unwrap();
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&positive_value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );
        // TODO(#6064)
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&negative_value),
            "\u{61c}-\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );

        // Long
        let fmt_long = CurrencyFormatter::try_new_long(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&positive_value),
            "١٢٬٣٤٥٫٦٧ جنيه مصري"
        );
        assert_writeable_eq!(
            fmt_long.format_fixed_decimal(&negative_value),
            "\u{61c}-١٢٬٣٤٥٫٦٧ جنيه مصري"
        );
    }

    #[test]
    pub fn test_usd_in_fr_fr() {
        let prefs = locale!("fr-FR").into();
        let currency_code = CurrencyCode(tinystr!(3, "USD"));
        let value = "12345.67".parse().unwrap();

        // Short USD in fr-FR should be US$ or $US
        let fmt_short = CurrencyFormatter::try_new_short(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_short.format_fixed_decimal(&value),
            "12\u{202f}345,67\u{a0}$US"
        );

        // Narrow USD in fr-FR should be $
        let fmt_narrow = CurrencyFormatter::try_new_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_narrow.format_fixed_decimal(&value),
            "12\u{202f}345,67\u{a0}$"
        );
    }

    #[test]
    pub fn test_numbering_system_override() {
        let prefs_arab = locale!("ar-EG").into();
        let prefs_latn = locale!("ar-EG-u-nu-latn").into();
        let currency_code = CurrencyCode(tinystr!(3, "EGP"));
        let value = "12345.67".parse().unwrap();

        // 1. Default numbering system (arab) - Short
        let fmt_arab_short = CurrencyFormatter::try_new_short(prefs_arab, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_arab_short.format_fixed_decimal(&value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}ج.م.\u{200f}"
        );

        // 2. Locale extension override (latn) - Short
        let fmt_latn_short = CurrencyFormatter::try_new_short(prefs_latn, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_latn_short.format_fixed_decimal(&value),
            "\u{200f}12,345.67\u{a0}ج.م.\u{200f}"
        );

        // 3. Default numbering system (arab) - Narrow
        let fmt_arab_narrow =
            CurrencyFormatter::try_new_narrow(prefs_arab, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_arab_narrow.format_fixed_decimal(&value),
            "\u{200f}١٢٬٣٤٥٫٦٧\u{a0}E£"
        );

        // 4. Locale extension override (latn) - Narrow
        let fmt_latn_narrow =
            CurrencyFormatter::try_new_narrow(prefs_latn, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_latn_narrow.format_fixed_decimal(&value),
            "\u{200f}12,345.67\u{a0}E£"
        );

        // 5. Default numbering system (arab) - Long
        let fmt_arab_long = CurrencyFormatter::try_new_long(prefs_arab, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_arab_long.format_fixed_decimal(&value),
            "١٢٬٣٤٥٫٦٧ جنيه مصري"
        );

        // 6. Locale extension override (latn) - Long
        let fmt_latn_long = CurrencyFormatter::try_new_long(prefs_latn, &currency_code).unwrap();
        assert_writeable_eq!(
            fmt_latn_long.format_fixed_decimal(&value),
            "12,345.67 جنيه مصري"
        );
    }

    #[test]
    pub fn test_en_us_cad() {
        let prefs = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "CAD"));
        let value = "12345.67".parse().unwrap();

        // Short
        let fmt_short = CurrencyFormatter::try_new_short(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_short.format_fixed_decimal(&value), "CA$12,345.67");

        // Narrow
        let fmt_narrow = CurrencyFormatter::try_new_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_narrow.format_fixed_decimal(&value), "$12,345.67");
    }

    #[test]
    pub fn test_long_fallback_to_iso() {
        let prefs: CurrencyFormatterPreferences = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "XYZ"));
        let value = "12345.67".parse().unwrap();

        let fmt_long = CurrencyFormatter::try_new_long(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_long.format_fixed_decimal(&value), "12,345.67 XYZ");
    }

    #[test]
    pub fn test_jpy() {
        let prefs: CurrencyFormatterPreferences = locale!("en-US").into();
        let currency_code = CurrencyCode(tinystr!(3, "JPY"));
        let value = "12345.67".parse().unwrap();

        // JPY has 0 decimals (defined in global CurrencyFractionsV1)
        // Short
        let fmt_short = CurrencyFormatter::try_new_short(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_short.format_fixed_decimal(&value), "¥12,346");

        // Narrow
        let fmt_narrow = CurrencyFormatter::try_new_narrow(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_narrow.format_fixed_decimal(&value), "¥12,346");

        // Long
        let fmt_long = CurrencyFormatter::try_new_long(prefs, &currency_code).unwrap();
        assert_writeable_eq!(fmt_long.format_fixed_decimal(&value), "12,346 Japanese yen");
    }
}
