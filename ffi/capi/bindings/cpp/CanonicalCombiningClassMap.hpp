#ifndef CanonicalCombiningClassMap_HPP
#define CanonicalCombiningClassMap_HPP

#include "CanonicalCombiningClassMap.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XCanonicalCombiningClassMap_create_result {union {diplomat::capi::CanonicalCombiningClassMap* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCanonicalCombiningClassMap_create_result;
    ICU4XCanonicalCombiningClassMap_create_result ICU4XCanonicalCombiningClassMap_create(const diplomat::capi::DataProvider* provider);
    
    uint8_t ICU4XCanonicalCombiningClassMap_get(const diplomat::capi::CanonicalCombiningClassMap* self, char32_t ch);
    
    uint8_t ICU4XCanonicalCombiningClassMap_get32(const diplomat::capi::CanonicalCombiningClassMap* self, uint32_t ch);
    
    
    void ICU4XCanonicalCombiningClassMap_destroy(CanonicalCombiningClassMap* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError> CanonicalCombiningClassMap::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCanonicalCombiningClassMap_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError>(diplomat::Ok<std::unique_ptr<CanonicalCombiningClassMap>>(std::unique_ptr<CanonicalCombiningClassMap>(CanonicalCombiningClassMap::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CanonicalCombiningClassMap>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline uint8_t CanonicalCombiningClassMap::get(char32_t ch) const {
  auto result = diplomat::capi::ICU4XCanonicalCombiningClassMap_get(this->AsFFI(),
    ch);
  return result;
}

inline uint8_t CanonicalCombiningClassMap::get32(uint32_t ch) const {
  auto result = diplomat::capi::ICU4XCanonicalCombiningClassMap_get32(this->AsFFI(),
    ch);
  return result;
}

inline const diplomat::capi::CanonicalCombiningClassMap* CanonicalCombiningClassMap::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CanonicalCombiningClassMap*>(this);
}

inline diplomat::capi::CanonicalCombiningClassMap* CanonicalCombiningClassMap::AsFFI() {
  return reinterpret_cast<diplomat::capi::CanonicalCombiningClassMap*>(this);
}

inline const CanonicalCombiningClassMap* CanonicalCombiningClassMap::FromFFI(const diplomat::capi::CanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<const CanonicalCombiningClassMap*>(ptr);
}

inline CanonicalCombiningClassMap* CanonicalCombiningClassMap::FromFFI(diplomat::capi::CanonicalCombiningClassMap* ptr) {
  return reinterpret_cast<CanonicalCombiningClassMap*>(ptr);
}

inline void CanonicalCombiningClassMap::operator delete(void* ptr) {
  diplomat::capi::ICU4XCanonicalCombiningClassMap_destroy(reinterpret_cast<diplomat::capi::CanonicalCombiningClassMap*>(ptr));
}


#endif // CanonicalCombiningClassMap_HPP