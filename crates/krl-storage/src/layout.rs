use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub const DATA_DIRECTORY_NAME: &str = "KRL";
pub const STORAGE_MANIFEST_FILE: &str = "krl-storage.json";

const STORAGE_FORMAT_VERSION: u32 = 1;
const APPLICATION_NAME: &str = "Kerynth Rust Launcher";

const ROOT_DIRECTORIES: &[&str] = &[
    "launcher",
    "metadata",
    "versions",
    "libraries",
    "assets",
    "assets/indexes",
    "assets/objects",
    "runtimes",
    "instances",
    "downloads",
    "cache",
];

const INSTANCE_DIRECTORIES: &[&str] = &[
    "mods",
    "config",
    "resourcepacks",
    "shaderpacks",
    "saves",
    "screenshots",
    "logs",
    "crash-reports",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorageManifest {
    pub format_version: u32,
    pub application: String,
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("storage path is not a real directory: {0}")]
    InvalidDirectory(PathBuf),
    #[error("directory is not an initialized KRL storage root: {0}")]
    UnrecognizedDirectory(PathBuf),
    #[error("storage metadata belongs to a different application or is invalid")]
    InvalidMetadata,
    #[error("storage format version {0} is not supported")]
    UnsupportedFormat(u32),
    #[error("storage identifier is invalid")]
    InvalidIdentifier,
    #[error("storage metadata serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageLayout {
    root: PathBuf,
}

impl StorageLayout {
    pub fn initialize(parent_directory: impl AsRef<Path>) -> Result<Self, StorageError> {
        let parent = parent_directory.as_ref();
        ensure_directory(parent)?;

        let root = parent.join(DATA_DIRECTORY_NAME);

        match fs::symlink_metadata(&root) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.is_dir() {
                    return Err(StorageError::InvalidDirectory(root));
                }

                let manifest_path = root.join(STORAGE_MANIFEST_FILE);
                if manifest_path.exists() {
                    return Self::open(root);
                }

                let mut entries =
                    fs::read_dir(&root).map_err(|source| io_error(&root, source))?;
                if entries.next().is_some() {
                    return Err(StorageError::UnrecognizedDirectory(root));
                }
            }
            Err(source) if source.kind() == io::ErrorKind::NotFound => {
                fs::create_dir(&root).map_err(|source| io_error(&root, source))?;
            }
            Err(source) => return Err(io_error(&root, source)),
        }

        let canonical_root =
            fs::canonicalize(&root).map_err(|source| io_error(&root, source))?;
        let layout = Self {
            root: canonical_root,
        };

        layout.write_manifest()?;
        layout.ensure_layout()?;

        Ok(layout)
    }

    pub fn open(root_directory: impl AsRef<Path>) -> Result<Self, StorageError> {
        let root = root_directory.as_ref();
        ensure_directory(root)?;

        let manifest_path = root.join(STORAGE_MANIFEST_FILE);
        let metadata = fs::symlink_metadata(&manifest_path)
            .map_err(|source| io_error(&manifest_path, source))?;

        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(StorageError::InvalidMetadata);
        }

        let bytes =
            fs::read(&manifest_path).map_err(|source| io_error(&manifest_path, source))?;
        let manifest: StorageManifest = serde_json::from_slice(&bytes)?;

        if manifest.application != APPLICATION_NAME {
            return Err(StorageError::InvalidMetadata);
        }

        if manifest.format_version != STORAGE_FORMAT_VERSION {
            return Err(StorageError::UnsupportedFormat(manifest.format_version));
        }

        let canonical_root = fs::canonicalize(root).map_err(|source| io_error(root, source))?;
        let layout = Self {
            root: canonical_root,
        };

        layout.ensure_layout()?;

        Ok(layout)
    }

    pub fn find_existing(
        parent_directory: impl AsRef<Path>,
    ) -> Result<Option<Self>, StorageError> {
        let root = parent_directory.as_ref().join(DATA_DIRECTORY_NAME);

        if !root.exists() {
            return Ok(None);
        }

        let manifest_path = root.join(STORAGE_MANIFEST_FILE);
        if !manifest_path.exists() {
            return Ok(None);
        }

        Self::open(root).map(Some)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn launcher_directory(&self) -> PathBuf {
        self.root.join("launcher")
    }

    pub fn metadata_directory(&self) -> PathBuf {
        self.root.join("metadata")
    }

    pub fn versions_directory(&self) -> PathBuf {
        self.root.join("versions")
    }

    pub fn version_directory(&self, version_id: &str) -> Result<PathBuf, StorageError> {
        validate_identifier(version_id)?;
        Ok(self.versions_directory().join(version_id))
    }

    pub fn libraries_directory(&self) -> PathBuf {
        self.root.join("libraries")
    }

    pub fn assets_directory(&self) -> PathBuf {
        self.root.join("assets")
    }

    pub fn runtimes_directory(&self) -> PathBuf {
        self.root.join("runtimes")
    }

    pub fn runtime_directory(&self, runtime_id: &str) -> Result<PathBuf, StorageError> {
        validate_identifier(runtime_id)?;
        Ok(self.runtimes_directory().join(runtime_id))
    }

    pub fn instances_directory(&self) -> PathBuf {
        self.root.join("instances")
    }

    pub fn instance_directory(&self, instance_id: &str) -> Result<PathBuf, StorageError> {
        validate_identifier(instance_id)?;
        Ok(self.instances_directory().join(instance_id))
    }

    pub fn instance_mods_directory(
        &self,
        instance_id: &str,
    ) -> Result<PathBuf, StorageError> {
        Ok(self.instance_directory(instance_id)?.join("mods"))
    }

    pub fn instance_saves_directory(
        &self,
        instance_id: &str,
    ) -> Result<PathBuf, StorageError> {
        Ok(self.instance_directory(instance_id)?.join("saves"))
    }

    pub fn downloads_directory(&self) -> PathBuf {
        self.root.join("downloads")
    }

    pub fn cache_directory(&self) -> PathBuf {
        self.root.join("cache")
    }

    pub fn ensure_instance(&self, instance_id: &str) -> Result<PathBuf, StorageError> {
        let directory = self.instance_directory(instance_id)?;
        ensure_directory(&directory)?;

        for child in INSTANCE_DIRECTORIES {
            ensure_directory(&directory.join(child))?;
        }

        Ok(directory)
    }

    fn ensure_layout(&self) -> Result<(), StorageError> {
        for relative in ROOT_DIRECTORIES {
            ensure_directory(&self.root.join(relative))?;
        }

        Ok(())
    }

    fn write_manifest(&self) -> Result<(), StorageError> {
        let manifest = StorageManifest {
            format_version: STORAGE_FORMAT_VERSION,
            application: APPLICATION_NAME.to_owned(),
        };
        let contents = serde_json::to_vec_pretty(&manifest)?;
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let temporary_path = self.root.join(format!(
            ".krl-storage-{}-{nonce}.tmp",
            std::process::id()
        ));
        let manifest_path = self.root.join(STORAGE_MANIFEST_FILE);

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
            .map_err(|source| io_error(&temporary_path, source))?;

        let write_result = file
            .write_all(&contents)
            .and_then(|()| file.sync_all());
        drop(file);

        if let Err(source) = write_result {
            let _ = fs::remove_file(&temporary_path);
            return Err(io_error(&temporary_path, source));
        }

        fs::rename(&temporary_path, &manifest_path)
            .map_err(|source| io_error(&manifest_path, source))?;

        Ok(())
    }
}

fn ensure_directory(path: &Path) -> Result<(), StorageError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(StorageError::InvalidDirectory(path.to_path_buf()));
            }
            Ok(())
        }
        Err(source) if source.kind() == io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|source| io_error(path, source))
        }
        Err(source) => Err(io_error(path, source)),
    }
}

fn validate_identifier(value: &str) -> Result<(), StorageError> {
    if value.is_empty()
        || value.len() > 128
        || value == "."
        || value == ".."
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-+".contains(&byte))
    {
        return Err(StorageError::InvalidIdentifier);
    }

    Ok(())
}

fn io_error(path: &Path, source: io::Error) -> StorageError {
    StorageError::Io {
        path: path.to_path_buf(),
        source,
    }
}

#[cfg(test)]
mod tests {
    use super::{DATA_DIRECTORY_NAME, StorageError, StorageLayout};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_directory() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be valid")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "krl-storage-test-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory should be created");
        path
    }

    #[test]
    fn initializes_shared_layout_and_instance_directories() {
        let parent = temporary_directory();
        let layout = StorageLayout::initialize(&parent).expect("storage should initialize");

        assert_eq!(
            layout.root().file_name().and_then(|value| value.to_str()),
            Some(DATA_DIRECTORY_NAME)
        );
        assert!(layout.root().join("krl-storage.json").is_file());
        assert!(layout.versions_directory().is_dir());
        assert!(layout.libraries_directory().is_dir());
        assert!(layout.assets_directory().join("objects").is_dir());
        assert!(layout.runtimes_directory().is_dir());

        let instance = layout
            .ensure_instance("survival-main")
            .expect("instance should be created");
        assert!(instance.join("mods").is_dir());
        assert!(instance.join("saves").is_dir());
        assert!(instance.join("config").is_dir());

        fs::remove_dir_all(parent).expect("test directory should be removed");
    }

    #[test]
    fn reopens_existing_storage_without_replacing_downloads() {
        let parent = temporary_directory();
        let first = StorageLayout::initialize(&parent).expect("storage should initialize");
        let version = first
            .version_directory("1.21.11")
            .expect("version identifier should be valid");
        fs::create_dir_all(&version).expect("version directory should be created");
        fs::write(version.join("client.jar"), b"existing-game-data")
            .expect("game file should be written");

        let root = first.root().to_path_buf();
        drop(first);

        let second = StorageLayout::open(&root).expect("storage should reopen");
        let client = second
            .version_directory("1.21.11")
            .expect("version identifier should be valid")
            .join("client.jar");

        assert_eq!(
            fs::read(client).expect("existing game data should remain"),
            b"existing-game-data"
        );

        fs::remove_dir_all(parent).expect("test directory should be removed");
    }

    #[test]
    fn finds_existing_storage_root() {
        let parent = temporary_directory();
        StorageLayout::initialize(&parent).expect("storage should initialize");

        let existing =
            StorageLayout::find_existing(&parent).expect("storage scan should succeed");
        assert!(existing.is_some());

        fs::remove_dir_all(parent).expect("test directory should be removed");
    }

    #[test]
    fn refuses_to_adopt_an_unrecognized_non_empty_directory() {
        let parent = temporary_directory();
        let root = parent.join(DATA_DIRECTORY_NAME);
        fs::create_dir(&root).expect("root should be created");
        fs::write(root.join("personal.txt"), b"keep").expect("file should be written");

        assert!(matches!(
            StorageLayout::initialize(&parent),
            Err(StorageError::UnrecognizedDirectory(_))
        ));
        assert_eq!(
            fs::read(root.join("personal.txt")).expect("personal file should remain"),
            b"keep"
        );

        fs::remove_dir_all(parent).expect("test directory should be removed");
    }

    #[test]
    fn rejects_path_traversal_identifiers() {
        let parent = temporary_directory();
        let layout = StorageLayout::initialize(&parent).expect("storage should initialize");

        assert!(matches!(
            layout.instance_directory("../outside"),
            Err(StorageError::InvalidIdentifier)
        ));
        assert!(matches!(
            layout.version_directory(".."),
            Err(StorageError::InvalidIdentifier)
        ));

        fs::remove_dir_all(parent).expect("test directory should be removed");
    }
}
