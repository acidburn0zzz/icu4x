// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use alloc::vec::Vec;
use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;
use litemap::LiteMap;
use tinystr::{TinyStr4, TinyStr8};

#[icu_provider::data_struct(AliasesV1Marker = "locale_canonicalizer/aliases@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
/// This alias data is used for locale canonicalization. Each field defines a
/// mapping from an old identifier to a new identifier, based upon the rules in
/// from <http://unicode.org/reports/tr35/#LocaleId_Canonicalization>. The data
/// is stored in sorted order, allowing for binary search to identify rules to
/// apply. It is broken down into smaller vectors based upon some characteristic
/// of the data, to help avoid unnecessary searches. For example, the `sgn_region`
/// field contains aliases for sign language and region, so that it is not
/// necessary to search the data unless the input is a sign language.
///
/// The algorithm in tr35 is not guaranteed to terminate on data other than what
/// is currently in CLDR. For this reason, it is not a good idea to attempt to add
/// or modify aliases for use in this structure.
pub struct AliasesV1 {
    /// Language data not covered by other rules, normally this will be empty.
    #[zerofrom(clone)]
    pub language: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    /// Language and variant.
    #[zerofrom(clone)]
    pub language_variants: Vec<(LanguageIdentifier, LanguageIdentifier)>,
    /// Sign language and region data.
    #[zerofrom(clone)]
    pub sgn_region: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Two character language codes.
    #[zerofrom(clone)]
    pub language_len2: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Three character language codes.
    #[zerofrom(clone)]
    pub language_len3: Vec<(TinyStr4, LanguageIdentifier)>,
    /// Scripts.
    #[zerofrom(clone)]
    pub script: Vec<(TinyStr4, TinyStr4)>,
    /// Alphabetical region codes.
    #[zerofrom(clone)]
    pub region_alpha: Vec<(TinyStr4, TinyStr4)>,
    /// Numeric region codes.
    #[zerofrom(clone)]
    pub region_num: Vec<(TinyStr4, TinyStr4)>,
    /// Old regions which map to more than one new region.
    #[zerofrom(clone)]
    pub complex_region: Vec<(TinyStr4, Vec<TinyStr4>)>,
    /// Variants.
    #[zerofrom(clone)]
    pub variant: Vec<(TinyStr8, TinyStr8)>,
    /// Subdivisions.
    #[zerofrom(clone)]
    pub subdivision: Vec<(TinyStr8, TinyStr8)>,
}

#[icu_provider::data_struct(LikelySubtagsV1Marker = "locale_canonicalizer/likelysubtags@1")]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
/// This likely subtags data is used for the minimize and maximize operations.
/// Each field defines a mapping from an old identifier to a new identifier,
/// based upon the rules in
/// <https://www.unicode.org/reports/tr35/#Likely_Subtags>.
///
/// The data is stored in sorted order, allowing for binary search to identify
/// rules to apply. It is broken down into smaller vectors based upon the rules
/// defined for the likely subtags maximize algorithm.
///
/// For efficiency, only the relevant part of the LanguageIdentifier is stored
/// for searching. E.g., the `language_script` field is used to store rules for
/// `LanguageIdentifier` that contain a language and a script, but not a region.
/// This also allows for space savings by using a `TinyStr4` rather than a full
/// `LanguageIdentifier`.
pub struct LikelySubtagsV1 {
    /// Language and script.
    #[zerofrom(clone)]
    pub language_script: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier>,
    /// Language and region.
    #[zerofrom(clone)]
    pub language_region: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier>,
    /// Just language.
    #[zerofrom(clone)]
    pub language: LiteMap<TinyStr4, LanguageIdentifier>,
    /// Script and region.
    #[zerofrom(clone)]
    pub script_region: LiteMap<(TinyStr4, TinyStr4), LanguageIdentifier>,
    /// Just script.
    #[zerofrom(clone)]
    pub script: LiteMap<TinyStr4, LanguageIdentifier>,
    /// Just region.
    #[zerofrom(clone)]
    pub region: LiteMap<TinyStr4, LanguageIdentifier>,
    /// Undefined.
    #[zerofrom(clone)]
    pub und: LanguageIdentifier,
}