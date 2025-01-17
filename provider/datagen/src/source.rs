// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::source::CldrCache;
pub use crate::transform::cldr::source::LocaleSubset as CldrLocaleSubset;
use elsa::sync::FrozenMap;
pub use icu_codepointtrie::TrieType as IcuTrieType;
use icu_provider::DataError;
use std::any::Any;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

/// Bag of options for datagen source data.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct SourceData {
    cldr_paths: Option<Arc<CldrCache>>,
    icuexport_paths: Option<Arc<TomlCache>>,
    segmenter_paths: Arc<TomlCache>,
    trie_type: IcuTrieType,
}

impl Default for SourceData {
    fn default() -> Self {
        Self {
            cldr_paths: None,
            icuexport_paths: None,
            segmenter_paths: Arc::new(TomlCache::new(
                AbstractFs::new(PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("data"))
                    .expect("valid dir"),
            )),
            trie_type: IcuTrieType::Small,
        }
    }
}

impl SourceData {
    /// Adds CLDR data to this `DataSource`. The root should point to a local
    /// `cldr-{version}-json-{full, modern}.zip` directory or ZIP file (see
    /// [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    pub fn with_cldr(
        self,
        root: PathBuf,
        locale_subset: CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        Ok(Self {
            cldr_paths: Some(Arc::new(CldrCache::new(
                AbstractFs::new(root)?,
                locale_subset,
            ))),
            ..self
        })
    }

    /// Adds ICU export data to this `DataSource`. The path should point to a local
    /// `icuexportdata_uprops_full.zip` directory or ZIP file (see [GitHub releases](
    /// https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport(self, root: PathBuf, trie_type: IcuTrieType) -> Result<Self, DataError> {
        Ok(Self {
            icuexport_paths: Some(Arc::new(TomlCache::new(AbstractFs::new(root)?))),
            trie_type,
            ..self
        })
    }

    /// Adds CLDR data to this `DataSource`. The data will be downloaded from GitHub
    /// using the given tag (see [GitHub releases](https://github.com/unicode-org/cldr-json/releases)).
    pub fn with_cldr_for_tag(
        self,
        tag: &str,
        locale_subset: CldrLocaleSubset,
    ) -> Result<Self, DataError> {
        self.with_cldr(
            cached_path::CacheBuilder::new().freshness_lifetime(u64::MAX).build().and_then(|cache| cache
                .cached_path(
                    &format!(
                        "https://github.com/unicode-org/cldr-json/releases/download/{}/cldr-{}-json-{}.zip",
                        tag, tag, locale_subset))).map_err(|e| DataError::custom("Download").with_display_context(&e))?,
            locale_subset
            )
    }

    /// Adds ICU export data to this `DataSource`. The data will be downloaded from GitHub
    /// using the given tag. (see [GitHub releases](https://github.com/unicode-org/icu/releases)).
    pub fn with_icuexport_for_tag(
        self,
        tag: &str,
        trie_type: IcuTrieType,
    ) -> Result<Self, DataError> {
        self.with_icuexport(
            cached_path::CacheBuilder::new().freshness_lifetime(u64::MAX).build().and_then(|cache| cache
                .cached_path(
                    &format!("https://github.com/unicode-org/icu/releases/download/{}/icuexportdata_uprops_full.zip", tag)
            )).map_err(|e| DataError::custom("Download").with_display_context(&e))?,
            trie_type)
    }

    /// Creates a [`SourceData`] object with the latest data from GitHub.
    pub fn latest(
        locale_subset: CldrLocaleSubset,
        trie_type: IcuTrieType,
    ) -> Result<Self, DataError> {
        // TODO query GitHub for the latest tags.
        Self::default()
            .with_cldr_for_tag("41.0.0", locale_subset)?
            .with_icuexport_for_tag("release-71-1", trie_type)
    }

    #[cfg(test)]
    /// Create `SourceData` pointing to test data.
    pub(crate) fn for_test() -> Self {
        Self::default()
            .with_cldr(
                icu_testdata::paths::cldr_json_root(),
                CldrLocaleSubset::Full,
            )
            .expect("testdata is valid")
            .with_icuexport(
                icu_testdata::paths::icuexport_toml_root(),
                IcuTrieType::Small,
            )
            .expect("testdata is valid")
    }

    /// Paths to CLDR source data.
    pub(crate) fn cldr(&self) -> Result<&CldrCache, DataError> {
        self.cldr_paths
            .as_deref()
            .ok_or(crate::error::MISSING_CLDR_ERROR)
    }

    /// Path to Unicode Properties source data.
    pub(crate) fn icuexport(&self) -> Result<&TomlCache, DataError> {
        self.icuexport_paths
            .as_deref()
            .ok_or(crate::error::MISSING_ICUEXPORT_ERROR)
    }

    // Uprops paths depend on the trie type, this saves some boilerplate at the call sites.
    pub(crate) fn read_and_parse_uprops<S>(&self, name: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        self.icuexport()?.read_and_parse_toml(&format!(
            "icuexportdata_uprops_full/{}/{}.toml",
            match self.trie_type {
                icu_codepointtrie::TrieType::Fast => "fast",
                icu_codepointtrie::TrieType::Small => "small",
            },
            name
        ))
    }

    /// Path to segmenter data.
    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub(crate) fn segmenter(&self) -> Result<&TomlCache, DataError> {
        Ok(&self.segmenter_paths)
    }

    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub(crate) fn trie_type(&self) -> IcuTrieType {
        self.trie_type
    }
}

pub(crate) struct TomlCache {
    root: AbstractFs,
    cache: Arc<FrozenMap<String, Box<dyn Any + Send + Sync>>>,
}

impl Debug for TomlCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TomlCache")
            .field("root", &self.root)
            // skip formatting the cache
            .finish()
    }
}

impl TomlCache {
    pub fn new(root: AbstractFs) -> Self {
        Self {
            root,
            cache: Arc::new(FrozenMap::new()),
        }
    }

    pub fn read_and_parse_toml<S>(&self, path: &str) -> Result<&S, DataError>
    where
        for<'de> S: serde::Deserialize<'de> + 'static + Send + Sync,
    {
        match self.cache.get(path) {
            Some(x) => x,
            None => {
                let file = self.root.read_to_buf(path)?;
                let file: S = toml::from_slice(&file).map_err(|e| {
                    crate::error::data_error_from_toml(e).with_display_context(&path)
                })?;
                self.cache.insert(path.to_string(), Box::new(file))
            }
        }
        .downcast_ref::<S>()
        .ok_or_else(|| DataError::custom("Uprops TOML error").with_type_context::<S>())
    }

    #[cfg_attr(not(feature = "experimental"), allow(dead_code))]
    pub fn list(&self, path: &str) -> Result<impl Iterator<Item = PathBuf>, DataError> {
        self.root.list(path).map(Vec::into_iter)
    }
}

#[derive(Debug)]
pub(crate) enum AbstractFs {
    Fs(PathBuf),
    Zip(PathBuf),
}

impl AbstractFs {
    pub fn new<P: AsRef<Path>>(root: P) -> Result<Self, DataError> {
        if std::fs::metadata(root.as_ref())?.is_dir() {
            Ok(Self::Fs(root.as_ref().to_path_buf()))
        } else {
            zip::ZipArchive::new(File::open(&root)?)
                .map_err(|e| DataError::custom("Zip").with_display_context(&e))?;
            Ok(Self::Zip(root.as_ref().into()))
        }
    }

    pub fn read_to_buf(&self, path: &str) -> Result<Vec<u8>, DataError> {
        match self {
            Self::Fs(root) => {
                log::trace!("Reading: {}/{}", root.display(), path);
                std::fs::read(&root.join(path))
                    .map_err(|e| DataError::from(e).with_path_context(&root.join(path)))
            }
            Self::Zip(root) => {
                log::trace!("Reading: {}/{}", root.display(), path);
                let mut buf = Vec::new();
                zip::ZipArchive::new(File::open(root)?)
                    .expect("validated in constructor")
                    .by_name(path)
                    .map_err(|e| {
                        DataError::custom("Zip")
                            .with_display_context(&e)
                            .with_display_context(path)
                    })?
                    .read_to_end(&mut buf)?;
                Ok(buf)
            }
        }
    }

    pub fn list(&self, path: &str) -> Result<Vec<PathBuf>, DataError> {
        Ok(match self {
            Self::Fs(root) => std::fs::read_dir(&root.join(path))?
                .map(|e| -> Result<_, DataError> { Ok(PathBuf::from(e?.file_name())) })
                .collect::<Result<_, DataError>>()?,
            Self::Zip(root) => zip::ZipArchive::new(File::open(root)?)
                .expect("validated in constructor")
                .file_names()
                .filter_map(|p| p.strip_prefix('/'))
                .filter_map(|p| p.strip_prefix(path))
                .filter_map(|suffix| suffix.split('/').next())
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
                .collect(),
        })
    }
}
