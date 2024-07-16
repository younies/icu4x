import { u8, u16, i32, u32 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { Calendar } from "./Calendar";
import { CalendarError } from "./CalendarError";
import { IsoDate } from "./IsoDate";
import { IsoWeekday } from "./IsoWeekday";
import { WeekCalculator } from "./WeekCalculator";
import { WeekOf } from "./WeekOf";

/**

 * An ICU4X Date object capable of containing a date and time for any calendar.

 * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html Rust documentation for `Date`} for more information.
 */
export class Date {

  /**

   * Creates a new {@link Date `Date`} representing the ISO date and time given but in a given calendar

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.new_from_iso Rust documentation for `new_from_iso`} for more information.
   * @throws {@link FFIError}<{@link CalendarError}>
   */
  static create_from_iso_in_calendar(year: i32, month: u8, day: u8, calendar: Calendar): Date | never;

  /**

   * Creates a new {@link Date `Date`} from the given codes, which are interpreted in the given calendar system

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.try_new_from_codes Rust documentation for `try_new_from_codes`} for more information.
   * @throws {@link FFIError}<{@link CalendarError}>
   */
  static create_from_codes_in_calendar(era_code: string, year: i32, month_code: string, day: u8, calendar: Calendar): Date | never;

  /**

   * Convert this date to one in a different calendar

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_calendar Rust documentation for `to_calendar`} for more information.
   */
  to_calendar(calendar: Calendar): Date;

  /**

   * Converts this date to ISO

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_iso Rust documentation for `to_iso`} for more information.
   */
  to_iso(): IsoDate;

  /**

   * Returns the 1-indexed day in the year for this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_year_info Rust documentation for `day_of_year_info`} for more information.
   */
  day_of_year(): u16;

  /**

   * Returns the 1-indexed day in the month for this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_month Rust documentation for `day_of_month`} for more information.
   */
  day_of_month(): u32;

  /**

   * Returns the day in the week for this day

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_week Rust documentation for `day_of_week`} for more information.
   */
  day_of_week(): IsoWeekday;

  /**

   * Returns the week number in this month, 1-indexed, based on what is considered the first day of the week (often a locale preference).

   * `first_weekday` can be obtained via `first_weekday()` on {@link WeekCalculator `WeekCalculator`}

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_month Rust documentation for `week_of_month`} for more information.
   */
  week_of_month(first_weekday: IsoWeekday): u32;

  /**

   * Returns the week number in this year, using week data

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_year Rust documentation for `week_of_year`} for more information.
   */
  week_of_year(calculator: WeekCalculator): WeekOf;

  /**

   * Returns 1-indexed number of the month of this date in its year

   * Note that for lunar calendars this may not lead to the same month having the same ordinal month across years; use month_code if you care about month identity.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month Rust documentation for `month`} for more information.
   */
  ordinal_month(): u32;

  /**

   * Returns the month code for this date. Typically something like "M01", "M02", but can be more complicated for lunar calendars.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month Rust documentation for `month`} for more information.
   */
  month_code(): string;

  /**

   * Returns the year number in the current era for this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.year Rust documentation for `year`} for more information.
   */
  year_in_era(): i32;

  /**

   * Returns the era for this date,

   * See the {@link https://docs.rs/icu/latest/icu/struct.Date.html#method.year Rust documentation for `year`} for more information.

   * Additional information: {@link https://docs.rs/icu/latest/icu/types/struct.Era.html 1}
   */
  era(): string;

  /**

   * Returns the number of months in the year represented by this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.months_in_year Rust documentation for `months_in_year`} for more information.
   */
  months_in_year(): u8;

  /**

   * Returns the number of days in the month represented by this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_month Rust documentation for `days_in_month`} for more information.
   */
  days_in_month(): u8;

  /**

   * Returns the number of days in the year represented by this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_year Rust documentation for `days_in_year`} for more information.
   */
  days_in_year(): u16;

  /**

   * Returns the {@link Calendar `Calendar`} object backing this date

   * See the {@link https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.calendar Rust documentation for `calendar`} for more information.
   */
  calendar(): Calendar;
}