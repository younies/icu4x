#ifndef icu4x_WordBreakIteratorUtf16_HPP
#define icu4x_WordBreakIteratorUtf16_HPP

#include "WordBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "SegmenterWordType.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    int32_t icu4x_WordBreakIteratorUtf16_next_mv1(icu4x::capi::WordBreakIteratorUtf16* self);

    icu4x::capi::SegmenterWordType icu4x_WordBreakIteratorUtf16_word_type_mv1(const icu4x::capi::WordBreakIteratorUtf16* self);

    bool icu4x_WordBreakIteratorUtf16_is_word_like_mv1(const icu4x::capi::WordBreakIteratorUtf16* self);

    void icu4x_WordBreakIteratorUtf16_destroy_mv1(WordBreakIteratorUtf16* self);

    } // extern "C"
} // namespace capi
} // namespace

inline int32_t icu4x::WordBreakIteratorUtf16::next() {
  auto result = icu4x::capi::icu4x_WordBreakIteratorUtf16_next_mv1(this->AsFFI());
  return result;
}

inline icu4x::SegmenterWordType icu4x::WordBreakIteratorUtf16::word_type() const {
  auto result = icu4x::capi::icu4x_WordBreakIteratorUtf16_word_type_mv1(this->AsFFI());
  return icu4x::SegmenterWordType::FromFFI(result);
}

inline bool icu4x::WordBreakIteratorUtf16::is_word_like() const {
  auto result = icu4x::capi::icu4x_WordBreakIteratorUtf16_is_word_like_mv1(this->AsFFI());
  return result;
}

inline const icu4x::capi::WordBreakIteratorUtf16* icu4x::WordBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::WordBreakIteratorUtf16*>(this);
}

inline icu4x::capi::WordBreakIteratorUtf16* icu4x::WordBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<icu4x::capi::WordBreakIteratorUtf16*>(this);
}

inline const icu4x::WordBreakIteratorUtf16* icu4x::WordBreakIteratorUtf16::FromFFI(const icu4x::capi::WordBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const icu4x::WordBreakIteratorUtf16*>(ptr);
}

inline icu4x::WordBreakIteratorUtf16* icu4x::WordBreakIteratorUtf16::FromFFI(icu4x::capi::WordBreakIteratorUtf16* ptr) {
  return reinterpret_cast<icu4x::WordBreakIteratorUtf16*>(ptr);
}

inline void icu4x::WordBreakIteratorUtf16::operator delete(void* ptr) {
  icu4x::capi::icu4x_WordBreakIteratorUtf16_destroy_mv1(reinterpret_cast<icu4x::capi::WordBreakIteratorUtf16*>(ptr));
}


#endif // icu4x_WordBreakIteratorUtf16_HPP
