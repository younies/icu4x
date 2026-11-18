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
| Unit quantity | What a unit measures, from its base unit; often narrower than the category, and **not all the units has quantities** | `voltage` and `electric-current`; both are in the `electric` category | "Quantity" |

A pre-built unit can be simple (`meter`) or non-simple (`kilogram`). A composed unit is never [this is not true I think?]
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
    - Width (`unitDisplay` in ECMAScript; CLDR calls it the length): long, short, or narrow.
        - Note: each category (to be filled).
    - Rounding and digits: integer, fraction, and significant digits, `roundingMode`, `roundingIncrement`, `roundingPriority`, and `trailingZeroDisplay`.
    - Notation (`notation`): standard, scientific, engineering, or compact.
    - Sign display (`signDisplay`).
    - Grouping (`useGrouping`).
    - Numbering system (`numberingSystem`).
    - Output: a string, formatted parts, and ranges.

## Pre-built Units (example: `meter` and `kilogram`)

### What we need to format a pre-built unit

1. The unit's data entry from the CLDR data, found by:
    - the unit ID,
    - the locale,
    - the width (`long`, `short`, or `narrow`).
2. The locale's plural rules, to pick the pattern for the number from this data entry. Note: the plural rules apply to all the widths, `narrow`, `short`, and `long`.


### Current implementation

The units code is in `icu_experimental`. It formats pre-built units in only five categories, which are written by hand: area, duration, length, mass, and volume.

#### Current implementation of the API (categorized units)

##### 📌 Core Architecture

*   **Definition:** `CategorizedFormatter<C>` formats units for a specific category `C` (like `Length`, `speed`), which determines the underlying data used.
*   **Settings:**
    *   **Inputs:** Takes preferences (locale + numbering system), a unit, and options.
    *   **Options:** Only configures width—`short` (default), `long`, or `narrow`.

##### 🔍 Constructor Lookup Hierarchy

Each of the three constructors searches cumulative tiers of data to find the unit:

| Constructor | Data Searched (Falls back in order) |
| :--- | :--- |
| **`try_new_core`** | `Core` |
| **`try_new_extended`** | `Core` ➡️ `Extended` |
| **`try_new_outlier`** | `Core` ➡️ `Extended` ➡️ `Outlier` |

###### 💻 Code Example

```rust
let formatter = CategorizedFormatter::<Area>::try_new_core(
    locale!("de-DE").into(),
    Area::square_meter(),
    UnitsFormatterOptions::default(),
)?;

let value = Decimal::from_str("1000")?;

// Formats the value using "de-DE" rules
// Expected Output: "1.000 m²" (German uses "." as a thousands separator)
formatter.format_fixed_decimal(&value); 
```

#### Problems with the current API

##### 1. The categories are written by hand

* The categories are written by hand: the five category types and their 16 units.

##### 1. Solution

1. We need a code generator that generates the categories. 
    * The generator must also keep track of the category names, because CLDR can change or rename a category.
    * In the MeasureUnit section, we need to handle also generating the 

##### 2. Naming 
* Do we recommend the current name `CategorizedFormatter`?

##### 2. Solution
* we can name it 
    * `MeasureFormatter`
    * `UnitFormatter`
    * `PreBuiltUnitFormatter`
    * ... etc.


##### 3. Length does not has it own formatter

right now, width (like `long`, `short` and `narrow`) is passing by options, and it is better to split the data based on the width

##### 3. Solution

Therefore, I propose to have a formatter based on the width
* `CategorizedFormatterLong`
* `CategorizedFormatterShort`
* `CategorizedFormatterNarrow`




#### Current implementation of the units data

There are **15 total markers** (3 markers for each of the 5 categories). These categories classify how units are distributed across different regions:

| Category | Description | Example Marker |
| :--- | :--- | :--- |
| **`Core`** | Units actively used in the locale's region according to CLDR unit preferences. | `UnitsNamesLengthCoreV1` |
| **`Extended`** | Units not used in the local region, but used in other regions. | *N/A* |
| **`Outlier`** | Units not used in any region's standard preferences. | *N/A* |



*   **Lookup Key:** A unit's entry is retrieved using the **locale** and a composite key made of
    1. The **width** 
    2. The **CLDR unit ID**
    3. E.g., `short-meter`.
*   **Plural Forms:** The retrieved entry contains the formatting patterns, with one pattern mapped for each plural form.

#### Problems with the Current implementation of the units data

##### 1. Manual creation for each category

* Each category data marker is written manually

##### 1. solution

* We need a code generator to generate all the markers
    * NOTE: in case CLDR changed one of the categories name, the marker name will be changed, the marker name should be change.
        * This means that we need to reserve the old markers name and make them include the same data of the new category name, for example, `LengthV1` if renamded to `TallV1`, `LengthV1` and `TallV1` shall have the same data, so we keep the data markers exist and the data will be dedublicated.


##### 2. Data Representaion

1. The data is combining all the widthes together in the same marker and splitting the data will help in having a spilitted API based on the length



##### 2. Solution

* I propose to have a marker for each category based on the width,
    * `UnitsNamesLengthLongCoreV1`
    * `UnitsNamesLengthShortCoreV1`


##### 3. Data Split , core, extended and outlier

* Distinguish between Core and Extended causing some issues
    * There will be different data in `Core` based on the locale, which will not help in deduplication, especially in short and narrow data.
    * when the locale fallback, it will reach always default `001` , which in some locales , like en-US, and `meter` , core will return the  `001` fallback instead failing to make the user to use `Extended`


##### 3. Solution

* I propose to combine both the Core and Extended to be just `Core`, this will result to have only `Core` for all the units in unit prefernces and `Outliers` for the other units.

##### 4. There is no split based on simple or non-simple unit

This will come later, but we can discuss it here, as in the composed units formatting, we will need to have 

1. composition rules
2. simple units
3. per-unit data

##### 4. Solution

I propose to have a split to the units based on simplicity, for example: `UnitsNamesLengthLongCoreSimpleV1` and `UnitsNamesLengthLongCoreV1` for non simple units

Also, having the pre-unit data already in a marker, `UnitsPerunitLengthLongCoreSimpleV1` and `UnitsPerunitLengthLongCoreV1`






#### Current implementation of the measure units (`MeasureUnit`)

Measure units is a struct that actually mapping what is inside a MeasureUnit

```rust
pub struct MeasureUnit {
    single_units: Vec<SingleUnit>, // the list of single units
    constant_denominator: u64,     // 0 when there's no constant
}
```

```rust
// icu::experimental::measure::provider::single_unit
pub struct SingleUnit {
    pub power: i8,           // e.g. 2 for square-, -1 when after "per"
    pub si_prefix: SiPrefix, // SI or binary prefix
    pub unit_id: u16,        // index into hand-written table (157 units, measure/parser/ids.rs)
}
```


```rust
// Example: "liter-per-100-kilometer"
MeasureUnit {
    single_units: [
        SingleUnit { power:  1, si_prefix: { power: 0, base: Decimal }, unit_id: liter }, // liter
        SingleUnit { power: -1, si_prefix: { power: 3, base: Decimal }, unit_id: meter }, // per kilometer
    ],
    constant_denominator: 100,
}
```

**16 hand-written constructors:** 
Duration 10 · Mass 2 · Volume 2 · Length 1 · Area 1

#### Problems Current implementation of the measure units (`MeasureUnit`)

while the `MeasureUnit` struct is very efficient for representing the internal measure unit representaion in a way that we can:
* convert the unit (extract the units conversion data)
* know the internal structure like 
    * How many single units
    * Constant and so on

but for a user, who needs just to format a pre-built unit, they need just to have a the unit to format it

For example: `PreBuilt::Meter` or `PreBuiltUnit::KiloGram` ... etc.


##### Proposal for `Prebuilt MeasureUnit`

Having a struct, enum called `PrebuiltUnit` and the id of each enum is the id of the prebuilt unit

```rust=
enum PrebuiltUnit {
    METER = 0,
    SQUARE_METER = 1,
    MILE = 2,
    KILO_GRAM = 3,
}
```

These prebuilt units can be auto generated, and when the CLDR deprecate one, we keep it and we can give it and give it the number of the deprecated unit

For example:

```rust=
enum PrebuiltUnit {
    TONNE = 123, // deprecated
    METRIC_TONNE = 123 // replacing TONNE
}
```





## Composed Units

TBD

## Mixed Units
TBD


## Smart Units

TBD


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

## Appendix B: Data needed by each category in option 2

<style>
/* Heading colors by depth, for HackMD. GitHub shows this block as text. */
.markdown-body h1 { color: #000000; }
.markdown-body h2 { color: #333333; }
.markdown-body h3 { color: #1967d2; }
.markdown-body h4 { color: #188038; }
</style>
