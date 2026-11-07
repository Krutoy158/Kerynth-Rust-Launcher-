use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

pub const LATEST_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VersionManifest {
    pub latest: LatestVersions,
    pub versions: Vec<VersionEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: Option<String>,
    pub compliance_level: Option<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionChannel {
    Releases,
    Snapshots,
    Legacy,
    All,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("manifest JSON is invalid: {0}")]
    Json(#[from] serde_json::Error),
    #[error("version id is invalid: {0}")]
    InvalidVersionId(String),
    #[error("manifest contains a duplicate version id: {0}")]
    DuplicateVersionId(String),
    #[error("manifest does not contain the latest release entry: {0}")]
    MissingLatestRelease(String),
}

impl VersionManifest {
    pub fn find(&self, id: &str) -> Option<&VersionEntry> {
        self.versions.iter().find(|entry| entry.id == id)
    }

    pub fn latest_release(&self) -> Option<&VersionEntry> {
        self.find(&self.latest.release)
    }

    pub fn latest_snapshot(&self) -> Option<&VersionEntry> {
        self.find(&self.latest.snapshot)
    }

    pub fn filtered(&self, channel: VersionChannel) -> impl Iterator<Item = &VersionEntry> {
        self.versions.iter().filter(move |entry| match channel {
            VersionChannel::Releases => entry.version_type == "release",
            VersionChannel::Snapshots => entry.version_type == "snapshot",
            VersionChannel::Legacy => {
                entry.version_type == "old_beta" || entry.version_type == "old_alpha"
            }
            VersionChannel::All => true,
        })
    }
}

pub fn parse_manifest(raw: &str) -> Result<VersionManifest, ManifestError> {
    let manifest: VersionManifest = serde_json::from_str(raw)?;
    let mut seen = HashSet::with_capacity(manifest.versions.len());

    for entry in &manifest.versions {
        if !is_valid_version_id(&entry.id) {
            return Err(ManifestError::InvalidVersionId(entry.id.clone()));
        }

        if !seen.insert(entry.id.as_str()) {
            return Err(ManifestError::DuplicateVersionId(entry.id.clone()));
        }
    }

    if !manifest
        .versions
        .iter()
        .any(|entry| entry.id == manifest.latest.release)
    {
        return Err(ManifestError::MissingLatestRelease(
            manifest.latest.release.clone(),
        ));
    }

    Ok(manifest)
}

fn is_valid_version_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._-+".contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::{VersionChannel, parse_manifest};

    const FIXTURE: &str = r#"{
        "latest": {"release": "1.21.11", "snapshot": "25w01a"},
        "versions": [
            {
                "id": "1.21.11",
                "type": "release",
                "url": "https://example.invalid/release.json",
                "time": "2026-01-01T00:00:00+00:00",
                "releaseTime": "2026-01-01T00:00:00+00:00",
                "sha1": null,
                "complianceLevel": 1
            },
            {
                "id": "25w01a",
                "type": "snapshot",
                "url": "https://example.invalid/snapshot.json",
                "time": "2025-01-01T00:00:00+00:00",
                "releaseTime": "2025-01-01T00:00:00+00:00",
                "sha1": null,
                "complianceLevel": 1
            },
            {
                "id": "b1.7.3",
                "type": "old_beta",
                "url": "https://example.invalid/beta.json",
                "time": "2011-01-01T00:00:00+00:00",
                "releaseTime": "2011-01-01T00:00:00+00:00",
                "sha1": null,
                "complianceLevel": 0
            }
        ]
    }"#;

    #[test]
    fn parses_and_filters_channels() {
        let manifest = parse_manifest(FIXTURE).expect("fixture should parse");
        assert_eq!(
            manifest.latest_release().map(|entry| entry.id.as_str()),
            Some("1.21.11")
        );
        assert_eq!(manifest.filtered(VersionChannel::Releases).count(), 1);
        assert_eq!(manifest.filtered(VersionChannel::Snapshots).count(), 1);
        assert_eq!(manifest.filtered(VersionChannel::Legacy).count(), 1);
        assert_eq!(manifest.filtered(VersionChannel::All).count(), 3);
    }

    #[test]
    fn rejects_duplicate_ids() {
        let duplicate = FIXTURE.replace(r#""id": "25w01a""#, r#""id": "1.21.11""#);
        assert!(parse_manifest(&duplicate).is_err());
    }

    #[test]
    fn rejects_path_like_version_ids() {
        let unsafe_id = FIXTURE.replace("1.21.11", "../game");
        assert!(parse_manifest(&unsafe_id).is_err());
    }
}
