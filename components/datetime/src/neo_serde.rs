// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde definitions for semantic skeleta

use crate::{
    fieldsets::{self, enums::*, Combo},
    options::*,
    raw::neo::RawOptions,
};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Bring `Day`, `Hour`, ... into scope in this file. They are used in multiple places
use FieldSetField::*;

/// 🚧 \[Experimental\] An error when resolving a [`CompositeFieldSet`]
/// from a [`CompositeFieldSetSerde`].
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/5825">#5825</a>
/// </div>
#[derive(Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum CompositeFieldSetSerdeError {
    /// The deserialized field set contains no fields.
    #[displaydoc("at least one field is required")]
    NoFields,
    /// The fields in the deserialized field set are invalid together.
    #[displaydoc("the given combination of fields does not create a valid semantic skeleton")]
    InvalidFields,
}

impl core::error::Error for CompositeFieldSetSerdeError {}

/// 🚧 \[Experimental\] A type corresponding to [`CompositeFieldSet`] that implements
/// [`serde::Serialize`] and [`serde::Deserialize`].
///
/// The serialized representation is subject to change.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/5825">#5825</a>
/// </div>
///
/// # Examples
///
/// ```
/// use icu::datetime::fieldsets;
/// use icu::datetime::fieldsets::enums::CompositeFieldSet;
/// use icu::datetime::fieldsets::enums::DateFieldSet;
/// use icu::datetime::fieldsets::serde::CompositeFieldSetSerde;
///
/// let field_set =
///     CompositeFieldSet::Date(DateFieldSet::YMD(fieldsets::YMD::short()));
/// let serde_input = CompositeFieldSetSerde::from(field_set);
///
/// let json_string = serde_json::to_string(&serde_input).unwrap();
/// assert_eq!(
///     json_string,
///     r#"{"fieldSet":["year","month","day"],"length":"short"}"#
/// );
///
/// let serde_output =
///     serde_json::from_str::<CompositeFieldSetSerde>(&json_string).unwrap();
/// let deserialized = CompositeFieldSet::try_from(serde_output).unwrap();
///
/// assert_eq!(field_set, deserialized);
/// ```
///
/// If the field set is invalid, an error will occur:
///
/// ```
/// use icu::datetime::fieldsets::enums::CompositeFieldSet;
/// use icu::datetime::fieldsets::serde::CompositeFieldSetSerde;
/// use icu::datetime::fieldsets::serde::CompositeFieldSetSerdeError;
///
/// let json_string = r#"{"fieldSet":["year","time"],"length":"short"}"#;
/// let serde_output =
///     serde_json::from_str::<CompositeFieldSetSerde>(&json_string).unwrap();
///
/// assert!(matches!(
///     CompositeFieldSet::try_from(serde_output),
///     Err(CompositeFieldSetSerdeError::InvalidFields)
/// ));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompositeFieldSetSerde {
    #[serde(rename = "fieldSet")]
    pub(crate) field_set: FieldSetSerde,
    pub(crate) length: Length,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) alignment: Option<Alignment>,
    #[serde(rename = "yearStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) year_style: Option<YearStyle>,
    #[serde(rename = "timePrecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) time_precision: Option<TimePrecision>,
}

impl From<CompositeFieldSet> for CompositeFieldSetSerde {
    fn from(value: CompositeFieldSet) -> Self {
        let (serde_field, options) = match value {
            CompositeFieldSet::Date(v) => FieldSetSerde::from_date_field_set(v),
            CompositeFieldSet::CalendarPeriod(v) => {
                FieldSetSerde::from_calendar_period_field_set(v)
            }
            CompositeFieldSet::Time(v) => FieldSetSerde::from_time_field_set(v),
            CompositeFieldSet::Zone(v) => FieldSetSerde::from_zone_field_set(v, true),
            CompositeFieldSet::DateTime(v) => {
                let (date_serde, date_options) =
                    FieldSetSerde::from_date_field_set(v.to_date_field_set());
                let (time_serde, time_options) =
                    FieldSetSerde::from_time_field_set(v.to_time_field_set());
                (
                    date_serde.extend(time_serde),
                    date_options.merge(time_options),
                )
            }
            CompositeFieldSet::DateZone(v) => {
                let (date_serde, date_options) = FieldSetSerde::from_date_field_set(v.dt());
                let (zone_serde, _ignored_options) =
                    FieldSetSerde::from_zone_field_set(v.z(), false);
                (date_serde.extend(zone_serde), date_options)
            }
            CompositeFieldSet::TimeZone(v) => {
                let (time_serde, time_options) = FieldSetSerde::from_time_field_set(v.dt());
                let (zone_serde, _ignored_options) =
                    FieldSetSerde::from_zone_field_set(v.z(), false);
                (time_serde.extend(zone_serde), time_options)
            }
            CompositeFieldSet::DateTimeZone(v) => {
                let (date_serde, date_options) =
                    FieldSetSerde::from_date_field_set(v.dt().to_date_field_set());
                let (time_serde, time_options) =
                    FieldSetSerde::from_time_field_set(v.dt().to_time_field_set());
                let (zone_serde, _ignored_options) =
                    FieldSetSerde::from_zone_field_set(v.z(), false);
                (
                    date_serde.extend(time_serde).extend(zone_serde),
                    date_options.merge(time_options),
                )
            }
        };
        Self {
            field_set: serde_field,
            length: options.length,
            alignment: options.alignment,
            year_style: options.year_style,
            time_precision: options.time_precision,
        }
    }
}

impl TryFrom<CompositeFieldSetSerde> for CompositeFieldSet {
    type Error = CompositeFieldSetSerdeError;
    fn try_from(value: CompositeFieldSetSerde) -> Result<Self, Self::Error> {
        let date = value.field_set.date_only();
        let time = value.field_set.time_only();
        let zone = value.field_set.zone_only();
        let options = RawOptions {
            length: value.length,
            alignment: value.alignment,
            year_style: value.year_style,
            time_precision: value.time_precision,
        };
        match (!date.is_empty(), !time.is_empty(), !zone.is_empty()) {
            (true, false, false) => date
                .to_date_field_set(options)
                .map(CompositeFieldSet::Date)
                .or_else(|| {
                    date.to_calendar_period_field_set(options)
                        .map(CompositeFieldSet::CalendarPeriod)
                })
                .ok_or(Self::Error::InvalidFields),
            (false, true, false) => time
                .to_time_field_set(options)
                .map(CompositeFieldSet::Time)
                .ok_or(Self::Error::InvalidFields),
            (false, false, true) => zone
                .to_zone_field_set(options, true)
                .map(CompositeFieldSet::Zone)
                .ok_or(Self::Error::InvalidFields),
            (true, true, false) => date
                .to_date_field_set(options)
                .map(|date_field_set| {
                    CompositeFieldSet::DateTime(
                        DateAndTimeFieldSet::from_date_field_set_with_raw_options(
                            date_field_set,
                            options,
                        ),
                    )
                })
                .ok_or(Self::Error::InvalidFields),
            (true, false, true) => date
                .to_date_field_set(options)
                .and_then(|date_field_set| {
                    zone.to_zone_field_set(options, false)
                        .map(|zone_field_set| {
                            CompositeFieldSet::DateZone(Combo::new(date_field_set, zone_field_set))
                        })
                })
                .ok_or(Self::Error::InvalidFields),
            (false, true, true) => time
                .to_time_field_set(options)
                .and_then(|time_field_set| {
                    zone.to_zone_field_set(options, false)
                        .map(|zone_field_set| {
                            CompositeFieldSet::TimeZone(Combo::new(time_field_set, zone_field_set))
                        })
                })
                .ok_or(Self::Error::InvalidFields),
            (true, true, true) => date
                .to_date_field_set(options)
                .and_then(|date_field_set| {
                    zone.to_zone_field_set(options, false)
                        .map(|zone_field_set| {
                            CompositeFieldSet::DateTimeZone(Combo::new(
                                DateAndTimeFieldSet::from_date_field_set_with_raw_options(
                                    date_field_set,
                                    options,
                                ),
                                zone_field_set,
                            ))
                        })
                })
                .ok_or(Self::Error::InvalidFields),
            (false, false, false) => Err(Self::Error::NoFields),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum TimePrecisionSerde {
    Hour,
    Minute,
    Second,
    SecondF1,
    SecondF2,
    SecondF3,
    SecondF4,
    SecondF5,
    SecondF6,
    SecondF7,
    SecondF8,
    SecondF9,
    MinuteOptional,
}

impl From<TimePrecision> for TimePrecisionSerde {
    fn from(value: TimePrecision) -> Self {
        match value {
            TimePrecision::Hour => TimePrecisionSerde::Hour,
            TimePrecision::Minute => TimePrecisionSerde::Minute,
            TimePrecision::Second => TimePrecisionSerde::Second,
            TimePrecision::FractionalSecond(FractionalSecondDigits::F1) => {
                TimePrecisionSerde::SecondF1
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F2) => {
                TimePrecisionSerde::SecondF2
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F3) => {
                TimePrecisionSerde::SecondF3
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F4) => {
                TimePrecisionSerde::SecondF4
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F5) => {
                TimePrecisionSerde::SecondF5
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F6) => {
                TimePrecisionSerde::SecondF6
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F7) => {
                TimePrecisionSerde::SecondF7
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F8) => {
                TimePrecisionSerde::SecondF8
            }
            TimePrecision::FractionalSecond(FractionalSecondDigits::F9) => {
                TimePrecisionSerde::SecondF9
            }
            TimePrecision::MinuteOptional => TimePrecisionSerde::MinuteOptional,
        }
    }
}

impl From<TimePrecisionSerde> for TimePrecision {
    fn from(value: TimePrecisionSerde) -> Self {
        match value {
            TimePrecisionSerde::Hour => TimePrecision::Hour,
            TimePrecisionSerde::Minute => TimePrecision::Minute,
            TimePrecisionSerde::Second => TimePrecision::Second,
            TimePrecisionSerde::SecondF1 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F1)
            }
            TimePrecisionSerde::SecondF2 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F2)
            }
            TimePrecisionSerde::SecondF3 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F3)
            }
            TimePrecisionSerde::SecondF4 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F4)
            }
            TimePrecisionSerde::SecondF5 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F5)
            }
            TimePrecisionSerde::SecondF6 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F6)
            }
            TimePrecisionSerde::SecondF7 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F7)
            }
            TimePrecisionSerde::SecondF8 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F8)
            }
            TimePrecisionSerde::SecondF9 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F9)
            }
            TimePrecisionSerde::MinuteOptional => TimePrecision::MinuteOptional,
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum FieldSetField {
    // Day and Date Fields
    Year = 1,
    Month = 2,
    Day = 3,
    Weekday = 4,
    WeekOfYear = 5,
    WeekOfMonth = 6,
    // Time Fields
    Time = 16,
    // Zone Fields
    ZoneSpecific = 32, // defaults to short if not standalone
    // ZoneSpecificShort = 33,
    ZoneSpecificLong = 34,
    ZoneOffset = 35, // defaults to short if not standalone
    // ZoneOffsetShort = 36,
    ZoneOffsetLong = 37,
    ZoneGeneric = 38, // defaults to short if not standalone
    // ZoneGenericShort = 39,
    ZoneGenericLong = 40,
    ZoneLocation = 41,
    ZoneExemplar = 42,
}

impl FieldSetField {
    const VALUES: &'static [FieldSetField] = &[
        Year,
        Month,
        Day,
        Weekday,
        Time,
        WeekOfYear,
        WeekOfMonth,
        ZoneSpecific,
        // ZoneSpecificShort,
        ZoneSpecificLong,
        ZoneOffset,
        // ZoneOffsetShort,
        ZoneOffsetLong,
        ZoneGeneric,
        // ZoneGenericShort,
        ZoneGenericLong,
        ZoneLocation,
    ];
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct FieldSetHumanReadableSerde {
    fields: Vec<FieldSetField>,
}

impl From<FieldSetSerde> for FieldSetHumanReadableSerde {
    fn from(value: FieldSetSerde) -> Self {
        let mut fields = Vec::with_capacity(value.bit_fields.count_ones() as usize);
        for i in 0..(8 * core::mem::size_of::<u64>()) {
            if (value.bit_fields & (1 << i)) != 0 {
                // Note: This could be made more efficient, but for now it is only used in
                // human-readable deserialization which is not a hot path
                let Some(field) = FieldSetField::VALUES
                    .iter()
                    .find(|field| i == **field as usize)
                else {
                    debug_assert!(false, "unknown field discriminant: {i}");
                    break;
                };
                fields.push(*field);
            }
        }
        Self { fields }
    }
}

impl From<FieldSetHumanReadableSerde> for FieldSetSerde {
    fn from(value: FieldSetHumanReadableSerde) -> Self {
        Self::from_fields(&value.fields)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct FieldSetSerde {
    pub(crate) bit_fields: u64,
}

impl FieldSetSerde {
    // Day Components
    const DAY: Self = Self::from_fields(&[Day]);
    const MONTH_DAY: Self = Self::from_fields(&[Month, Day]);
    const YEAR_MONTH_DAY: Self = Self::from_fields(&[Year, Month, Day]);
    const DAY_WEEKDAY: Self = Self::from_fields(&[Day, Weekday]);
    const MONTH_DAY_WEEKDAY: Self = Self::from_fields(&[Month, Day, Weekday]);
    const YEAR_MONTH_DAY_WEEKDAY: Self = Self::from_fields(&[Year, Month, Day, Weekday]);
    const WEEKDAY: Self = Self::from_fields(&[Weekday]);

    // Date Components
    const MONTH: Self = Self::from_fields(&[Month]);
    const YEAR_MONTH: Self = Self::from_fields(&[Year, Month]);
    const YEAR: Self = Self::from_fields(&[Year]);

    // Time Components
    const TIME: Self = Self::from_fields(&[Time]);

    // Zone Components
    const ZONE_SPECIFIC: Self = Self::from_fields(&[ZoneSpecific]);
    const ZONE_SPECIFIC_LONG: Self = Self::from_fields(&[ZoneSpecificLong]);
    const ZONE_OFFSET: Self = Self::from_fields(&[ZoneOffset]);
    const ZONE_OFFSET_LONG: Self = Self::from_fields(&[ZoneOffsetLong]);
    const ZONE_GENERIC: Self = Self::from_fields(&[ZoneGeneric]);
    const ZONE_GENERIC_LONG: Self = Self::from_fields(&[ZoneGenericLong]);
    const ZONE_LOCATION: Self = Self::from_fields(&[ZoneLocation]);
    const ZONE_EXEMPLAR: Self = Self::from_fields(&[ZoneExemplar]);

    const fn from_fields(fields: &[FieldSetField]) -> Self {
        let mut bit_fields = 0;
        let mut i = 0;
        #[allow(clippy::indexing_slicing)] // const function, guarded by loop condition
        while i < fields.len() {
            bit_fields |= 1 << (fields[i] as usize);
            i += 1;
        }
        Self { bit_fields }
    }

    const fn date_only(self) -> Self {
        Self {
            bit_fields: self.bit_fields & 0x000000000000ffff,
        }
    }

    const fn time_only(self) -> Self {
        Self {
            bit_fields: self.bit_fields & 0x00000000ffff0000,
        }
    }

    const fn zone_only(self) -> Self {
        Self {
            bit_fields: self.bit_fields & 0x0000ffff00000000,
        }
    }

    const fn is_empty(self) -> bool {
        self.bit_fields == 0
    }

    fn extend(self, other: FieldSetSerde) -> Self {
        Self {
            bit_fields: self.bit_fields | other.bit_fields,
        }
    }
}

impl From<FieldSetField> for FieldSetSerde {
    fn from(value: FieldSetField) -> Self {
        Self {
            bit_fields: 1 << (value as usize),
        }
    }
}

impl Serialize for FieldSetSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            let human_readable = FieldSetHumanReadableSerde::from(*self);
            human_readable.serialize(serializer)
        } else {
            self.bit_fields.serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for FieldSetSerde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let human_readable = FieldSetHumanReadableSerde::deserialize(deserializer)?;
            Ok(Self::from(human_readable))
        } else {
            let bit_fields = <u64>::deserialize(deserializer)?;
            Ok(Self { bit_fields })
        }
    }
}

impl FieldSetSerde {
    fn from_date_field_set(value: DateFieldSet) -> (Self, RawOptions) {
        match value {
            DateFieldSet::D(v) => (Self::DAY, v.to_raw_options()),
            DateFieldSet::MD(v) => (Self::MONTH_DAY, v.to_raw_options()),
            DateFieldSet::YMD(v) => (Self::YEAR_MONTH_DAY, v.to_raw_options()),
            DateFieldSet::DE(v) => (Self::DAY_WEEKDAY, v.to_raw_options()),
            DateFieldSet::MDE(v) => (Self::MONTH_DAY_WEEKDAY, v.to_raw_options()),
            DateFieldSet::YMDE(v) => (Self::YEAR_MONTH_DAY_WEEKDAY, v.to_raw_options()),
            DateFieldSet::E(v) => (Self::WEEKDAY, v.to_raw_options()),
        }
    }

    fn to_date_field_set(self, options: RawOptions) -> Option<DateFieldSet> {
        use DateFieldSet::*;
        match self {
            Self::DAY => Some(D(fieldsets::D::from_raw_options(options))),
            Self::MONTH_DAY => Some(MD(fieldsets::MD::from_raw_options(options))),
            Self::YEAR_MONTH_DAY => Some(YMD(fieldsets::YMD::from_raw_options(options))),
            Self::DAY_WEEKDAY => Some(DE(fieldsets::DE::from_raw_options(options))),
            Self::MONTH_DAY_WEEKDAY => Some(MDE(fieldsets::MDE::from_raw_options(options))),
            Self::YEAR_MONTH_DAY_WEEKDAY => Some(YMDE(fieldsets::YMDE::from_raw_options(options))),
            Self::WEEKDAY => Some(E(fieldsets::E::from_raw_options(options))),
            _ => None,
        }
    }

    fn from_calendar_period_field_set(value: CalendarPeriodFieldSet) -> (Self, RawOptions) {
        match value {
            CalendarPeriodFieldSet::M(v) => (Self::MONTH, v.to_raw_options()),
            CalendarPeriodFieldSet::YM(v) => (Self::YEAR_MONTH, v.to_raw_options()),
            CalendarPeriodFieldSet::Y(v) => (Self::YEAR, v.to_raw_options()),
        }
    }

    fn to_calendar_period_field_set(self, options: RawOptions) -> Option<CalendarPeriodFieldSet> {
        use CalendarPeriodFieldSet::*;
        match self {
            Self::MONTH => Some(M(fieldsets::M::from_raw_options(options))),
            Self::YEAR_MONTH => Some(YM(fieldsets::YM::from_raw_options(options))),
            Self::YEAR => Some(Y(fieldsets::Y::from_raw_options(options))),
            _ => None,
        }
    }

    fn from_time_field_set(value: TimeFieldSet) -> (Self, RawOptions) {
        match value {
            TimeFieldSet::T(v) => (Self::TIME, v.to_raw_options()),
        }
    }

    fn to_time_field_set(self, options: RawOptions) -> Option<TimeFieldSet> {
        use TimeFieldSet::*;
        match self {
            Self::TIME => Some(T(fieldsets::T::from_raw_options(options))),
            _ => None,
        }
    }

    fn from_zone_field_set(value: ZoneFieldSet, is_standalone: bool) -> (Self, RawOptions) {
        match (value, is_standalone) {
            // Standalone: return the field and length separately
            (ZoneFieldSet::Z(v), true) => (Self::ZONE_SPECIFIC, v.to_raw_options()),
            (ZoneFieldSet::Zs(v), true) => (Self::ZONE_SPECIFIC, v.to_raw_options()),
            (ZoneFieldSet::O(v), true) => (Self::ZONE_OFFSET, v.to_raw_options()),
            (ZoneFieldSet::Os(v), true) => (Self::ZONE_OFFSET, v.to_raw_options()),
            (ZoneFieldSet::V(v), true) => (Self::ZONE_GENERIC, v.to_raw_options()),
            (ZoneFieldSet::Vs(v), true) => (Self::ZONE_GENERIC, v.to_raw_options()),
            (ZoneFieldSet::L(v), true) => (Self::ZONE_LOCATION, v.to_raw_options()),
            (ZoneFieldSet::X(v), true) => (Self::ZONE_EXEMPLAR, v.to_raw_options()),
            // Non-standalone: return the short as default and long as opt-in
            (ZoneFieldSet::Z(v), false) => (Self::ZONE_SPECIFIC_LONG, v.to_raw_options()),
            (ZoneFieldSet::Zs(v), false) => (Self::ZONE_SPECIFIC, v.to_raw_options()),
            (ZoneFieldSet::O(v), false) => (Self::ZONE_OFFSET_LONG, v.to_raw_options()),
            (ZoneFieldSet::Os(v), false) => (Self::ZONE_OFFSET, v.to_raw_options()),
            (ZoneFieldSet::V(v), false) => (Self::ZONE_GENERIC_LONG, v.to_raw_options()),
            (ZoneFieldSet::Vs(v), false) => (Self::ZONE_GENERIC, v.to_raw_options()),
            (ZoneFieldSet::L(v), false) => (Self::ZONE_LOCATION, v.to_raw_options()),
            (ZoneFieldSet::X(v), false) => (Self::ZONE_EXEMPLAR, v.to_raw_options()),
        }
    }

    fn to_zone_field_set(self, options: RawOptions, is_standalone: bool) -> Option<ZoneFieldSet> {
        use ZoneFieldSet::*;
        match (self, is_standalone, options.length) {
            (Self::ZONE_SPECIFIC_LONG, _, _) => Some(Z(fieldsets::Z::new())),
            (Self::ZONE_SPECIFIC, false, _) => Some(Zs(fieldsets::Zs::new())),
            (Self::ZONE_SPECIFIC, true, Length::Long) => Some(Z(fieldsets::Z::new())),
            (Self::ZONE_SPECIFIC, true, Length::Short) => Some(Zs(fieldsets::Zs::new())),
            (Self::ZONE_OFFSET_LONG, _, _) => Some(O(fieldsets::O::new())),
            (Self::ZONE_OFFSET, false, _) => Some(Os(fieldsets::Os::new())),
            (Self::ZONE_OFFSET, true, Length::Long) => Some(O(fieldsets::O::new())),
            (Self::ZONE_OFFSET, true, Length::Short) => Some(Os(fieldsets::Os::new())),
            (Self::ZONE_GENERIC_LONG, _, _) => Some(V(fieldsets::V::new())),
            (Self::ZONE_GENERIC, false, _) => Some(Vs(fieldsets::Vs::new())),
            (Self::ZONE_GENERIC, true, Length::Long) => Some(V(fieldsets::V::new())),
            (Self::ZONE_GENERIC, true, Length::Short) => Some(Vs(fieldsets::Vs::new())),
            (Self::ZONE_LOCATION, _, _) => Some(L(fieldsets::L::new())),
            (Self::ZONE_EXEMPLAR, _, _) => Some(X(fieldsets::X::new())),
            (_, _, _) => None,
        }
    }
}

#[test]
fn test_basic() {
    let skeleton = CompositeFieldSet::DateTimeZone(Combo::new(
        DateAndTimeFieldSet::YMDET(fieldsets::YMDET {
            length: Length::Medium,
            alignment: Some(Alignment::Column),
            year_style: Some(YearStyle::WithEra),
            time_precision: Some(TimePrecision::FractionalSecond(FractionalSecondDigits::F3)),
        }),
        ZoneFieldSet::Vs(fieldsets::Vs::new()),
    ));
    let skeleton_serde = CompositeFieldSetSerde::from(skeleton);

    let json_string = serde_json::to_string(&skeleton_serde).unwrap();
    assert_eq!(
        json_string,
        r#"{"fieldSet":["year","month","day","weekday","time","zoneGeneric"],"length":"medium","alignment":"column","yearStyle":"withEra","timePrecision":"secondF3"}"#
    );
    let json_skeleton: CompositeFieldSet =
        serde_json::from_str::<CompositeFieldSetSerde>(&json_string)
            .unwrap()
            .try_into()
            .unwrap();
    assert_eq!(skeleton, json_skeleton);

    let bincode_bytes = bincode::serialize(&skeleton_serde).unwrap();
    let bincode_skeleton: CompositeFieldSet =
        bincode::deserialize::<CompositeFieldSetSerde>(&bincode_bytes)
            .unwrap()
            .try_into()
            .unwrap();
    assert_eq!(skeleton, bincode_skeleton);
}
