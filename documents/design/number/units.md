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
| Unit category | CLDR's group for a unit; the first part of a long unit ID such as `length-meter` | `length`, `speed`, `mass` | "unitType", or "grouping" |
| Unit quantity | What a unit measures, from its base unit; often narrower than the category | `voltage` and `electric-current`; both are in the `electric` category | "Quantity" |

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

1. We need to support and format:
    - pre-built units,
    - composed units, which also use the per-unit data entry (`perUnitPattern`),
    - mixed units,
    - units that are converted for the locale (locale-aware unit conversion).

2. We need to support every option that ECMAScript has for formatting a unit:
    - Length (`unitDisplay`): long, short, or narrow.
        - Note: each category (to be filled).
    - Rounding and digits: integer, fraction, and significant digits, `roundingMode`, `roundingIncrement`, `roundingPriority`, and `trailingZeroDisplay`.
    - Notation (`notation`): standard, scientific, engineering, or compact.
    - Sign display (`signDisplay`).
    - Grouping (`useGrouping`).
    - Numbering system (`numberingSystem`).
    - Output: a string, formatted parts, and ranges.

## Pre-built Units (example: `meter` and `kilogram`)

### what do we need to format pre-built units

1. data entry from the CLDR data based on:
    - unit id
    - locale
3. applying the prular rule for this data entry (NOTE: the plural rule is applying for all the lengthes `narrow` , `short` and `long`).


### Current Implementaion

To be filled.

#### Current Implementation for API (categorized units)
to be filled

#### Current Implemetattion for the MeasurementUnits
to be filled

#### Current impoementaiont for the Units Data

to be filled



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

<style>
/* Heading colors by depth, for HackMD. GitHub shows this block as text. */
.markdown-body h1 { color: #000000; }
.markdown-body h2 { color: #333333; }
.markdown-body h3 { color: #1967d2; }
.markdown-body h4 { color: #188038; }
</style>
