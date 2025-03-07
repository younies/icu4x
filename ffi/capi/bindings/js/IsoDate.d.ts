// generated by diplomat-tool
import type { Calendar } from "./Calendar"
import type { CalendarError } from "./CalendarError"
import type { CalendarParseError } from "./CalendarParseError"
import type { Date } from "./Date"
import type { WeekCalculator } from "./WeekCalculator"
import type { WeekOf } from "./WeekOf"
import type { Weekday } from "./Weekday"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X Date object capable of containing a ISO-8601 date
*
*See the [Rust documentation for `Date`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html) for more information.
*/


export class IsoDate {
    
    get ffiValue(): pointer;

    static fromString(v: string): IsoDate;

    toCalendar(calendar: Calendar): Date;

    toAny(): Date;

    get dayOfYear(): number;

    get dayOfMonth(): number;

    get dayOfWeek(): Weekday;

    weekOfMonth(firstWeekday: Weekday): number;

    weekOfYear(calculator: WeekCalculator): WeekOf;

    get month(): number;

    get year(): number;

    get isInLeapYear(): boolean;

    get monthsInYear(): number;

    get daysInMonth(): number;

    get daysInYear(): number;

    constructor(year: number, month: number, day: number);
}