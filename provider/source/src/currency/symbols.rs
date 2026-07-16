// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::numbers::NumberPattern;
use crate::cldr_serde::numbers::NumberPatternItem;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use tinystr::TinyAsciiStr;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use icu::experimental::dimension::provider::currency::symbols::*;
use icu::experimental::dimension::provider::currency::ule::MAX_SYMBOL_INDEX;
use icu::properties::CodePointMapData;
use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
use icu_provider::DataProvider;
use icu_provider::prelude::*;

/// Returns the pattern selection for a currency.
/// For example:
///    if the pattern is ¤#,##0.00 and the symbol is EGP,
///    this means the return value will be `PatternSelection::StandardAlphaNextToNumber`
///    because the character closest to the number is a letter.
/// NOTE:
///   `symbol` must not be empty.
fn currency_pattern_selection(
    provider: &SourceDataProvider,
    pattern: &NumberPattern,
    symbol: &str,
) -> Result<PatternSelection, DataError> {
    if symbol.is_empty() {
        return Err(DataError::custom("Symbol must not be empty"));
    }

    // TODO(#6064): Handle the negative sub pattern.
    let pattern = &pattern.positive;

    let currency_sign_index = pattern
        .iter()
        .position(|i| matches!(i, NumberPatternItem::Currency))
        .unwrap();
    let first_num_index = pattern
        .iter()
        .position(|i| {
            matches!(
                i,
                NumberPatternItem::MandatoryDigit | NumberPatternItem::OptionalDigit
            )
        })
        .unwrap();
    let last_num_index = pattern
        .iter()
        .rposition(|i| {
            matches!(
                i,
                NumberPatternItem::MandatoryDigit | NumberPatternItem::OptionalDigit
            )
        })
        .unwrap();

    let letters_set = CodePointMapData::<GeneralCategory>::try_new_unstable(provider)?
        .as_borrowed()
        .get_set_for_value_group(GeneralCategoryGroup::Letter);

    let char_closer_to_number = if currency_sign_index < first_num_index {
        symbol.chars().next_back().unwrap()
    } else if currency_sign_index > last_num_index {
        symbol.chars().next().unwrap()
    } else {
        return Err(DataError::custom(
            "Currency sign must not be in the middle of the pattern",
        ));
    };

    Ok(
        if letters_set.as_borrowed().contains(char_closer_to_number) {
            PatternSelection::StandardAlphaNextToNumber
        } else {
            PatternSelection::Standard
        },
    )
}

impl DataProvider<CurrencySymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencySymbolsV1>, DataError> {
        self.check_req::<CurrencySymbolsV1>(req)?;

        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(req.id.locale, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let nsname = if !req.id.marker_attributes.is_empty() {
            req.id.marker_attributes.as_str()
        } else {
            &numbers_resource.main.value.numbers.default_numbering_system
        };

        let result = extract_currency_symbols(self, currencies_resource, numbers_resource, nsname);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

impl IterableDataProviderCached<CurrencySymbolsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers_with_locales()
    }
}

fn extract_currency_symbols<'data>(
    provider: &SourceDataProvider,
    currencies_resource: &cldr_serde::currencies::data::Resource,
    numbers_resource: &cldr_serde::numbers::Resource,
    numsys_name: &str,
) -> Result<CurrencySymbols<'data>, DataError> {
    let currencies = &currencies_resource.main.value.numbers.currencies;

    let numbers_block = &numbers_resource.main.value.numbers;
    let default_numsys = &numbers_block.default_numbering_system;
    let currency_formats = numbers_block
        .numsys_data
        .currency_patterns
        .get(numsys_name)
        .or_else(|| {
            numbers_block
                .numsys_data
                .currency_patterns
                .get(default_numsys)
        })
        .or_else(|| numbers_block.numsys_data.currency_patterns.get("latn"))
        .ok_or_else(|| DataError::custom("Could not find currency patterns"))?;

    let standard = &currency_formats.standard;

    let mut currency_patterns_map =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>::new();
    let mut currency_patterns_standard_none =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>::new();
    let mut currency_patterns_standard_next_to_num =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>::new();
    let mut symbols = Vec::<&str>::new();
    let mut symbols_checker_map = HashMap::<&str, u16>::new();

    fn intern_symbol<'a>(
        symbol: &'a str,
        iso: &str,
        symbols: &mut Vec<&'a str>,
        symbols_checker_map: &mut HashMap<&'a str, u16>,
    ) -> Result<CurrencySymbol, DataError> {
        if let Some(&index) = symbols_checker_map.get(symbol) {
            Ok(CurrencySymbol::Index(index))
        } else if symbol == iso {
            Ok(CurrencySymbol::ISO)
        } else {
            let index = symbols.len() as u16;
            if index > MAX_SYMBOL_INDEX {
                return Err(DataError::custom("symbol index exceeded MAX_SYMBOL_INDEX"));
            }
            symbols.push(symbol);
            symbols_checker_map.insert(symbol, index);
            Ok(CurrencySymbol::Index(index))
        }
    }

    for (iso, currency_pattern) in currencies {
        let short_symbol = currency_pattern
            .short
            .as_ref()
            .map(|p| intern_symbol(p.as_str(), iso, &mut symbols, &mut symbols_checker_map))
            .transpose()?;

        let narrow_symbol = currency_pattern
            .narrow
            .as_ref()
            .map(|p| intern_symbol(p.as_str(), iso, &mut symbols, &mut symbols_checker_map))
            .transpose()?;

        let short_pattern_selection = currency_pattern_selection(
            provider,
            standard,
            currency_pattern.short.as_ref().unwrap_or(iso),
        )?;
        let narrow_pattern_selection = currency_pattern_selection(
            provider,
            standard,
            currency_pattern.narrow.as_ref().unwrap_or(iso),
        )?;

        let currency_patterns = CurrencyPatternConfig {
            short_pattern_selection,
            narrow_pattern_selection,
            short_symbol,
            narrow_symbol,
        };

        let iso = TinyAsciiStr::try_from_str(iso).unwrap().to_unvalidated();
        match (short_pattern_selection, narrow_pattern_selection) {
            (PatternSelection::Standard, PatternSelection::Standard)
                if short_symbol.is_none() && narrow_symbol.is_none() =>
            {
                currency_patterns_standard_none.insert(iso, currency_patterns);
            }
            (
                PatternSelection::StandardAlphaNextToNumber,
                PatternSelection::StandardAlphaNextToNumber,
            ) if short_symbol.is_none() && narrow_symbol.is_none() => {
                currency_patterns_standard_next_to_num.insert(iso, currency_patterns);
            }
            _ => {
                currency_patterns_map.insert(iso, currency_patterns);
            }
        }
    }

    let default_pattern_config =
        if currency_patterns_standard_none.len() <= currency_patterns_standard_next_to_num.len() {
            currency_patterns_map.extend(currency_patterns_standard_none);
            CurrencyPatternConfig {
                short_pattern_selection: PatternSelection::StandardAlphaNextToNumber,
                narrow_pattern_selection: PatternSelection::StandardAlphaNextToNumber,
                short_symbol: None,
                narrow_symbol: None,
            }
        } else {
            currency_patterns_map.extend(currency_patterns_standard_next_to_num);
            CurrencyPatternConfig {
                short_pattern_selection: PatternSelection::Standard,
                narrow_pattern_selection: PatternSelection::Standard,
                short_symbol: None,
                narrow_symbol: None,
            }
        };

    Ok(CurrencySymbols {
        pattern_config_map: ZeroMap::from_iter(currency_patterns_map.iter()),
        symbols: VarZeroVec::from(&symbols),
        default_pattern_config,
    })
}

#[test]
fn test_symbols() {
    use icu::experimental::dimension::currency::CurrencyCode;
    use icu::experimental::dimension::provider::currency::symbols::Width;
    use icu::locale::langid;
    use tinystr::tinystr;

    const USD: CurrencyCode = CurrencyCode(tinystr!(3, "USD"));
    const EGP: CurrencyCode = CurrencyCode(tinystr!(3, "EGP"));
    let provider = SourceDataProvider::new_testing();

    let en: DataPayload<CurrencySymbolsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(en.get().get(Width::Short, &USD).0, "$");
    assert_eq!(en.get().get(Width::Narrow, &USD).0, "$");

    // TODO(#6064)
    assert_eq!(en.get().get(Width::Short, &EGP).0, "EGP");
    assert_eq!(en.get().get(Width::Narrow, &EGP).0, "E£");

    let ar_eg: DataPayload<CurrencySymbolsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ar-EG").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(ar_eg.get().get(Width::Short, &EGP).0, "ج.م.\u{200f}");
    assert_eq!(ar_eg.get().get(Width::Narrow, &EGP).0, "E£");

    assert_eq!(ar_eg.get().get(Width::Short, &USD).0, "US$");
    assert_eq!(ar_eg.get().get(Width::Narrow, &USD).0, "US$");
}
