// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



/**
 * See the [Rust documentation for `LineBreakWordOption`](https://docs.rs/icu/2.0.0/icu/segmenter/options/enum.LineBreakWordOption.html) for more information.
 */
export class LineBreakWordOption {

    /** @internal */
    static fromValue(value: LineBreakWordOption | string): LineBreakWordOption;

    get value(): string;

    /** @internal */
    get ffiValue(): number;

    static Normal : LineBreakWordOption;
    static BreakAll : LineBreakWordOption;
    static KeepAll : LineBreakWordOption;


    constructor(value: LineBreakWordOption | string );
}