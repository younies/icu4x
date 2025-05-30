// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



/**
 * See the [Rust documentation for `IndicSyllabicCategory`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.IndicSyllabicCategory.html) for more information.
 */
export class IndicSyllabicCategory {
    #value = undefined;

    static #values = new Map([
        ["Other", 0],
        ["Avagraha", 1],
        ["Bindu", 2],
        ["BrahmiJoiningNumber", 3],
        ["CantillationMark", 4],
        ["Consonant", 5],
        ["ConsonantDead", 6],
        ["ConsonantFinal", 7],
        ["ConsonantHeadLetter", 8],
        ["ConsonantInitialPostfixed", 9],
        ["ConsonantKiller", 10],
        ["ConsonantMedial", 11],
        ["ConsonantPlaceholder", 12],
        ["ConsonantPrecedingRepha", 13],
        ["ConsonantPrefixed", 14],
        ["ConsonantSucceedingRepha", 15],
        ["ConsonantSubjoined", 16],
        ["ConsonantWithStacker", 17],
        ["GeminationMark", 18],
        ["InvisibleStacker", 19],
        ["Joiner", 20],
        ["ModifyingLetter", 21],
        ["NonJoiner", 22],
        ["Nukta", 23],
        ["Number", 24],
        ["NumberJoiner", 25],
        ["PureKiller", 26],
        ["RegisterShifter", 27],
        ["SyllableModifier", 28],
        ["ToneLetter", 29],
        ["ToneMark", 30],
        ["Virama", 31],
        ["Visarga", 32],
        ["Vowel", 33],
        ["VowelDependent", 34],
        ["VowelIndependent", 35],
        ["ReorderingKiller", 36]
    ]);

    static getAllEntries() {
        return IndicSyllabicCategory.#values.entries();
    }

    #internalConstructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return this;
            }
            return IndicSyllabicCategory.#objectValues[arguments[1]];
        }

        if (value instanceof IndicSyllabicCategory) {
            return value;
        }

        let intVal = IndicSyllabicCategory.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal != null) {
            return IndicSyllabicCategory.#objectValues[intVal];
        }

        throw TypeError(value + " is not a IndicSyllabicCategory and does not correspond to any of its enumerator values.");
    }

    /** @internal */
    static fromValue(value) {
        return new IndicSyllabicCategory(value);
    }

    get value(){
        return [...IndicSyllabicCategory.#values.keys()][this.#value];
    }

    /** @internal */
    get ffiValue(){
        return this.#value;
    }
    static #objectValues = [
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 4),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 5),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 6),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 7),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 8),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 9),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 10),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 11),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 12),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 13),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 14),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 15),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 16),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 17),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 18),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 19),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 20),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 21),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 22),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 23),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 24),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 25),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 26),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 27),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 28),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 29),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 30),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 31),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 32),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 33),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 34),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 35),
        new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 36),
    ];

    static Other = IndicSyllabicCategory.#objectValues[0];
    static Avagraha = IndicSyllabicCategory.#objectValues[1];
    static Bindu = IndicSyllabicCategory.#objectValues[2];
    static BrahmiJoiningNumber = IndicSyllabicCategory.#objectValues[3];
    static CantillationMark = IndicSyllabicCategory.#objectValues[4];
    static Consonant = IndicSyllabicCategory.#objectValues[5];
    static ConsonantDead = IndicSyllabicCategory.#objectValues[6];
    static ConsonantFinal = IndicSyllabicCategory.#objectValues[7];
    static ConsonantHeadLetter = IndicSyllabicCategory.#objectValues[8];
    static ConsonantInitialPostfixed = IndicSyllabicCategory.#objectValues[9];
    static ConsonantKiller = IndicSyllabicCategory.#objectValues[10];
    static ConsonantMedial = IndicSyllabicCategory.#objectValues[11];
    static ConsonantPlaceholder = IndicSyllabicCategory.#objectValues[12];
    static ConsonantPrecedingRepha = IndicSyllabicCategory.#objectValues[13];
    static ConsonantPrefixed = IndicSyllabicCategory.#objectValues[14];
    static ConsonantSucceedingRepha = IndicSyllabicCategory.#objectValues[15];
    static ConsonantSubjoined = IndicSyllabicCategory.#objectValues[16];
    static ConsonantWithStacker = IndicSyllabicCategory.#objectValues[17];
    static GeminationMark = IndicSyllabicCategory.#objectValues[18];
    static InvisibleStacker = IndicSyllabicCategory.#objectValues[19];
    static Joiner = IndicSyllabicCategory.#objectValues[20];
    static ModifyingLetter = IndicSyllabicCategory.#objectValues[21];
    static NonJoiner = IndicSyllabicCategory.#objectValues[22];
    static Nukta = IndicSyllabicCategory.#objectValues[23];
    static Number = IndicSyllabicCategory.#objectValues[24];
    static NumberJoiner = IndicSyllabicCategory.#objectValues[25];
    static PureKiller = IndicSyllabicCategory.#objectValues[26];
    static RegisterShifter = IndicSyllabicCategory.#objectValues[27];
    static SyllableModifier = IndicSyllabicCategory.#objectValues[28];
    static ToneLetter = IndicSyllabicCategory.#objectValues[29];
    static ToneMark = IndicSyllabicCategory.#objectValues[30];
    static Virama = IndicSyllabicCategory.#objectValues[31];
    static Visarga = IndicSyllabicCategory.#objectValues[32];
    static Vowel = IndicSyllabicCategory.#objectValues[33];
    static VowelDependent = IndicSyllabicCategory.#objectValues[34];
    static VowelIndependent = IndicSyllabicCategory.#objectValues[35];
    static ReorderingKiller = IndicSyllabicCategory.#objectValues[36];


    /**
     * See the [Rust documentation for `for_char`](https://docs.rs/icu/2.0.0/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
     */
    static forChar(ch) {

        const result = wasm.icu4x_IndicSyllabicCategory_for_char_mv1(ch);

        try {
            return new IndicSyllabicCategory(diplomatRuntime.internalConstructor, result);
        }

        finally {
        }
    }

    /**
     * Convert to an integer value usable with ICU4C and CodePointMapData
     *
     * See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.IndicSyllabicCategory.html#method.to_icu4c_value) for more information.
     */
    toIntegerValue() {

        const result = wasm.icu4x_IndicSyllabicCategory_to_integer_value_mv1(this.ffiValue);

        try {
            return result;
        }

        finally {
        }
    }

    /**
     * Convert from an integer value from ICU4C or CodePointMapData
     *
     * See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.IndicSyllabicCategory.html#method.from_icu4c_value) for more information.
     */
    static fromIntegerValue(other) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.icu4x_IndicSyllabicCategory_from_integer_value_mv1(diplomatReceive.buffer, other);

        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return new IndicSyllabicCategory(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
        }

        finally {
            diplomatReceive.free();
        }
    }

    constructor(value) {
        return this.#internalConstructor(...arguments)
    }
}