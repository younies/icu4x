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
| Unit quantity | What a unit measures, from its base unit; often narrower than the category, and not every unit has one: `generic` and some composed units don't | `voltage` and `electric-current`; both are in the `electric` category | "Quantity" |

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

##### Core architecture

*   **Definition:** `CategorizedFormatter<C>` formats units for a specific category `C` (like `Length` or `Mass`), which determines the underlying data used.
*   **Settings:**
    *   **Inputs:** Takes preferences (locale + numbering system), a unit, and options.
    *   **Options:** only the width: `short` (the default), `long`, or `narrow`.

##### Constructor lookup order

Each of the three constructors searches cumulative tiers of data to find the unit:

| Constructor | Data searched, in order |
| :--- | :--- |
| **`try_new_core`** | `Core` |
| **`try_new_extended`** | `Core`, then `Extended` |
| **`try_new_outlier`** | `Core`, then `Extended`, then `Outlier` |

**Code example**

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

##### Problem 1: The categories are written by hand

The five category types and their 16 units are written by hand.

**Solution.** We need a code generator that generates the categories.

* The generator must also keep track of the category names, because CLDR can change or rename a category.
* The `MeasureUnit` section below covers generating the units of each category.

##### Problem 2: The name of the formatter

Do we recommend the current name `CategorizedFormatter`?

**Solution.** We can name it:

* `MeasureFormatter`
* `UnitFormatter`
* `PrebuiltUnitFormatter`
* ... etc.

##### Problem 3: The width is an option, not part of the formatter

Today the width (`long`, `short`, or `narrow`) is passed in the options. So every formatter ships the data of all three widths.

**Solution.** Split the formatter by width. There are two ways to do it, and both let an app ship one width:

| Choice | Example | Cost |
| :--- | :--- | :--- |
| Three formatter types | `CategorizedFormatterLong`, `CategorizedFormatterShort`, `CategorizedFormatterNarrow` | Three types to name, document, and keep the same |
| Constructors per width on one type | `CategorizedFormatter::try_new_long_core` | More constructors on one type |

The currency formatter already uses the second shape (`try_new_symbol`, `try_new_name`, `try_new_code`), so I propose that one.

#### Current implementation of the units data

There are **15 markers**: 3 for each of the 5 categories. The three tiers say where CLDR's unit preferences use a unit:

| Tier | Which units | Example marker |
| :--- | :--- | :--- |
| **`Core`** | Units that the preferences use in the locale's region | `UnitsNamesLengthCoreV1` |
| **`Extended`** | Units that the preferences use only in other regions | `UnitsNamesLengthExtendedV1` |
| **`Outlier`** | Units that the preferences don't use | `UnitsNamesLengthOutlierV1` |

*   **Lookup Key:** A unit's entry is retrieved using the **locale** and a composite key made of
    1. The **width** 
    2. The **CLDR unit ID**
    3. E.g., `short-meter`.
*   **Plural Forms:** The retrieved entry contains the formatting patterns, with one pattern mapped for each plural form.

#### Problems with the current implementation of the units data

##### Problem 1: Each category's markers are written by hand

Each category data marker is written manually.

**Solution.** We need a code generator to generate all the markers.

If CLDR renames a category, the marker names stay. The generator keeps a table from the CLDR category name to the Rust name. If we want the new name too, it is a type alias of the old marker, so there is one copy of the data. For example, if CLDR renamed `length` to `tall`:

```rust
pub type UnitsNamesTallCoreV1 = UnitsNamesLengthCoreV1;
```

Two markers with the same data would not work here: the baked data removes duplicates inside one marker, not across two markers, so the data would be stored twice.

ICU4X already keeps the old name of a renamed type as an alias. For example, in `components/calendar/src/cal/mod.rs`:

```rust
#[deprecated(since = "2.1.0", note = "use `KoreanTraditional`")]
pub type Dangi = KoreanTraditional;
```

*Note: the type alias is a suggestion from an AI assistant, and it needs a review. ICU4X uses aliases for renamed types, but we found no renamed data marker that is kept this way.*

##### Problem 2: Data representation

One marker holds all the widths together. Splitting the data by width will help in having an API that is split based on the width.

**Solution.** I propose to have a marker for each category based on the width:

* `UnitsNamesLengthLongCoreV1`
* `UnitsNamesLengthShortCoreV1`
* `UnitsNamesLengthNarrowCoreV1`

##### Problem 3: The split into `Core`, `Extended`, and `Outlier`

Distinguishing between `Core` and `Extended` causes some issues:

* There will be different data in `Core` based on the locale, which will not help in deduplication, especially in the short and narrow data.
* With locale fallback, a request for `meter` in `Core` for `en-US` goes from `en-US` to `en` to `und`. `und` has a `Core` entry for `meter`, so the user gets the root name, and not an error that tells them to use `Extended`.

**Solution.** I propose to combine `Core` and `Extended` into just `Core`. Then there is only `Core` for all the units in the unit preferences, and `Outlier` for the other units.

##### Problem 4: There is no split based on simple or non-simple units

This will come later, but we can discuss it here. To format composed units, we will need:

1. composition rules
2. simple units
3. per-unit data

**Solution.** I propose to split the units based on simplicity, for example `UnitsNamesLengthLongCoreSimpleV1` for simple units and `UnitsNamesLengthLongCoreNonSimpleV1` for non-simple units.

For the per-unit data, I propose 3 markers in total, one per width, keyed by unit ID: `UnitsPerUnitLongV1`, `UnitsPerUnitShortV1`, and `UnitsPerUnitNarrowV1`. The same split as the names (22 categories, 3 widths, 2 tiers, and simple or non-simple) would give 264 per-unit markers, but in CLDR 49 only 28 units in 5 categories have a per-unit pattern.

#### Current implementation of the measure units (`MeasureUnit`)

`MeasureUnit` is a struct that holds the parts of a unit:

```rust
pub struct MeasureUnit {
    id: Option<&'static str>,    // the CLDR ID; only the category constructors set it
    single_units: SingleUnitVec, // the single units; no allocation for one or two units
    constant_denominator: u64,   // 0 when there is no constant
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
    id: None,
    single_units: [
        SingleUnit { power:  1, si_prefix: { power: 0, base: Decimal }, unit_id: liter }, // liter
        SingleUnit { power: -1, si_prefix: { power: 3, base: Decimal }, unit_id: meter }, // per kilometer
    ],
    constant_denominator: 100,
}
```

**16 hand-written constructors:** 
Duration 10 · Mass 2 · Volume 2 · Length 1 · Area 1

#### Problems with the current implementation of the measure units (`MeasureUnit`)

The `MeasureUnit` struct is very efficient for the internal representation of a measure unit, in a way that we can:
* convert the unit (extract the units conversion data)
* know the internal structure like 
    * How many single units
    * Constant and so on

But a user who needs just to format a pre-built unit needs just to have the unit to format it.

For example: `Length::METER` or `Mass::KILOGRAM`.

##### Proposal for `PrebuiltUnit`

Having a struct called `PrebuiltUnit` that holds the ID of the pre-built unit. The category is a type parameter, and the units are constants on their category:

```rust
pub struct PrebuiltUnit<C: MeasureUnitCategory>(u16, PhantomData<C>); // the unit ID

impl Length {
    pub const METER: PrebuiltUnit<Length> = PrebuiltUnit(85, PhantomData);
}

impl Mass {
    pub const KILOGRAM: PrebuiltUnit<Mass> = PrebuiltUnit(75, PhantomData);
}
```

* The numbers are the unit IDs that the parser table has today.
* `PhantomData` is there because Rust doesn't allow a type parameter that no field uses.
* The category in the type is what lets a formatter ship the data of one category only.
* It is a struct with constants and not an enum, because the units are 238 and grow with each CLDR release. ICU4X's guide (`documents/design/enums_or_ids.md`) asks whether the set is fixed and whether it has 30 or fewer items. Both answers are no here, and that points to an ID.

These pre-built units can be auto-generated. When CLDR deprecates a unit, we keep the old name, and the new name gets the number of the deprecated unit.

For example, CLDR deprecated `metric-ton` and replaced it with `tonne`:

```rust
impl Mass {
    pub const TONNE: PrebuiltUnit<Mass> = PrebuiltUnit(146, PhantomData);

    #[deprecated(note = "CLDR renamed metric-ton to tonne")]
    pub const METRIC_TON: PrebuiltUnit<Mass> = Mass::TONNE;
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
.markdown-body h5 { color: #b06000; }
</style>
