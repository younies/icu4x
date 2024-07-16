#ifndef DecomposingNormalizer_HPP
#define DecomposingNormalizer_HPP

#include "DecomposingNormalizer.d.hpp"

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
    
    typedef struct ICU4XDecomposingNormalizer_create_nfd_result {union {diplomat::capi::DecomposingNormalizer* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfd_result;
    ICU4XDecomposingNormalizer_create_nfd_result ICU4XDecomposingNormalizer_create_nfd(const diplomat::capi::DataProvider* provider);
    
    typedef struct ICU4XDecomposingNormalizer_create_nfkd_result {union {diplomat::capi::DecomposingNormalizer* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XDecomposingNormalizer_create_nfkd_result;
    ICU4XDecomposingNormalizer_create_nfkd_result ICU4XDecomposingNormalizer_create_nfkd(const diplomat::capi::DataProvider* provider);
    
    void ICU4XDecomposingNormalizer_normalize(const diplomat::capi::DecomposingNormalizer* self, const char* s_data, size_t s_len, diplomat::capi::DiplomatWrite* write);
    
    bool ICU4XDecomposingNormalizer_is_normalized(const diplomat::capi::DecomposingNormalizer* self, const char* s_data, size_t s_len);
    
    bool ICU4XDecomposingNormalizer_is_normalized_utf16(const diplomat::capi::DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    size_t ICU4XDecomposingNormalizer_is_normalized_up_to(const diplomat::capi::DecomposingNormalizer* self, const char* s_data, size_t s_len);
    
    size_t ICU4XDecomposingNormalizer_is_normalized_utf16_up_to(const diplomat::capi::DecomposingNormalizer* self, const char16_t* s_data, size_t s_len);
    
    
    void ICU4XDecomposingNormalizer_destroy(DecomposingNormalizer* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError> DecomposingNormalizer::create_nfd(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XDecomposingNormalizer_create_nfd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<DecomposingNormalizer>>(std::unique_ptr<DecomposingNormalizer>(DecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError> DecomposingNormalizer::create_nfkd(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XDecomposingNormalizer_create_nfkd(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Ok<std::unique_ptr<DecomposingNormalizer>>(std::unique_ptr<DecomposingNormalizer>(DecomposingNormalizer::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string DecomposingNormalizer::normalize(std::string_view s) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XDecomposingNormalizer_normalize(this->AsFFI(),
    s.data(),
    s.size(),
    &write);
  return output;
}

inline bool DecomposingNormalizer::is_normalized(std::string_view s) const {
  auto result = diplomat::capi::ICU4XDecomposingNormalizer_is_normalized(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline bool DecomposingNormalizer::is_normalized_utf16(std::u16string_view s) const {
  auto result = diplomat::capi::ICU4XDecomposingNormalizer_is_normalized_utf16(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t DecomposingNormalizer::is_normalized_up_to(std::string_view s) const {
  auto result = diplomat::capi::ICU4XDecomposingNormalizer_is_normalized_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline size_t DecomposingNormalizer::is_normalized_utf16_up_to(std::u16string_view s) const {
  auto result = diplomat::capi::ICU4XDecomposingNormalizer_is_normalized_utf16_up_to(this->AsFFI(),
    s.data(),
    s.size());
  return result;
}

inline const diplomat::capi::DecomposingNormalizer* DecomposingNormalizer::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::DecomposingNormalizer*>(this);
}

inline diplomat::capi::DecomposingNormalizer* DecomposingNormalizer::AsFFI() {
  return reinterpret_cast<diplomat::capi::DecomposingNormalizer*>(this);
}

inline const DecomposingNormalizer* DecomposingNormalizer::FromFFI(const diplomat::capi::DecomposingNormalizer* ptr) {
  return reinterpret_cast<const DecomposingNormalizer*>(ptr);
}

inline DecomposingNormalizer* DecomposingNormalizer::FromFFI(diplomat::capi::DecomposingNormalizer* ptr) {
  return reinterpret_cast<DecomposingNormalizer*>(ptr);
}

inline void DecomposingNormalizer::operator delete(void* ptr) {
  diplomat::capi::ICU4XDecomposingNormalizer_destroy(reinterpret_cast<diplomat::capi::DecomposingNormalizer*>(ptr));
}


#endif // DecomposingNormalizer_HPP