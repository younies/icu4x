# icu_datetime [![crates.io](https://img.shields.io/crates/v/icu_datetime)](https://crates.io/crates/icu_datetime)

<!-- cargo-rdme start -->

Localized formatting of dates, times, and time zones.

This module is published as its own crate ([`icu_datetime`](https://docs.rs/icu_datetime/latest/icu_datetime/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.

ICU4X datetime formatting follows the Unicode UTS 35 standard for [Semantic Skeletons](https://unicode.org/reports/tr35/tr35-dates.html#Semantic_Skeletons).
First you choose a _field set_, then you configure the formatting _options_ to your desired context.

1. Field Sets: [`icu::datetime::fieldsets`](fieldsets)
2. Options: [`icu::datetime::options`](options)

ICU4X supports formatting in over one dozen _calendar systems_, including Gregorian, Buddhist,
Hijri, and more. The calendar system is usually derived from the locale, but it can also be
specified explicitly.

The main formatter in this crate is [`DateTimeFormatter`], which supports all field sets,
options, and calendar systems. Additional formatter types are available to developers in
resource-constrained environments.

The formatters accept input types from the [`calendar`](icu_calendar) and
[`timezone`](icu_time) crates (Also reexported from the [`input`] module of this crate):

1. [`Date`](icu_calendar::Date)
2. [`DateTime`](icu_time::DateTime)
3. [`Time`](icu_time::Time)
4. [`UtcOffset`](icu_time::zone::UtcOffset)
5. [`TimeZoneInfo`](icu_time::TimeZoneInfo)
6. [`ZonedDateTime`](icu_time::ZonedDateTime)

Not all inputs are valid for all field sets.

## Binary Size Tradeoffs

The datetime crate has been engineered with a focus on giving developers the ability to
tune binary size to their needs. The table illustrates the two main tradeoffs, field sets
and calendar systems:

| Factor | Static (Lower Binary Size) | Dynamic (Greater Binary Size) |
|---|---|---|
| Field Sets | Specific [`fieldsets`] types | Enumerations from [`fieldsets::enums`] |
| Calendar Systems | [`FixedCalendarDateTimeFormatter`] | [`DateTimeFormatter`] |

If formatting times and time zones without dates, consider using [`NoCalendarFormatter`].

## Examples

```rust
use icu::datetime::fieldsets;
use icu::datetime::input::Date;
use icu::datetime::input::{DateTime, Time};
use icu::datetime::DateTimeFormatter;
use icu::locale::{locale, Locale};
use writeable::assert_writeable_eq;

// Field set for year, month, day, hour, and minute with a medium length:
let field_set_with_options = fieldsets::YMD::medium().with_time_hm();

// Create a formatter for Argentinian Spanish:
let locale = locale!("es-AR");
let dtf = DateTimeFormatter::try_new(locale.into(), field_set_with_options)
    .unwrap();

// Format something:
let datetime = DateTime {
    date: Date::try_new_iso(2025, 1, 15).unwrap(),
    time: Time::try_new(16, 9, 35, 0).unwrap(),
};
let formatted_date = dtf.format(&datetime);

assert_writeable_eq!(formatted_date, "15 de ene de 2025, 4:09 p. m.");
```

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
