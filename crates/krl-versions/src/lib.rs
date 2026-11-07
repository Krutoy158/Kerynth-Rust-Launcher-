mod manifest;

pub use manifest::{
    LATEST_MANIFEST_URL, ManifestError, VersionChannel, VersionEntry, VersionManifest,
    parse_manifest,
};
