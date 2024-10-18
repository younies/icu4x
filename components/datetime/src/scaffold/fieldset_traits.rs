// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    format::neo::*,
    neo_skeleton::*,
    provider::{neo::*, time_zones::tz, *},
    scaffold::*,
};
use icu_calendar::{
    types::{
        DayOfMonth, IsoHour, IsoMinute, IsoSecond, IsoWeekday, MonthInfo, NanoSecond, YearInfo,
    },
    AnyCalendarKind, Date, Iso, Time,
};
use icu_provider::{marker::NeverMarker, prelude::*};
use icu_timezone::scaffold::IntoOption;
use icu_timezone::{TimeZoneBcp47Id, UtcOffset, ZoneVariant};

/// Trait for components that can be formatted at runtime.
pub trait IsRuntimeComponents: UnstableSealed + GetField<NeoComponents> {}

/// A trait associating [`NeoComponents`].
pub trait HasConstComponents {
    /// The associated components.
    const COMPONENTS: NeoComponents;
}

/// A trait associating [`NeoDateComponents`].
pub trait HasConstDateComponents {
    /// The associated components.
    const COMPONENTS: NeoDateComponents;
}

/// A trait associating [`NeoTimeComponents`].
pub trait HasConstTimeComponents {
    /// The associated components.
    const COMPONENTS: NeoTimeComponents;
}

/// A trait associating [`NeoTimeZoneStyle`].
pub trait HasConstZoneComponent {
    /// The associated component.
    const COMPONENT: NeoTimeZoneStyle;
}

// TODO: Add WeekCalculator and FixedDecimalFormatter optional bindings here

/// A trait associating types for date formatting in any calendar
/// (input types only).
pub trait DateInputMarkers: UnstableSealed {
    /// Marker for resolving the year input field.
    type YearInput: IntoOption<YearInfo>;
    /// Marker for resolving the month input field.
    type MonthInput: IntoOption<MonthInfo>;
    /// Marker for resolving the day-of-month input field.
    type DayOfMonthInput: IntoOption<DayOfMonth>;
    /// Marker for resolving the day-of-week input field.
    type DayOfWeekInput: IntoOption<IsoWeekday>;
    /// Marker for resolving the any-calendar-kind input field.
    type AnyCalendarKindInput: IntoOption<AnyCalendarKind>;
}

/// A trait associating types for date formatting in a specific calendar
/// (data markers only).
pub trait TypedDateDataMarkers<C>: UnstableSealed {
    /// Marker for loading date skeleton patterns.
    type DateSkeletonPatternsV1Marker: DataMarker<DataStruct = PackedPatternsV1<'static>>;
    /// Marker for loading year names.
    type YearNamesV1Marker: DataMarker<DataStruct = YearNamesV1<'static>>;
    /// Marker for loading month names.
    type MonthNamesV1Marker: DataMarker<DataStruct = MonthNamesV1<'static>>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: DataMarker<DataStruct = LinearNamesV1<'static>>;
}

/// A trait associating types for date formatting in any calendar
/// (data markers only).
pub trait DateDataMarkers: UnstableSealed {
    /// Cross-calendar data markers for date skeleta.
    type Skel: CalMarkers<ErasedPackedPatterns>;
    /// Cross-calendar data markers for year names.
    type Year: CalMarkers<YearNamesV1Marker>;
    /// Cross-calendar data markers for month names.
    type Month: CalMarkers<MonthNamesV1Marker>;
    /// Marker for loading weekday names.
    type WeekdayNamesV1Marker: DataMarker<DataStruct = LinearNamesV1<'static>>;
}

/// A trait associating types for time formatting
/// (input types and data markers).
pub trait TimeMarkers: UnstableSealed {
    /// Marker for resolving the day-of-month input field.
    type HourInput: IntoOption<IsoHour>;
    /// Marker for resolving the day-of-week input field.
    type MinuteInput: IntoOption<IsoMinute>;
    /// Marker for resolving the day-of-year input field.
    type SecondInput: IntoOption<IsoSecond>;
    /// Marker for resolving the any-calendar-kind input field.
    type NanoSecondInput: IntoOption<NanoSecond>;
    /// Marker for loading time skeleton patterns.
    type TimeSkeletonPatternsV1Marker: DataMarker<DataStruct = PackedPatternsV1<'static>>;
    /// Marker for loading day period names.
    type DayPeriodNamesV1Marker: DataMarker<DataStruct = LinearNamesV1<'static>>;
}

/// A trait associating types for time zone formatting
/// (input types and data markers).
pub trait ZoneMarkers: UnstableSealed {
    /// Marker for resolving the time zone id input field.
    type TimeZoneIdInput: IntoOption<TimeZoneBcp47Id>;
    /// Marker for resolving the time zone offset input field.
    type TimeZoneOffsetInput: IntoOption<UtcOffset>;
    /// Marker for resolving the time zone variant input field.
    type TimeZoneVariantInput: IntoOption<ZoneVariant>;
    /// Marker for resolving the time zone non-location display names, which depend on the datetime.
    type TimeZoneLocalTimeInput: IntoOption<(Date<Iso>, Time)>;
    /// Marker for loading core time zone data.
    type EssentialsV1Marker: DataMarker<DataStruct = tz::EssentialsV1<'static>>;
    /// Marker for loading location names for time zone formatting
    type LocationsV1Marker: DataMarker<DataStruct = tz::LocationsV1<'static>>;
    /// Marker for loading generic long time zone names.
    type GenericLongV1Marker: DataMarker<DataStruct = tz::MzGenericV1<'static>>;
    /// Marker for loading generic short time zone names.
    type GenericShortV1Marker: DataMarker<DataStruct = tz::MzGenericV1<'static>>;
    /// Marker for loading specific long time zone names.
    type SpecificLongV1Marker: DataMarker<DataStruct = tz::MzSpecificV1<'static>>;
    /// Marker for loading generic short time zone names.
    type SpecificShortV1Marker: DataMarker<DataStruct = tz::MzSpecificV1<'static>>;
    /// Marker for loading metazone periods.
    type MetazonePeriodV1Marker: DataMarker<DataStruct = tz::MzPeriodV1<'static>>;
}

/// A trait associating constants and types implementing various other traits
/// required for datetime formatting.
pub trait DateTimeMarkers: UnstableSealed + DateTimeNamesMarker {
    /// Associated types for date formatting.
    ///
    /// Should implement [`DateDataMarkers`], [`TypedDateDataMarkers`], and [`DateInputMarkers`].
    type D;
    /// Associated types for time formatting.
    ///
    /// Should implement [`TimeMarkers`].
    type T;
    /// Associated types for time zone formatting.
    ///
    /// Should implement [`ZoneMarkers`].
    type Z;
    /// Type of the length option in the constructor.
    type LengthOption: IntoOption<NeoSkeletonLength>;
    /// Type of the alignment option in the constructor.
    type AlignmentOption: IntoOption<Alignment>;
    /// Type of the year style option in the constructor.
    type YearStyleOption: IntoOption<YearStyle>;
    /// Type of the fractional seconds display option in the constructor.
    type FractionalSecondDigitsOption: IntoOption<FractionalSecondDigits>;
    /// Marker for loading the date/time glue pattern.
    type GluePatternV1Marker: DataMarker<DataStruct = GluePatternV1<'static>>;
}

/// Trait to consolidate input markers.
pub trait AllInputMarkers<R: DateTimeMarkers>:
    GetField<<R::D as DateInputMarkers>::YearInput>
    + GetField<<R::D as DateInputMarkers>::MonthInput>
    + GetField<<R::D as DateInputMarkers>::DayOfMonthInput>
    + GetField<<R::D as DateInputMarkers>::DayOfWeekInput>
    + GetField<<R::D as DateInputMarkers>::AnyCalendarKindInput>
    + GetField<<R::T as TimeMarkers>::HourInput>
    + GetField<<R::T as TimeMarkers>::MinuteInput>
    + GetField<<R::T as TimeMarkers>::SecondInput>
    + GetField<<R::T as TimeMarkers>::NanoSecondInput>
    + GetField<<R::Z as ZoneMarkers>::TimeZoneIdInput>
    + GetField<<R::Z as ZoneMarkers>::TimeZoneOffsetInput>
    + GetField<<R::Z as ZoneMarkers>::TimeZoneVariantInput>
    + GetField<<R::Z as ZoneMarkers>::TimeZoneLocalTimeInput>
where
    R::D: DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
{
}

impl<T, R> AllInputMarkers<R> for T
where
    R: DateTimeMarkers,
    R::D: DateInputMarkers,
    R::T: TimeMarkers,
    R::Z: ZoneMarkers,
    T: GetField<<R::D as DateInputMarkers>::YearInput>
        + GetField<<R::D as DateInputMarkers>::MonthInput>
        + GetField<<R::D as DateInputMarkers>::DayOfMonthInput>
        + GetField<<R::D as DateInputMarkers>::DayOfWeekInput>
        + GetField<<R::D as DateInputMarkers>::AnyCalendarKindInput>
        + GetField<<R::T as TimeMarkers>::HourInput>
        + GetField<<R::T as TimeMarkers>::MinuteInput>
        + GetField<<R::T as TimeMarkers>::SecondInput>
        + GetField<<R::T as TimeMarkers>::NanoSecondInput>
        + GetField<<R::Z as ZoneMarkers>::TimeZoneIdInput>
        + GetField<<R::Z as ZoneMarkers>::TimeZoneOffsetInput>
        + GetField<<R::Z as ZoneMarkers>::TimeZoneVariantInput>
        + GetField<<R::Z as ZoneMarkers>::TimeZoneLocalTimeInput>,
{
}

/// A struct implementing traits for never loading data.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty marker enum
pub enum NeoNeverMarker {}

impl UnstableSealed for NeoNeverMarker {}

impl DateInputMarkers for NeoNeverMarker {
    type YearInput = ();
    type MonthInput = ();
    type DayOfMonthInput = ();
    type DayOfWeekInput = ();
    type AnyCalendarKindInput = ();
}

impl<C> TypedDateDataMarkers<C> for NeoNeverMarker {
    type DateSkeletonPatternsV1Marker = NeverMarker<PackedPatternsV1<'static>>;
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl DateDataMarkers for NeoNeverMarker {
    type Skel = NoDataCalMarkers;
    type Year = NoDataCalMarkers;
    type Month = NoDataCalMarkers;
    type WeekdayNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl TimeMarkers for NeoNeverMarker {
    type HourInput = ();
    type MinuteInput = ();
    type SecondInput = ();
    type NanoSecondInput = ();
    type TimeSkeletonPatternsV1Marker = NeverMarker<PackedPatternsV1<'static>>;
    type DayPeriodNamesV1Marker = NeverMarker<LinearNamesV1<'static>>;
}

impl ZoneMarkers for NeoNeverMarker {
    type TimeZoneIdInput = ();
    type TimeZoneOffsetInput = ();
    type TimeZoneVariantInput = ();
    type TimeZoneLocalTimeInput = ();
    type EssentialsV1Marker = NeverMarker<tz::EssentialsV1<'static>>;
    type LocationsV1Marker = NeverMarker<tz::LocationsV1<'static>>;
    type GenericLongV1Marker = NeverMarker<tz::MzGenericV1<'static>>;
    type GenericShortV1Marker = NeverMarker<tz::MzGenericV1<'static>>;
    type SpecificLongV1Marker = NeverMarker<tz::MzSpecificV1<'static>>;
    type SpecificShortV1Marker = NeverMarker<tz::MzSpecificV1<'static>>;
    type MetazonePeriodV1Marker = NeverMarker<tz::MzPeriodV1<'static>>;
}

macro_rules! datetime_marker_helper {
    (@years/typed, yes) => {
        C::YearNamesV1Marker
    };
    (@years/typed,) => {
        NeverMarker<YearNamesV1<'static>>
    };
    (@months/typed, yes) => {
        C::MonthNamesV1Marker
    };
    (@months/typed,) => {
        NeverMarker<MonthNamesV1<'static>>
    };
    (@dates/typed, yes) => {
        C::SkeletaV1Marker
    };
    (@dates/typed,) => {
        NeverMarker<PackedPatternsV1<'static>>
    };
    (@calmarkers, yes) => {
        FullDataCalMarkers
    };
    (@calmarkers,) => {
        NoDataCalMarkers
    };
    (@weekdays, yes) => {
        WeekdayNamesV1Marker
    };
    (@weekdays,) => {
        NeverMarker<LinearNamesV1<'static>>
    };
    (@dayperiods, yes) => {
        DayPeriodNamesV1Marker
    };
    (@dayperiods,) => {
        NeverMarker<LinearNamesV1<'static>>
    };
    (@times, yes) => {
        TimeNeoSkeletonPatternsV1Marker
    };
    (@times,) => {
        NeverMarker<ErasedPackedPatterns>
    };
    (@glue, yes) => {
        GluePatternV1Marker
    };
    (@glue,) => {
        NeverMarker<GluePatternV1<'static>>
    };
    (@option/length, yes) => {
        NeoSkeletonLength
    };
    (@option/length, Long) => {
        NeoSkeletonLength
    };
    (@option/length, Medium) => {
        NeoSkeletonLength
    };
    (@option/length, Short) => {
        NeoSkeletonLength
    };
    (@option/yearstyle, yes) => {
        Option<YearStyle>
    };
    (@option/alignment, yes) => {
        Option<Alignment>
    };
    (@option/fractionalsecondigits, yes) => {
        Option<FractionalSecondDigits>
    };
    (@option/$any:ident,) => {
        ()
    };
    (@input/year, yes) => {
        YearInfo
    };
    (@input/month, yes) => {
        MonthInfo
    };
    (@input/day_of_month, yes) => {
        DayOfMonth
    };
    (@input/day_of_week, yes) => {
        IsoWeekday
    };
    (@input/day_of_year, yes) => {
        DayOfYearInfo
    };
    (@input/any_calendar_kind, yes) => {
        AnyCalendarKind
    };
    (@input/hour, yes) => {
        IsoHour
    };
    (@input/minute, yes) => {
        IsoMinute
    };
    (@input/second, yes) => {
        IsoSecond
    };
    (@input/nanosecond, yes) => {
        NanoSecond
    };
    (@input/timezone/id, yes) => {
        TimeZoneBcp47Id
    };
    (@input/timezone/offset, yes) => {
        Option<UtcOffset>
    };
    (@input/timezone/variant, yes) => {
        ZoneVariant
    };
    (@input/timezone/local_time, yes) => {
        (Date<Iso>, Time)
    };
    (@input/timezone/$any:ident,) => {
        ()
    };
    (@input/$any:ident,) => {
        ()
    };
    (@data/zone/essentials, yes) => {
        tz::EssentialsV1Marker
    };
    (@data/zone/locations, yes) => {
        tz::LocationsV1Marker
    };
    (@data/zone/generic_long, yes) => {
        tz::MzGenericLongV1Marker
    };
    (@data/zone/generic_short, yes) => {
        tz::MzGenericShortV1Marker
    };
    (@data/zone/specific_long, yes) => {
        tz::MzSpecificLongV1Marker
    };
    (@data/zone/specific_short, yes) => {
        tz::MzSpecificShortV1Marker
    };
    (@data/zone/metazone_periods, yes) => {
        tz::MzPeriodV1Marker
    };
    (@data/zone/essentials,) => {
        NeverMarker<tz::EssentialsV1<'static>>
    };
    (@data/zone/locations,) => {
        NeverMarker<tz::LocationsV1<'static>>
    };
    (@data/zone/generic_long,) => {
        NeverMarker<tz::MzGenericV1<'static>>
    };
    (@data/zone/generic_short,) => {
        NeverMarker<tz::MzGenericV1<'static>>
    };
    (@data/zone/specific_long,) => {
        NeverMarker<tz::MzSpecificV1<'static>>
    };
    (@data/zone/specific_short,) => {
        NeverMarker<tz::MzSpecificV1<'static>>
    };
    (@data/zone/metazone_periods,) => {
        NeverMarker<tz::MzPeriodV1<'static>>
    };
    (@names/year, yes) => {
        YearNamesV1Marker
    };
    (@names/month, yes) => {
        MonthNamesV1Marker
    };
    (@names/weekday, yes) => {
        WeekdayNamesV1Marker
    };
    (@names/dayperiod, yes) => {
        DayPeriodNamesV1Marker
    };
    (@names/zone/essentials, yes) => {
        tz::EssentialsV1Marker
    };
    (@names/zone/locations, yes) => {
        tz::LocationsV1Marker
    };
    (@names/zone/generic_long, yes) => {
        tz::MzGenericLongV1Marker
    };
    (@names/zone/generic_short, yes) => {
        tz::MzGenericShortV1Marker
    };
    (@names/zone/specific_long, yes) => {
        tz::MzSpecificLongV1Marker
    };
    (@names/zone/specific_short, yes) => {
        tz::MzSpecificShortV1Marker
    };
    (@names/zone/metazone_periods, yes) => {
        tz::MzPeriodV1Marker
    };
    (@names/$any:ident,) => {
        NeverMarker<()>
    };
    (@names/$any:ident,) => {
        NeverMarker<()>
    };
    (@names/zone/$any:ident,) => {
        NeverMarker<()>
    };
    () => {
        unreachable!() // prevent bugs
    };
}
pub(crate) use datetime_marker_helper;