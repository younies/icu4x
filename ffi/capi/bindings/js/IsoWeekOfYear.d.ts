// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

export type IsoWeekOfYear_obj = {
    weekNumber: number;
    isoYear: number;
};



export class IsoWeekOfYear {
    get weekNumber(): number;
    set weekNumber(value: number);
    get isoYear(): number;
    set isoYear(value: number);
    /** @internal */
    static fromFields(structObj : IsoWeekOfYear_obj) : IsoWeekOfYear;

    /**
    * Create `IsoWeekOfYear` from an object that contains all of `IsoWeekOfYear`s fields.
    * Optional fields do not need to be included in the provided object.
    */
    constructor(structObj: IsoWeekOfYear_obj);

}