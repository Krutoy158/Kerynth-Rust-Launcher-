use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use thiserror::Error;
use zip::ZipArchive;

const MAX_MOD_BYTES: u64 = 512 * 1024 * 1024;
const MAX_ARCHIVE_ENTRIES: usize = 20_000;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModInfo {
    pub file_name: String,
    pub sha256: String,
    pub loader_hints: Vec<String>,
    pub size_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstallOutcome {
    Installed(ModInfo),
    AlreadyInstalled(ModInfo),
}

#[derive(Debug, Error)]
pub enum ModError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("only .jar mod files are accepted")]
    NotJar,
    #[error("the selected file name is invalid")]
    InvalidFileName,
    #[error("mod exceeds the 512 MiB safety limit")]
    TooLarge,
    #[error("mod archive is invalid: {0}")]
    InvalidArchive(String),
    #[error("mod archive contains too many entries")]
    TooManyEntries,
    #[error("requested mod is not a regular file in the profile mod directory")]
    UnsafePath,
}

pub fn install_mod<R: Read>(
    mut input: R,
    source_name: &str,
    mods_directory: impl AsRef<Path>,
) -> Result<InstallOutcome, ModError> {
    let safe_stem = sanitized_stem(source_name)?;
    let directory = prepare_directory(mods_directory.as_ref())?;
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temporary_path = directory.join(format!(".krl-{sequence}.part"));

    let write_result = write_and_hash(&mut input, &temporary_path);
    let (digest, size_bytes) = match write_result {
        Ok(result) => result,
        Err(error) => {
            let _ = fs::remove_file(&temporary_path);
            return Err(error);
        }
    };

    let loader_hints = match inspect_archive(&temporary_path) {
        Ok(hints) => hints,
        Err(error) => {
            let _ = fs::remove_file(&temporary_path);
            return Err(error);
        }
    };

    if let Some(existing) = find_duplicate(&directory, &digest)? {
        let _ = fs::remove_file(&temporary_path);
        return Ok(InstallOutcome::AlreadyInstalled(existing));
    }

    let suffix = &digest[..12];
    let file_name = format!("{safe_stem}-{suffix}.jar");
    let target_path = directory.join(&file_name);

    if target_path.exists() {
        let _ = fs::remove_file(&temporary_path);
        let existing = inspect_file(&target_path)?;
        return Ok(InstallOutcome::AlreadyInstalled(existing));
    }

    fs::rename(&temporary_path, &target_path)?;

    Ok(InstallOutcome::Installed(ModInfo {
        file_name,
        sha256: digest,
        loader_hints,
        size_bytes,
    }))
}

pub fn list_mods(mods_directory: impl AsRef<Path>) -> Result<Vec<ModInfo>, ModError> {
    let directory = prepare_directory(mods_directory.as_ref())?;
    let mut mods = Vec::new();

    for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("jar") {
            continue;
        }

        mods.push(inspect_file(&path)?);
    }

    mods.sort_by(|left, right| left.file_name.cmp(&right.file_name));
    Ok(mods)
}

pub fn remove_mod(mods_directory: impl AsRef<Path>, file_name: &str) -> Result<(), ModError> {
    if file_name.is_empty()
        || file_name.contains('/')
        || file_name.contains('\\')
        || Path::new(file_name)
            .file_name()
            .and_then(|value| value.to_str())
            != Some(file_name)
    {
        return Err(ModError::UnsafePath);
    }

    let directory = prepare_directory(mods_directory.as_ref())?;
    let path = directory.join(file_name);
    let metadata = fs::symlink_metadata(&path)?;

    if !metadata.file_type().is_file() {
        return Err(ModError::UnsafePath);
    }

    fs::remove_file(path)?;
    Ok(())
}

fn write_and_hash<R: Read>(
    input: &mut R,
    temporary_path: &Path,
) -> Result<(String, u64), ModError> {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temporary_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut total = 0_u64;

    loop {
        let read = input.read(&mut buffer)?;
        if read == 0 {
            break;
        }

        total = total.checked_add(read as u64).ok_or(ModError::TooLarge)?;
        if total > MAX_MOD_BYTES {
            return Err(ModError::TooLarge);
        }

        hasher.update(&buffer[..read]);
        output.write_all(&buffer[..read])?;
    }

    output.sync_all()?;
    Ok((hex::encode(hasher.finalize()), total))
}

fn inspect_file(path: &Path) -> Result<ModInfo, ModError> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() {
        return Err(ModError::UnsafePath);
    }

    let digest = hash_file(path)?;
    let loader_hints = inspect_archive(path)?;

    Ok(ModInfo {
        file_name: path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or(ModError::InvalidFileName)?
            .to_owned(),
        sha256: digest,
        loader_hints,
        size_bytes: metadata.len(),
    })
}

fn inspect_archive(path: &Path) -> Result<Vec<String>, ModError> {
    let file = File::open(path)?;
    let mut archive =
        ZipArchive::new(file).map_err(|error| ModError::InvalidArchive(error.to_string()))?;

    if archive.len() > MAX_ARCHIVE_ENTRIES {
        return Err(ModError::TooManyEntries);
    }

    let checks = [
        ("fabric.mod.json", "Fabric"),
        ("quilt.mod.json", "Quilt"),
        ("META-INF/mods.toml", "Forge"),
        ("META-INF/neoforge.mods.toml", "NeoForge"),
    ];

    let mut hints = Vec::new();
    for (entry_name, loader) in checks {
        if archive.by_name(entry_name).is_ok() {
            hints.push(loader.to_owned());
        }
    }

    Ok(hints)
}

fn find_duplicate(directory: &Path, digest: &str) -> Result<Option<ModInfo>, ModError> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("jar") {
            continue;
        }

        if hash_file(&path)? == digest {
            return inspect_file(&path).map(Some);
        }
    }

    Ok(None)
}

fn hash_file(path: &Path) -> Result<String, ModError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(hex::encode(hasher.finalize()))
}

fn prepare_directory(path: &Path) -> Result<PathBuf, ModError> {
    fs::create_dir_all(path)?;
    Ok(fs::canonicalize(path)?)
}

fn sanitized_stem(source_name: &str) -> Result<String, ModError> {
    let file_name = Path::new(source_name)
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or(ModError::InvalidFileName)?;

    if !file_name.to_ascii_lowercase().ends_with(".jar") {
        return Err(ModError::NotJar);
    }

    let stem = &file_name[..file_name.len() - 4];
    let mut result = String::with_capacity(stem.len().min(80));

    for character in stem.chars().take(80) {
        if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
            result.push(character);
        } else {
            result.push('_');
        }
    }

    let trimmed = result.trim_matches(|character| character == '_' || character == '-');
    if trimmed.is_empty() {
        return Err(ModError::InvalidFileName);
    }

    Ok(trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{InstallOutcome, ModError, install_mod, remove_mod, sanitized_stem};
    use std::fs;
    use std::io::{Cursor, Write};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};
    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    fn temporary_directory() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("krl-mod-test-{nonce}"))
    }

    fn valid_jar() -> Vec<u8> {
        let cursor = Cursor::new(Vec::new());
        let mut writer = ZipWriter::new(cursor);
        writer
            .start_file("fabric.mod.json", SimpleFileOptions::default())
            .expect("archive entry should be created");
        writer
            .write_all(br#"{"schemaVersion":1,"id":"sample"}"#)
            .expect("metadata should be written");
        writer.finish().expect("archive should finish").into_inner()
    }

    #[test]
    fn sanitizes_source_names() {
        assert_eq!(
            sanitized_stem("../Cool Mod.jar").expect("valid name"),
            "Cool_Mod"
        );
        assert!(matches!(sanitized_stem("file.zip"), Err(ModError::NotJar)));
    }

    #[test]
    fn imports_jar_and_detects_duplicate() {
        let directory = temporary_directory();
        let bytes = valid_jar();

        let first = install_mod(Cursor::new(bytes.clone()), "sample.jar", &directory)
            .expect("first import should succeed");
        let second = install_mod(Cursor::new(bytes), "renamed.jar", &directory)
            .expect("duplicate import should be detected");

        assert!(matches!(first, InstallOutcome::Installed(_)));
        assert!(matches!(second, InstallOutcome::AlreadyInstalled(_)));
        fs::remove_dir_all(directory).expect("test directory should be removed");
    }

    #[test]
    fn rejects_non_archive_input() {
        let directory = temporary_directory();
        let result = install_mod(Cursor::new(b"not a jar".to_vec()), "bad.jar", &directory);
        assert!(matches!(result, Err(ModError::InvalidArchive(_))));
        fs::remove_dir_all(directory).expect("test directory should be removed");
    }

    #[test]
    fn refuses_path_based_removal() {
        let directory = temporary_directory();
        fs::create_dir_all(&directory).expect("test directory should be created");
        assert!(matches!(
            remove_mod(&directory, "../outside.jar"),
            Err(ModError::UnsafePath)
        ));
        fs::remove_dir_all(directory).expect("test directory should be removed");
    }
}
