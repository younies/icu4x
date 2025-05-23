// generated by diplomat-tool
// dart format off

part of 'lib.g.dart';

/// See the [Rust documentation for `BidiClass`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html) for more information.
enum BidiClass {
  /// See the [Rust documentation for `LeftToRight`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.LeftToRight) for more information.
  leftToRight,
  /// See the [Rust documentation for `RightToLeft`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.RightToLeft) for more information.
  rightToLeft,
  /// See the [Rust documentation for `EuropeanNumber`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.EuropeanNumber) for more information.
  europeanNumber,
  /// See the [Rust documentation for `EuropeanSeparator`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.EuropeanSeparator) for more information.
  europeanSeparator,
  /// See the [Rust documentation for `EuropeanTerminator`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.EuropeanTerminator) for more information.
  europeanTerminator,
  /// See the [Rust documentation for `ArabicNumber`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.ArabicNumber) for more information.
  arabicNumber,
  /// See the [Rust documentation for `CommonSeparator`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.CommonSeparator) for more information.
  commonSeparator,
  /// See the [Rust documentation for `ParagraphSeparator`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.ParagraphSeparator) for more information.
  paragraphSeparator,
  /// See the [Rust documentation for `SegmentSeparator`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.SegmentSeparator) for more information.
  segmentSeparator,
  /// See the [Rust documentation for `WhiteSpace`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.WhiteSpace) for more information.
  whiteSpace,
  /// See the [Rust documentation for `OtherNeutral`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.OtherNeutral) for more information.
  otherNeutral,
  /// See the [Rust documentation for `LeftToRightEmbedding`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.LeftToRightEmbedding) for more information.
  leftToRightEmbedding,
  /// See the [Rust documentation for `LeftToRightOverride`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.LeftToRightOverride) for more information.
  leftToRightOverride,
  /// See the [Rust documentation for `ArabicLetter`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.ArabicLetter) for more information.
  arabicLetter,
  /// See the [Rust documentation for `RightToLeftEmbedding`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.RightToLeftEmbedding) for more information.
  rightToLeftEmbedding,
  /// See the [Rust documentation for `RightToLeftOverride`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.RightToLeftOverride) for more information.
  rightToLeftOverride,
  /// See the [Rust documentation for `PopDirectionalFormat`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.PopDirectionalFormat) for more information.
  popDirectionalFormat,
  /// See the [Rust documentation for `NonspacingMark`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.NonspacingMark) for more information.
  nonspacingMark,
  /// See the [Rust documentation for `BoundaryNeutral`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.BoundaryNeutral) for more information.
  boundaryNeutral,
  /// See the [Rust documentation for `FirstStrongIsolate`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.FirstStrongIsolate) for more information.
  firstStrongIsolate,
  /// See the [Rust documentation for `LeftToRightIsolate`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.LeftToRightIsolate) for more information.
  leftToRightIsolate,
  /// See the [Rust documentation for `RightToLeftIsolate`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.RightToLeftIsolate) for more information.
  rightToLeftIsolate,
  /// See the [Rust documentation for `PopDirectionalIsolate`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#associatedconstant.PopDirectionalIsolate) for more information.
  popDirectionalIsolate;

  /// See the [Rust documentation for `for_char`](https://docs.rs/icu/2.0.0/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
  static BidiClass forChar(Rune ch) {
    final result = _icu4x_BidiClass_for_char_mv1(ch);
    return BidiClass.values[result];
  }

  /// Get the "long" name of this property value (returns empty if property value is unknown)
  ///
  /// See the [Rust documentation for `get`](https://docs.rs/icu/2.0.0/icu/properties/struct.PropertyNamesLongBorrowed.html#method.get) for more information.
  String? longName() {
    final result = _icu4x_BidiClass_long_name_mv1(index);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok._toDart([], isStatic: true);
  }

  /// Get the "short" name of this property value (returns empty if property value is unknown)
  ///
  /// See the [Rust documentation for `get`](https://docs.rs/icu/2.0.0/icu/properties/struct.PropertyNamesShortBorrowed.html#method.get) for more information.
  String? shortName() {
    final result = _icu4x_BidiClass_short_name_mv1(index);
    if (!result.isOk) {
      return null;
    }
    return result.union.ok._toDart([], isStatic: true);
  }

  /// Convert to an integer value usable with ICU4C and CodePointMapData
  ///
  /// See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#method.to_icu4c_value) for more information.
  int toIntegerValue() {
    final result = _icu4x_BidiClass_to_integer_value_mv1(index);
    return result;
  }

  /// Convert from an integer value from ICU4C or CodePointMapData
  ///
  /// See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.0.0/icu/properties/props/struct.BidiClass.html#method.from_icu4c_value) for more information.
  static BidiClass? fromIntegerValue(int other) {
    final result = _icu4x_BidiClass_from_integer_value_mv1(other);
    if (!result.isOk) {
      return null;
    }
    return BidiClass.values[result.union.ok];
  }

}

@_DiplomatFfiUse('icu4x_BidiClass_for_char_mv1')
@ffi.Native<ffi.Int32 Function(ffi.Uint32)>(isLeaf: true, symbol: 'icu4x_BidiClass_for_char_mv1')
// ignore: non_constant_identifier_names
external int _icu4x_BidiClass_for_char_mv1(Rune ch);

@_DiplomatFfiUse('icu4x_BidiClass_long_name_mv1')
@ffi.Native<_ResultSliceUtf8Void Function(ffi.Int32)>(isLeaf: true, symbol: 'icu4x_BidiClass_long_name_mv1')
// ignore: non_constant_identifier_names
external _ResultSliceUtf8Void _icu4x_BidiClass_long_name_mv1(int self);

@_DiplomatFfiUse('icu4x_BidiClass_short_name_mv1')
@ffi.Native<_ResultSliceUtf8Void Function(ffi.Int32)>(isLeaf: true, symbol: 'icu4x_BidiClass_short_name_mv1')
// ignore: non_constant_identifier_names
external _ResultSliceUtf8Void _icu4x_BidiClass_short_name_mv1(int self);

@_DiplomatFfiUse('icu4x_BidiClass_to_integer_value_mv1')
@ffi.Native<ffi.Uint8 Function(ffi.Int32)>(isLeaf: true, symbol: 'icu4x_BidiClass_to_integer_value_mv1')
// ignore: non_constant_identifier_names
external int _icu4x_BidiClass_to_integer_value_mv1(int self);

@_DiplomatFfiUse('icu4x_BidiClass_from_integer_value_mv1')
@ffi.Native<_ResultInt32Void Function(ffi.Uint8)>(isLeaf: true, symbol: 'icu4x_BidiClass_from_integer_value_mv1')
// ignore: non_constant_identifier_names
external _ResultInt32Void _icu4x_BidiClass_from_integer_value_mv1(int other);

// dart format on
