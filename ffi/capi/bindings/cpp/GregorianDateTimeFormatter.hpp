#ifndef GregorianDateTimeFormatter_HPP
#define GregorianDateTimeFormatter_HPP

#include "GregorianDateTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataProvider.hpp"
#include "DateLength.hpp"
#include "Error.hpp"
#include "IsoDateTime.hpp"
#include "Locale.hpp"
#include "TimeLength.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XGregorianDateTimeFormatter_create_with_lengths_result {union {diplomat::capi::GregorianDateTimeFormatter* ok; diplomat::capi::Error err;}; bool is_ok;} ICU4XGregorianDateTimeFormatter_create_with_lengths_result;
    ICU4XGregorianDateTimeFormatter_create_with_lengths_result ICU4XGregorianDateTimeFormatter_create_with_lengths(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale, diplomat::capi::DateLength date_length, diplomat::capi::TimeLength time_length);
    
    void ICU4XGregorianDateTimeFormatter_format_iso_datetime(const diplomat::capi::GregorianDateTimeFormatter* self, const diplomat::capi::IsoDateTime* value, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XGregorianDateTimeFormatter_destroy(GregorianDateTimeFormatter* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error> GregorianDateTimeFormatter::create_with_lengths(const DataProvider& provider, const Locale& locale, DateLength date_length, TimeLength time_length) {
  auto result = diplomat::capi::ICU4XGregorianDateTimeFormatter_create_with_lengths(provider.AsFFI(),
    locale.AsFFI(),
    date_length.AsFFI(),
    time_length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error>(diplomat::Ok<std::unique_ptr<GregorianDateTimeFormatter>>(std::unique_ptr<GregorianDateTimeFormatter>(GregorianDateTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GregorianDateTimeFormatter>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)));
}

inline std::string GregorianDateTimeFormatter::format_iso_datetime(const IsoDateTime& value) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XGregorianDateTimeFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const diplomat::capi::GregorianDateTimeFormatter* GregorianDateTimeFormatter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::GregorianDateTimeFormatter*>(this);
}

inline diplomat::capi::GregorianDateTimeFormatter* GregorianDateTimeFormatter::AsFFI() {
  return reinterpret_cast<diplomat::capi::GregorianDateTimeFormatter*>(this);
}

inline const GregorianDateTimeFormatter* GregorianDateTimeFormatter::FromFFI(const diplomat::capi::GregorianDateTimeFormatter* ptr) {
  return reinterpret_cast<const GregorianDateTimeFormatter*>(ptr);
}

inline GregorianDateTimeFormatter* GregorianDateTimeFormatter::FromFFI(diplomat::capi::GregorianDateTimeFormatter* ptr) {
  return reinterpret_cast<GregorianDateTimeFormatter*>(ptr);
}

inline void GregorianDateTimeFormatter::operator delete(void* ptr) {
  diplomat::capi::ICU4XGregorianDateTimeFormatter_destroy(reinterpret_cast<diplomat::capi::GregorianDateTimeFormatter*>(ptr));
}


#endif // GregorianDateTimeFormatter_HPP