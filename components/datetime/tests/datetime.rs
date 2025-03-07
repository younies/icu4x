// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;
mod patterns;

use fixtures::TestOutputItem;
use icu_calendar::cal::{
    Buddhist, Chinese, Coptic, Dangi, Gregorian, Hebrew, HijriCivil, HijriObservational,
    HijriTabular, HijriUmmAlQura, Indian, Iso, Persian, Roc, {Ethiopian, EthiopianEraStyle},
    {Japanese, JapaneseExtended},
};
use icu_calendar::{
    any_calendar::{AnyCalendarKind, IntoAnyCalendar},
    Calendar,
};
use icu_datetime::scaffold::CldrCalendar;
use icu_datetime::{fieldsets::enums::*, DateTimeFormatterPreferences};
use icu_datetime::{
    pattern::DateTimePattern, pattern::FixedCalendarDateTimeNames, DateTimeFormatter,
    FixedCalendarDateTimeFormatter,
};
use icu_locale_core::{
    extensions::unicode::{key, value, Value},
    locale,
    preferences::extensions::unicode::keywords::HourCycle,
    Locale,
};
use icu_provider::prelude::*;
use icu_time::{
    zone::{IanaParser, UtcOffset},
    DateTime, TimeZoneInfo, ZonedDateTime,
};
use patterns::{
    dayperiods::{DayPeriodExpectation, DayPeriodTests},
    time_zones::TimeZoneTests,
};
use writeable::{assert_try_writeable_eq, assert_writeable_eq};

mod mock;

fn apply_preference_bag_to_locale(hour_cycle: HourCycle, locale: &mut Locale) {
    const H11: Value = value!("h11");
    const H12: Value = value!("h12");
    const H23: Value = value!("h23");
    const H24: Value = value!("h24");
    let value = match hour_cycle {
        HourCycle::H11 => H11,
        HourCycle::H12 => H12,
        HourCycle::H23 => H23,
        HourCycle::H24 => H24,
        _ => unreachable!(),
    };
    locale.extensions.unicode.keywords.set(key!("hc"), value);
}

fn test_fixture(fixture_name: &str, file: &str) {
    for fx in serde_json::from_str::<fixtures::Fixture>(file)
        .expect("Unable to get fixture.")
        .0
    {
        let skeleton = match fx.input.options.semantic {
            Some(semantic) => semantic.build_composite_datetime().unwrap(),
            None => {
                eprintln!("Warning: Skipping test with no semantic skeleton: {fx:?}");
                continue;
            }
        };
        let input_iso = DateTime::try_from_str(&fx.input.value, Iso).unwrap();

        let input_buddhist = DateTime::try_from_str(&fx.input.value, Buddhist).unwrap();
        let input_chinese = DateTime::try_from_str(&fx.input.value, Chinese::new()).unwrap();
        let input_coptic = DateTime::try_from_str(&fx.input.value, Coptic).unwrap();
        let input_dangi = DateTime::try_from_str(&fx.input.value, Dangi::new()).unwrap();
        let input_ethiopian = DateTime::try_from_str(&fx.input.value, Ethiopian::new()).unwrap();
        let input_ethioaa = DateTime::try_from_str(
            &fx.input.value,
            Ethiopian::new_with_era_style(EthiopianEraStyle::AmeteAlem),
        )
        .unwrap();
        let input_gregorian = DateTime::try_from_str(&fx.input.value, Gregorian).unwrap();
        let input_hebrew = DateTime::try_from_str(&fx.input.value, Hebrew).unwrap();
        let input_indian = DateTime::try_from_str(&fx.input.value, Indian).unwrap();
        let input_hijri_civil = DateTime::try_from_str(&fx.input.value, HijriCivil).unwrap();
        let input_hijri_observational = DateTime::try_from_str(
            &fx.input.value,
            HijriObservational::new_always_calculating(),
        )
        .unwrap();
        let input_hijri_tabular = DateTime::try_from_str(&fx.input.value, HijriTabular).unwrap();
        let input_hijri_umm_al_qura =
            DateTime::try_from_str(&fx.input.value, HijriUmmAlQura::new_always_calculating())
                .unwrap();
        let input_japanese = DateTime::try_from_str(&fx.input.value, Japanese::new()).unwrap();
        let input_japanext =
            DateTime::try_from_str(&fx.input.value, JapaneseExtended::new()).unwrap();
        let input_persian = DateTime::try_from_str(&fx.input.value, Persian).unwrap();
        let input_roc = DateTime::try_from_str(&fx.input.value, Roc).unwrap();

        let description = match fx.description {
            Some(description) => {
                format!("\n  test: {description:?}\n  file: {fixture_name}.json\n")
            }
            None => format!("\n  file: {fixture_name}.json\n"),
        };
        for (locale, output_value) in fx.output.values {
            let mut locale =
                Locale::try_from_str(&locale).expect("Expected parseable locale in fixture");
            if let Some(hour_cycle) = fx.input.options.hour_cycle {
                apply_preference_bag_to_locale(hour_cycle.into(), &mut locale);
            }
            if let Some(kind) = AnyCalendarKind::from_prefs((&locale).into()) {
                match kind {
                    AnyCalendarKind::Buddhist => assert_fixture_element(
                        &locale,
                        &input_buddhist,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Chinese => assert_fixture_element(
                        &locale,
                        &input_chinese,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Coptic => assert_fixture_element(
                        &locale,
                        &input_coptic,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Dangi => assert_fixture_element(
                        &locale,
                        &input_dangi,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Ethiopian => assert_fixture_element(
                        &locale,
                        &input_ethiopian,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::EthiopianAmeteAlem => assert_fixture_element(
                        &locale,
                        &input_ethioaa,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Hebrew => assert_fixture_element(
                        &locale,
                        &input_hebrew,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Indian => assert_fixture_element(
                        &locale,
                        &input_indian,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::HijriCivil => assert_fixture_element(
                        &locale,
                        &input_hijri_civil,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::HijriObservational => assert_fixture_element(
                        &locale,
                        &input_hijri_observational,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::HijriTabular => assert_fixture_element(
                        &locale,
                        &input_hijri_tabular,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::HijriUmmAlQura => assert_fixture_element(
                        &locale,
                        &input_hijri_umm_al_qura,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Japanese => assert_fixture_element(
                        &locale,
                        &input_japanese,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::JapaneseExtended => assert_fixture_element(
                        &locale,
                        &input_japanext,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Persian => assert_fixture_element(
                        &locale,
                        &input_persian,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    AnyCalendarKind::Roc => assert_fixture_element(
                        &locale,
                        &input_roc,
                        &input_iso,
                        &output_value,
                        skeleton,
                        &description,
                    ),
                    _ => panic!("datetime test does not support locale {locale:?}"),
                }
            } else {
                assert_fixture_element(
                    &locale,
                    &input_gregorian,
                    &input_iso,
                    &output_value,
                    skeleton,
                    &description,
                )
            }
        }
    }
}

fn assert_fixture_element<C>(
    locale: &Locale,
    input_value: &DateTime<C>,
    input_iso: &DateTime<Iso>,
    output_value: &TestOutputItem,
    skeleton: CompositeDateTimeFieldSet,
    description: &str,
) where
    C: Calendar + CldrCalendar + IntoAnyCalendar + Clone,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::YearNamesV1>,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::MonthNamesV1>,
    icu_datetime::provider::Baked: DataProvider<<C as CldrCalendar>::SkeletaV1>,
{
    assert!(
        input_value.date.calendar().any_calendar_kind().is_some(),
        "{} does not specify its AsCalendarKind",
        input_value.date.calendar().debug_name()
    );

    let input_value = ZonedDateTime {
        date: input_value.date.clone(),
        time: input_value.time,
        zone: TimeZoneInfo::utc(),
    };
    let input_iso = ZonedDateTime {
        date: input_iso.date,
        time: input_iso.time,
        zone: TimeZoneInfo::utc(),
    };

    let any_input = ZonedDateTime {
        date: input_value.date.clone().to_any(),
        time: input_value.time,
        zone: TimeZoneInfo::utc(),
    };
    let iso_any_input = ZonedDateTime {
        date: input_iso.date.to_any(),
        time: input_iso.time,
        zone: TimeZoneInfo::utc(),
    };

    let dtf = FixedCalendarDateTimeFormatter::try_new(locale.into(), skeleton).expect(description);

    let any_dtf = DateTimeFormatter::try_new(locale.into(), skeleton).expect(description);

    let actual1 = dtf.format(&input_value);
    assert_writeable_eq!(actual1, output_value.expectation(), "{}", description);

    let actual2 = any_dtf.format_same_calendar(&any_input).unwrap();
    assert_writeable_eq!(
        actual2,
        output_value.expectation(),
        "(DateTimeFormatter) {}",
        description
    );

    let actual3 = any_dtf.format(&iso_any_input);
    assert_writeable_eq!(
        actual3,
        output_value.expectation(),
        "(DateTimeFormatter iso conversion) {}",
        description
    );

    let pattern = actual1.pattern();
    assert_eq!(pattern, actual2.pattern());
    assert_eq!(pattern, actual3.pattern());

    if let Some(expected_pattern) = output_value.pattern() {
        assert_writeable_eq!(pattern, expected_pattern);
    }
}

fn test_fixture_with_time_zones(fixture_name: &str, file: &str) {
    for fx in serde_json::from_str::<fixtures::Fixture>(file)
        .expect("Unable to get fixture.")
        .0
    {
        let fset = match fx.input.options.semantic {
            Some(semantic) => semantic.build_composite().unwrap(),
            None => {
                eprintln!("Warning: Skipping test with no semantic skeleton: {fx:?}");
                continue;
            }
        };

        let zoned_datetime = mock::parse_zoned_gregorian_from_str(&fx.input.value);

        let description = match fx.description {
            Some(description) => {
                format!("\n  test: {description:?}\n  file: {fixture_name}.json\n")
            }
            None => format!("\n  file: {fixture_name}.json\n"),
        };
        for (locale, output_value) in fx.output.values {
            let mut locale: Locale = locale.parse().unwrap();
            if let Some(hour_cycle) = fx.input.options.hour_cycle {
                apply_preference_bag_to_locale(hour_cycle.into(), &mut locale);
            }
            let dtf = {
                FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(locale.into(), fset)
                    .unwrap()
            };
            assert_writeable_eq!(
                dtf.format(&zoned_datetime),
                output_value.expectation(),
                "{}",
                description,
            );
        }
    }
}

#[test]
fn test_dayperiod_patterns() {
    for test in
        serde_json::from_str::<DayPeriodTests>(include_str!("patterns/tests/dayperiods.json"))
            .unwrap()
            .0
    {
        let locale: Locale = test.locale.parse().unwrap();
        for test_case in &test.test_cases {
            for dt_input in &test_case.datetimes {
                let datetime = DateTime::try_from_str(dt_input, Gregorian).unwrap();
                for DayPeriodExpectation { patterns, expected } in &test_case.expectations {
                    for pattern_input in patterns {
                        let parsed_pattern =
                            DateTimePattern::try_from_pattern_str(pattern_input).unwrap();
                        let mut pattern_formatter = FixedCalendarDateTimeNames::<
                            Gregorian,
                            CompositeDateTimeFieldSet,
                        >::try_new(
                            (&locale).into()
                        )
                        .unwrap();
                        let formatted_datetime = pattern_formatter
                            .include_for_pattern(&parsed_pattern)
                            .unwrap()
                            .format(&datetime);
                        assert_try_writeable_eq!(
                            formatted_datetime,
                            *expected,
                            Ok(()),
                            "\n\
                            locale:   `{}`,\n\
                            datetime: `{}`,\n\
                            pattern:  `{}`",
                            locale,
                            dt_input,
                            pattern_input,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_time_zone_format_configs() {
    for test in
        serde_json::from_str::<TimeZoneTests>(include_str!("patterns/tests/time_zones.json"))
            .unwrap()
            .0
    {
        let prefs: DateTimeFormatterPreferences = test.locale.parse::<Locale>().unwrap().into();
        let zoned_datetime = mock::parse_zoned_gregorian_from_str(&test.datetime);
        for (pattern_input, expect) in &test.expectations {
            let Some(skeleton) = patterns::time_zones::pattern_to_semantic_skeleton(pattern_input)
            else {
                continue;
            };
            let tzf =
                FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(prefs, skeleton).unwrap();
            assert_writeable_eq!(
                tzf.format(&zoned_datetime.zone),
                *expect,
                "\n\
                    prefs:  `{:?}`,\n\
                    datetime: `{}`,\n\
                    config: `{:?}`,\n
                    ",
                prefs,
                test.datetime,
                pattern_input,
            );
        }
    }
}

#[test]
fn test_time_zone_format_offset_seconds() {
    use icu_datetime::fieldsets::zone::LocalizedOffsetLong;

    let tzf =
        FixedCalendarDateTimeFormatter::<(), _>::try_new(locale!("en").into(), LocalizedOffsetLong)
            .unwrap();
    assert_writeable_eq!(
        tzf.format(&UtcOffset::try_from_seconds(12).unwrap()),
        "GMT+00:00:12",
    );
}

#[test]
fn test_time_zone_format_offset_fallback() {
    use icu_datetime::fieldsets::zone::LocalizedOffsetLong;

    let tzf =
        FixedCalendarDateTimeFormatter::<(), _>::try_new(locale!("en").into(), LocalizedOffsetLong)
            .unwrap();
    assert_writeable_eq!(
        tzf.format(
            &IanaParser::new()
                .parse("America/Los_Angeles")
                .with_offset(None)
        ),
        "GMT+?",
    );
}

#[test]
fn test_time_zone_patterns() {
    for test in
        serde_json::from_str::<TimeZoneTests>(include_str!("patterns/tests/time_zones.json"))
            .unwrap()
            .0
    {
        let prefs: DateTimeFormatterPreferences = test.locale.parse::<Locale>().unwrap().into();
        let zoned_datetime = mock::parse_zoned_gregorian_from_str(&test.datetime);

        for (pattern_input, expect) in &test.expectations {
            if pattern_input == "VVV" {
                // TODO(#5658): 'VVV' format not yet supported
                continue;
            }
            let parsed_pattern = DateTimePattern::try_from_pattern_str(pattern_input).unwrap();
            let mut pattern_formatter =
                FixedCalendarDateTimeNames::<Gregorian, ZoneFieldSet>::try_new(prefs).unwrap();
            let formatted_datetime = pattern_formatter
                .include_for_pattern(&parsed_pattern)
                .unwrap()
                .format(&zoned_datetime);
            assert_writeable_eq!(
                writeable::adapters::LossyWrap(formatted_datetime),
                *expect,
                "\n\
                    prefs:  `{:?}`,\n\
                    datetime: `{}`,\n\
                    pattern:  `{}`",
                prefs,
                test.datetime,
                pattern_input,
            );
        }
    }
}

#[test]
fn test_length_fixtures() {
    test_fixture("lengths", include_str!("fixtures/tests/lengths.json"));
    test_fixture_with_time_zones(
        "lengths_with_zones",
        include_str!("fixtures/tests/lengths_with_zones.json"),
    );
    test_fixture_with_time_zones(
        "lengths_with_zones_from_pdt",
        include_str!("fixtures/tests/lengths_with_zones_from_pdt.json"),
    );
}

#[test]
fn test_japanese() {
    test_fixture("japanese", include_str!("fixtures/tests/japanese.json"));
}

#[test]
fn test_lengths_with_preferences() {
    test_fixture(
        "lengths_with_preferences",
        include_str!("fixtures/tests/lengths_with_preferences.json"),
    );
}

/// Tests simple component::Bag.
#[test]
fn test_components() {
    test_fixture("components", include_str!("fixtures/tests/components.json"));
}

/// Tests component::Bag configurations that have exact matches to CLDR skeletons.
#[test]
fn test_components_exact_matches() {
    test_fixture(
        "components-exact-matches",
        include_str!("fixtures/tests/components-exact-matches.json"),
    );
}

#[test]
fn test_components_hour_cycle() {
    test_fixture(
        "components_hour_cycle",
        include_str!("fixtures/tests/components_hour_cycle.json"),
    );
}

/// Tests that time zones are included, which rely on the append items mechanism.
#[test]
fn test_components_with_zones() {
    test_fixture_with_time_zones(
        "components_with_zones",
        include_str!("fixtures/tests/components_with_zones.json"),
    );
}

/// Tests that component::Bags can adjust for width differences in the final pattern.
#[test]
fn test_components_width_differences() {
    test_fixture(
        "components-width-differences",
        include_str!("fixtures/tests/components-width-differences.json"),
    );
}

/// Tests that combine component::Bags options that don't exactly match a pattern.
#[test]
fn test_components_partial_matches() {
    test_fixture(
        "components-partial-matches",
        include_str!("fixtures/tests/components-partial-matches.json"),
    );
}

/// Tests that component::Bags can combine a date skeleton, and a time skeleton.
#[test]
fn test_components_combine_datetime() {
    test_fixture(
        "components-combine-datetime",
        include_str!("fixtures/tests/components-combine-datetime.json"),
    );
}
