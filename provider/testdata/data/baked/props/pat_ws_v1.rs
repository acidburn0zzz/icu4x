// @generated
type DataStruct = & 'static < :: icu_properties :: provider :: PatternWhiteSpaceV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct = &::icu_properties::provider::UnicodePropertyV1::InversionList(unsafe {
    #[allow(unused_unsafe)]
    ::icu_uniset::UnicodeSet::from_parts_unchecked(
        unsafe {
            ::zerovec::ZeroVec::from_bytes_unchecked(&[
                9u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 33u8, 0u8, 0u8, 0u8,
                133u8, 0u8, 0u8, 0u8, 134u8, 0u8, 0u8, 0u8, 14u8, 32u8, 0u8, 0u8, 16u8, 32u8, 0u8,
                0u8, 40u8, 32u8, 0u8, 0u8, 42u8, 32u8, 0u8, 0u8,
            ])
        },
        11usize,
    )
});
