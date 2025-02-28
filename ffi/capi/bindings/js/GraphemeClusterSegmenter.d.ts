// generated by diplomat-tool
import type { DataError } from "./DataError"
import type { DataProvider } from "./DataProvider"
import type { GraphemeClusterBreakIteratorUtf16 } from "./GraphemeClusterBreakIteratorUtf16"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints
*in strings.
*
*See the [Rust documentation for `GraphemeClusterSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html) for more information.
*/


export class GraphemeClusterSegmenter {
    
    get ffiValue(): pointer;

    static createWithProvider(provider: DataProvider): GraphemeClusterSegmenter;

    segment(input: string): GraphemeClusterBreakIteratorUtf16;

    constructor();
}