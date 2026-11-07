use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VmRuntime {
    pub library_path: PathBuf,
    pub major_version: u16,
    pub abi: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchPlan {
    pub runtime: VmRuntime,
    pub main_class: String,
    pub class_path: Vec<PathBuf>,
    pub game_directory: PathBuf,
    pub runtime_arguments: Vec<String>,
    pub game_arguments: Vec<String>,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RuntimeError {
    #[error("runtime library does not exist or is not a regular file")]
    MissingRuntime,
    #[error("runtime major version {actual} does not meet required version {required}")]
    IncompatibleVersion { required: u16, actual: u16 },
    #[error("runtime ABI is empty")]
    MissingAbi,
    #[error("main class is empty")]
    MissingMainClass,
    #[error("game directory does not exist")]
    MissingGameDirectory,
    #[error("classpath entry does not exist: {0}")]
    MissingClassPathEntry(PathBuf),
}

impl VmRuntime {
    pub fn validate(&self, required_major: u16) -> Result<(), RuntimeError> {
        if self.major_version < required_major {
            return Err(RuntimeError::IncompatibleVersion {
                required: required_major,
                actual: self.major_version,
            });
        }

        if !self.library_path.is_file() {
            return Err(RuntimeError::MissingRuntime);
        }

        if self.abi.trim().is_empty() {
            return Err(RuntimeError::MissingAbi);
        }

        Ok(())
    }
}

impl LaunchPlan {
    pub fn validate(&self, required_major: u16) -> Result<(), RuntimeError> {
        self.runtime.validate(required_major)?;

        if self.main_class.trim().is_empty() {
            return Err(RuntimeError::MissingMainClass);
        }

        if !self.game_directory.is_dir() {
            return Err(RuntimeError::MissingGameDirectory);
        }

        for entry in &self.class_path {
            if !entry.is_file() {
                return Err(RuntimeError::MissingClassPathEntry(entry.clone()));
            }
        }

        Ok(())
    }

    pub fn game_directory(&self) -> &Path {
        &self.game_directory
    }
}

#[cfg(test)]
mod tests {
    use super::{RuntimeError, VmRuntime};
    use std::path::PathBuf;

    #[test]
    fn rejects_missing_runtime_library() {
        let runtime = VmRuntime {
            library_path: PathBuf::from("missing-runtime.so"),
            major_version: 21,
            abi: "arm64-v8a".to_owned(),
        };

        assert_eq!(runtime.validate(21), Err(RuntimeError::MissingRuntime));
    }

    #[test]
    fn checks_runtime_version_before_library_lookup() {
        let runtime = VmRuntime {
            library_path: PathBuf::from("missing-runtime.so"),
            major_version: 17,
            abi: "arm64-v8a".to_owned(),
        };

        assert_eq!(
            runtime.validate(21),
            Err(RuntimeError::IncompatibleVersion {
                required: 21,
                actual: 17,
            })
        );
    }
}
