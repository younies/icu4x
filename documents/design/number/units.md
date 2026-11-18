
// we need to add style to the page so we can have colors to the section based on the depth, h1 dark black, h2 black, h3 blue, h4 green

# Units in ICU4X: pre-built units, composed units, and their data

| | |
| :--- | :--- |
| **Author** | Younies Mahmoud ([@younies](https://github.com/younies)) |
| **Status** | Draft for ICU4X team review |
| **Issue** | [#8678](https://github.com/unicode-org/icu4x/issues/8678) |
| **Epic** | [#8157](https://github.com/unicode-org/icu4x/issues/8157) (Units Formatter Graduation) |
| **Related** | [#6745](https://github.com/unicode-org/icu4x/issues/6745), [#8125](https://github.com/unicode-org/icu4x/issues/8125), [#8269](https://github.com/unicode-org/icu4x/issues/8269), [#2197](https://github.com/unicode-org/icu4x/issues/2197) |
| **Updated** | 2026-11-18 |

## Terminology

| Term in this doc | Meaning | Examples | Name in UTS #35 |
| :--- | :--- | :--- | :--- |
| Pre-built unit | A unit that has its own CLDR entry to format it | `meter`, `kilogram` | A unit with an "explicit translation"; a "precomposed" unit |
| Composed unit | A unit with no CLDR entry; formatted from other units' data and combining rules | `square-foot-per-hour` | A unit with a "generated translation" |
| Simple unit | One unit without a prefix or a power | `pound`, `gram`; not `kilogram` | "Simple unit" |
| Non-simple unit | A unit with a prefix or a power, or made of several units | `kilogram`, `square-meter`, `mile-per-hour` | "Complex unit", or "compound unit" |

A pre-built unit can be simple (`meter`) or non-simple (`kilogram`). A composed unit is never
simple, because CLDR 49 has an entry for every simple unit.

UTS #35 names unit identifiers by their structure
([Unit Identifiers](https://www.unicode.org/reports/tr35/tr35-general.html#Unit_Identifiers)). Each
level includes the levels above it. The tracker's `complexity` field uses the first five names.

| Name in UTS #35 | What it adds | Examples |
| :--- | :--- | :--- |
| Simple unit ID | Nothing | `meter`, `foot`, `pound-force` |
| Prefixed unit ID | A prefix | `kilometer`, `centigram` |
| Single unit ID | A power | `square-foot`, `cubic-centimeter` |
| Core unit ID | Products and `-per-` | `kilometer-per-hour`, `kilogram-meter` |
| Mixed unit ID | `-and-` between units of one kind | `foot-and-inch` |
| Long unit ID | The category in front; to be deprecated | `length-meter` |

ICU calls a pre-built unit a built-in unit, and has three levels: single, compound, and mixed.

## Requirements

1. We need to support and format 
    - pre-built units
    - composed units (using `per-unit` data entry)
    - mixed units
    - locale aware units conversion

2. we need to support all the options needed for EcmaScript for each formatting are
    - Length
        - long, short or narrow
        - NOTE: each category 
    - rounding
    - ...etc.

## Pre-built Units (example: `meter` and `kilogram`)

### what do we need to format pre-built units

1. data entry from the CLDR data based on:
    - unit id
    - locale
3. applying the prular rule for this data entry (NOTE: the plural rule is applying for all the lengthes `narrow` , `short` and `long`).


### Current implementation

The units code is in `icu_experimental`. It formats pre-built units in only five categories, which are written by hand: area, duration, length, mass, and volume.

#### Current implementation of the API (categorized units)

- The formatter is `CategorizedFormatter<C>`, where `C` is a category type such as `Length`. The category in the type decides which data the code refers to.
- It has three constructors: `try_new_core`, `try_new_extended`, and `try_new_outlier`. Each takes the preferences (the locale and the numbering system), a unit of that category, and the options.
- The only option is the width: long, short (the default), or narrow.
- The formatter keeps the unit's patterns, a decimal formatter, and the plural rules. `format_fixed_decimal` picks the pattern with the plural rules and puts the formatted number in it.
- Each constructor looks in more data than the one before it: `try_new_core` looks for the unit in the `Core` data, `try_new_extended` in `Core` and `Extended`, and `try_new_outlier` in `Core`, `Extended`, and `Outlier`.

```rust
let formatter = CategorizedFormatter::<Area>::try_new_core(
    locale!("en-US").into(),
    Area::square_meter(),
    UnitsFormatterOptions::default(),
)?;
let value = Decimal::from_str("1000")?;
formatter.format_fixed_decimal(&value); // "1,000 m²"
```

#### Current implementation of the measure units (`MeasureUnit`)

- A `MeasureUnit` is a list of single units and a constant for the denominator, such as the `100` in `liter-per-100-kilometer`.
- A single unit is a power, a prefix (SI or binary), and a unit ID. The unit ID is a `u16` from a table that is written by hand in the parser (`measure/parser/ids.rs`). The table has 157 units.
- `MeasureUnit::try_from_str` parses any identifier, such as `meter-per-second`, and needs no data. But a parsed unit has no CLDR ID, which the formatter needs to find the unit's patterns, so it can't be formatted.
- The formatter takes a `CategorizedMeasureUnit<C>`, which only comes from a constructor on the category type, such as `Length::meter()`. There are 16 constructors, written by hand: 10 for duration, 2 for mass, 2 for volume, 1 for length, and 1 for area.
- Each constructor reads its unit ID from baked data, so it needs the `compiled_data` feature.

#### Current implementation of the units data

- **Names.** There are 15 markers, three for each of the five categories: `Core`, `Extended`, and `Outlier`, as in `UnitsNamesLengthCoreV1`.
- **Which marker.** It depends on the locale's region. A unit is `Core` if CLDR's unit preferences use it in that region, `Extended` if they use it only in other regions, and `Outlier` if they don't use it.
- **Key.** A unit's entry is found by the locale and a key made of the width and the CLDR unit ID, such as `short-meter`. The entry is the unit's patterns, one for each plural form.
- **Unit IDs.** `UnitsIdV1` holds the unit ID of each unit in the parser table. The constructors read it.
- **Composition patterns.** `UnitsEssentialsV1` holds the patterns for prefixes, powers, `per`, and `times`. Nothing reads it yet.
- **Conversion.** `UnitsInfoV1` holds the conversion data. Formatting doesn't use it.



### propoal for the pre-built units

## Proposal

## Goals and non-goals

## ICU4X today

## Design

## Alternatives considered

## Testing

## Open questions

## Decisions needed

## Design questions

## Questions for the committee

## Appendix A: How many users need composed units

## Appendix B: Data needed by each category in option 2
