use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LoaderKind {
    Vanilla,
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameProfile {
    pub id: String,
    pub name: String,
    pub game_version: String,
    pub loader: LoaderKind,
    pub memory_limit_mb: u32,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ProfileError {
    #[error("profile id is invalid")]
    InvalidId,
    #[error("profile name is empty or too long")]
    InvalidName,
    #[error("game version id is invalid")]
    InvalidGameVersion,
    #[error("memory limit must be between 256 and 16384 MiB")]
    InvalidMemoryLimit,
}

impl GameProfile {
    pub fn validate(&self) -> Result<(), ProfileError> {
        if self.id.is_empty()
            || self.id.len() > 64
            || !self
                .id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-_".contains(&byte))
        {
            return Err(ProfileError::InvalidId);
        }

        let name = self.name.trim();
        if name.is_empty() || name.len() > 80 {
            return Err(ProfileError::InvalidName);
        }

        if self.game_version.is_empty()
            || self.game_version.len() > 64
            || !self
                .game_version
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._-+".contains(&byte))
        {
            return Err(ProfileError::InvalidGameVersion);
        }

        if !(256..=16_384).contains(&self.memory_limit_mb) {
            return Err(ProfileError::InvalidMemoryLimit);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{GameProfile, LoaderKind, ProfileError};

    fn profile() -> GameProfile {
        GameProfile {
            id: "survival-main".to_owned(),
            name: "Survival".to_owned(),
            game_version: "1.21.11".to_owned(),
            loader: LoaderKind::Fabric,
            memory_limit_mb: 2048,
        }
    }

    #[test]
    fn accepts_valid_profile() {
        assert!(profile().validate().is_ok());
    }

    #[test]
    fn rejects_path_traversal_in_profile_id() {
        let mut value = profile();
        value.id = "../outside".to_owned();
        assert_eq!(value.validate(), Err(ProfileError::InvalidId));
    }

    #[test]
    fn rejects_unreasonable_memory_limit() {
        let mut value = profile();
        value.memory_limit_mb = 32_768;
        assert_eq!(value.validate(), Err(ProfileError::InvalidMemoryLimit));
    }
}
