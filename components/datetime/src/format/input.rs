// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A collection of utilities for representing and working with dates as an input to
//! formatting operations.

use crate::scaffold::{DateInputMarkers, GetField, TimeMarkers, ZoneMarkers};
use icu_calendar::types::DayOfYearInfo;
use icu_calendar::Iso;
use icu_time::scaffold::IntoOption;
use icu_time::{zone::TimeZoneVariant, Hour, Minute, Nanosecond, Second};

use icu_calendar::Date;
use icu_time::{zone::UtcOffset, Time, TimeZone};

// TODO(#2630) fix up imports to directly import from icu_calendar
pub(crate) use icu_calendar::types::{DayOfMonth, MonthInfo, Weekday, YearInfo};

#[derive(Debug, Copy, Clone)]
pub(crate) struct ExtractedInput {
    pub(crate) year: Option<YearInfo>,
    pub(crate) month: Option<MonthInfo>,
    pub(crate) day_of_month: Option<DayOfMonth>,
    pub(crate) iso_weekday: Option<Weekday>,
    pub(crate) day_of_year: Option<DayOfYearInfo>,
    pub(crate) hour: Option<Hour>,
    pub(crate) minute: Option<Minute>,
    pub(crate) second: Option<Second>,
    pub(crate) subsecond: Option<Nanosecond>,
    pub(crate) time_zone_id: Option<TimeZone>,
    pub(crate) offset: Option<UtcOffset>,
    pub(crate) zone_variant: Option<TimeZoneVariant>,
    pub(crate) local_time: Option<(Date<Iso>, Time)>,
}

impl ExtractedInput {
    /// Construct given neo date input instances.
    pub(crate) fn extract_from_neo_input<D, T, Z, I>(input: &I) -> Self
    where
        D: DateInputMarkers,
        T: TimeMarkers,
        Z: ZoneMarkers,
        I: ?Sized
            + GetField<D::YearInput>
            + GetField<D::MonthInput>
            + GetField<D::DayOfMonthInput>
            + GetField<D::DayOfWeekInput>
            + GetField<D::DayOfYearInput>
            + GetField<T::HourInput>
            + GetField<T::MinuteInput>
            + GetField<T::SecondInput>
            + GetField<T::NanosecondInput>
            + GetField<Z::TimeZoneIdInput>
            + GetField<Z::TimeZoneOffsetInput>
            + GetField<Z::TimeZoneVariantInput>
            + GetField<Z::TimeZoneLocalTimeInput>,
    {
        Self {
            year: GetField::<D::YearInput>::get_field(input).into_option(),
            month: GetField::<D::MonthInput>::get_field(input).into_option(),
            day_of_month: GetField::<D::DayOfMonthInput>::get_field(input).into_option(),
            iso_weekday: GetField::<D::DayOfWeekInput>::get_field(input).into_option(),
            day_of_year: GetField::<D::DayOfYearInput>::get_field(input).into_option(),
            hour: GetField::<T::HourInput>::get_field(input).into_option(),
            minute: GetField::<T::MinuteInput>::get_field(input).into_option(),
            second: GetField::<T::SecondInput>::get_field(input).into_option(),
            subsecond: GetField::<T::NanosecondInput>::get_field(input).into_option(),
            time_zone_id: GetField::<Z::TimeZoneIdInput>::get_field(input).into_option(),
            offset: GetField::<Z::TimeZoneOffsetInput>::get_field(input).into_option(),
            zone_variant: GetField::<Z::TimeZoneVariantInput>::get_field(input).into_option(),
            local_time: GetField::<Z::TimeZoneLocalTimeInput>::get_field(input).into_option(),
        }
    }
}
