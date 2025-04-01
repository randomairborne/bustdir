#[cfg(feature = "askama")]
pub mod askama;
#[cfg(feature = "tera")]
pub mod tera;

use std::{fmt::Formatter, path::Path};

use ahash::AHashMap;
use blake3::Hash;
#[cfg(feature = "get_or_random")]
use rand::RngCore;

type DirMap = AHashMap<String, Hash>;

#[derive(Debug, Clone)]
pub struct BustDir {
    map: DirMap,
}

impl BustDir {
    /// Create a new [`BustDir`] with its root at `path`.
    /// # Errors
    /// This function can error if it finds weird characters in a path, or encounters an I/O error.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let mut map = AHashMap::new();
        build_dir_map(&mut map, path, path, "/")?;
        Ok(Self { map })
    }

    /// Get a path from the [`BustDir`], if it exists
    pub fn get(&self, path: &str) -> Option<Hash> {
        self.map.get(path).copied()
    }

    /// Get a path from the [`BustDir`], returning a random hash if no item is found there
    #[cfg(feature = "get_or_random")]
    pub fn get_or_random(&self, path: &str) -> Hash {
        fn rand_hash() -> Hash {
            let mut bytes = [0; 32];
            rand::thread_rng().fill_bytes(&mut bytes);
            Hash::from_bytes(bytes)
        }
        self.get(path).unwrap_or_else(rand_hash)
    }
}

fn build_dir_map(
    map: &mut DirMap,
    base_path: &Path,
    handle_path: &Path,
    prefix: &str,
) -> Result<(), Error> {
    let listing = handle_path.read_dir()?;
    let base_path = base_path.canonicalize()?;
    for item in listing {
        let item = item?;
        let kind = item.file_type()?;
        let path = item.path().canonicalize()?;
        if kind.is_file() {
            let path_str = path
                .strip_prefix(&base_path)?
                .to_str()
                .ok_or(Error::UnstringablePath)?;

            let file = std::fs::read(&path)?;
            let hash = blake3::hash(&file);

            let path = format!("{prefix}{path_str}");
            map.insert(path, hash);
        } else if kind.is_dir() {
            build_dir_map(map, &base_path, path.as_ref(), prefix)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    StripPrefix(std::path::StripPrefixError),
    UnstringablePath,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(value: std::path::StripPrefixError) -> Self {
        Self::StripPrefix(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {e}"),
            Self::StripPrefix(e) => write!(f, "Prefix stripping error: {e}"),
            Self::UnstringablePath => write!(f, "Path could not be stringified!"),
        }
    }
}

impl std::error::Error for Error {}
