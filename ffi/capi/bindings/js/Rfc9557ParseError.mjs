// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



/**
 * Additional information: [1](https://docs.rs/icu/2.0.0/icu/calendar/enum.ParseError.html), [2](https://docs.rs/icu/2.0.0/icu/time/enum.ParseError.html)
 */
export class Rfc9557ParseError {
    #value = undefined;

    static #values = new Map([
        ["Unknown", 0],
        ["InvalidSyntax", 1],
        ["OutOfRange", 2],
        ["MissingFields", 3],
        ["UnknownCalendar", 4]
    ]);

    static getAllEntries() {
        return Rfc9557ParseError.#values.entries();
    }

    #internalConstructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return this;
            }
            return Rfc9557ParseError.#objectValues[arguments[1]];
        }

        if (value instanceof Rfc9557ParseError) {
            return value;
        }

        let intVal = Rfc9557ParseError.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal != null) {
            return Rfc9557ParseError.#objectValues[intVal];
        }

        throw TypeError(value + " is not a Rfc9557ParseError and does not correspond to any of its enumerator values.");
    }

    /** @internal */
    static fromValue(value) {
        return new Rfc9557ParseError(value);
    }

    get value(){
        return [...Rfc9557ParseError.#values.keys()][this.#value];
    }

    /** @internal */
    get ffiValue(){
        return this.#value;
    }
    static #objectValues = [
        new Rfc9557ParseError(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new Rfc9557ParseError(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new Rfc9557ParseError(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new Rfc9557ParseError(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
        new Rfc9557ParseError(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 4),
    ];

    static Unknown = Rfc9557ParseError.#objectValues[0];
    static InvalidSyntax = Rfc9557ParseError.#objectValues[1];
    static OutOfRange = Rfc9557ParseError.#objectValues[2];
    static MissingFields = Rfc9557ParseError.#objectValues[3];
    static UnknownCalendar = Rfc9557ParseError.#objectValues[4];


    constructor(value) {
        return this.#internalConstructor(...arguments)
    }
}