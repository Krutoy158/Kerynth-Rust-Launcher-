use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "OfflineIdentityData")]
pub struct OfflineIdentity {
    username: String,
    uuid: String,
}

#[derive(Deserialize)]
struct OfflineIdentityData {
    username: String,
    uuid: String,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IdentityError {
    #[error("username must contain 3 to 16 ASCII letters, digits, or underscores")]
    InvalidUsername,
}

impl TryFrom<OfflineIdentityData> for OfflineIdentity {
    type Error = String;

    fn try_from(data: OfflineIdentityData) -> Result<Self, Self::Error> {
        let identity = Self::new(data.username).map_err(|error| error.to_string())?;
        if identity.uuid != data.uuid {
            return Err("offline UUID does not match username".to_owned());
        }
        Ok(identity)
    }
}

impl OfflineIdentity {
    pub fn new(username: impl Into<String>) -> Result<Self, IdentityError> {
        let username = username.into();
        validate_username(&username)?;

        let mut hasher = Md5::new();
        hasher.update(format!("OfflinePlayer:{username}").as_bytes());
        let digest = hasher.finalize();
        let mut bytes = [0_u8; 16];
        bytes.copy_from_slice(&digest);
        bytes[6] = (bytes[6] & 0x0f) | 0x30;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;

        let uuid = format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5],
            bytes[6], bytes[7],
            bytes[8], bytes[9],
            bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
        );

        Ok(Self { username, uuid })
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn uuid(&self) -> &str {
        &self.uuid
    }
}

fn validate_username(username: &str) -> Result<(), IdentityError> {
    let length = username.len();
    if !(3..=16).contains(&length)
        || !username
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(IdentityError::InvalidUsername);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{IdentityError, OfflineIdentity};

    #[test]
    fn generates_vanilla_compatible_notch_offline_uuid() {
        let identity = OfflineIdentity::new("Notch").expect("valid offline username");
        assert_eq!(identity.uuid(), "b50ad385-829d-3141-a216-7e7d7539ba7f");
    }

    #[test]
    fn preserves_username_case_and_uuid_is_case_sensitive() {
        let upper = OfflineIdentity::new("TemaPlayer").expect("valid username");
        let lower = OfflineIdentity::new("temaplayer").expect("valid username");
        assert_ne!(upper.uuid(), lower.uuid());
    }

    #[test]
    fn rejects_invalid_usernames() {
        for username in ["ab", "a username", "bad/name", "тема", "abcdefghijklmnopq"] {
            assert_eq!(
                OfflineIdentity::new(username),
                Err(IdentityError::InvalidUsername)
            );
        }
    }

    #[test]
    fn serializes_and_validates_local_identity() {
        let identity = OfflineIdentity::new("TemaPlayer").expect("valid username");
        let json = serde_json::to_string(&identity).expect("identity should serialize");
        let decoded: OfflineIdentity =
            serde_json::from_str(&json).expect("identity should deserialize");
        assert_eq!(decoded, identity);

        let altered = json.replace(identity.uuid(), "00000000-0000-3000-8000-000000000000");
        assert!(serde_json::from_str::<OfflineIdentity>(&altered).is_err());
    }
}
