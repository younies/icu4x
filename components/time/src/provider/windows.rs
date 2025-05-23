// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A provider for mapping Windows Zones to IANA identifiers.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ZeroVec;

use super::TimeZone;

icu_provider::data_marker!(
    /// See [`WindowsZonesToBcp47Map`].
    TimezoneIdentifiersWindowsV1,
    "timezone/identifiers/windows/v1",
    WindowsZonesToBcp47Map<'static>,
    is_singleton = true,
);

/// A mapping from Windows Timezone names to the corresponding BCP-47 IDs.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Debug, Clone, zerofrom::ZeroFrom, yoke::Yokeable)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_time::provider::windows))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct WindowsZonesToBcp47Map<'data> {
    /// A map from a `WindowsZoneIdentifier` and `WindowsRegion` to indexes of the associated BCP-47 time zone identifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,

    /// A sorted list of BCP-47 time zone identifiers.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub bcp47_ids: ZeroVec<'data, TimeZone>,
}

icu_provider::data_struct!(
    WindowsZonesToBcp47Map<'_>,
    #[cfg(feature = "datagen")]
);
