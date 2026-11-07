# KRL persistent game storage

## Goal

Game data must survive uninstalling and reinstalling KRL. The launcher APK and private application settings are not the home for downloaded game versions.

KRL uses a user-visible shared directory named KRL. On Android, the intended location is a folder named KRL inside a user-selected Documents directory, accessed through the Storage Access Framework (SAF).

## Directory layout

The storage root contains:

- krl-storage.json — identifies the directory as KRL storage and records its format version.
- launcher/ — launcher preferences and non-secret metadata.
- metadata/ — indexes and manifests maintained by KRL.
- versions/ — installed Minecraft version metadata and version-specific client files.
- libraries/ — shared game libraries reused by every instance.
- assets/indexes/ and assets/objects/ — shared game assets.
- runtimes/ — installed compatible game runtimes.
- instances/<instance-id>/ — separate game instances.
- downloads/ — resumable download state and temporary files.
- cache/ — data that can be safely regenerated.

Each instance has its own mods/, config/, resourcepacks/, shaderpacks/, saves/, screenshots/, logs/, and crash-reports/ directories. This keeps mods and worlds separate between instances while shared libraries and assets are downloaded only once.

## Reinstall behavior

1. On first setup, KRL asks the user to choose a shared Documents directory through the Android system picker.
2. KRL creates the KRL directory and writes krl-storage.json.
3. Version files, libraries, assets, runtimes, mods, and worlds are stored there rather than in app-specific storage.
4. After KRL is uninstalled, shared documents remain on the device. App-specific files do not.
5. After reinstalling, Android access to the old document tree must be requested again. KRL asks the user to select the existing KRL directory.
6. KRL opens the folder, validates krl-storage.json, indexes existing versions and instances, and reuses verified files. It must not download a version again when its required files are already present and valid.

The user may instead choose a different storage directory. KRL must never delete or overwrite an unrecognized folder just because it is named KRL.

## Android implementation requirements

- Use ACTION_OPEN_DOCUMENT_TREE and take the returned persistable read/write URI permission while the grant is available.
- Store the selected tree URI for normal launches, but do not assume the URI grant survives app uninstall.
- The selected directory must be outside app-specific storage and outside the media store to survive uninstall.
- Android 11 and newer restrict selecting certain protected roots and directories. Documents is the intended default parent, not the storage-volume root or Download.
- SAF document URIs are not ordinary filesystem paths. The Android implementation must use a Rust SAF/document-provider adapter; it must not turn a content URI into a fake PathBuf.
- Do not request broad all-files access solely to avoid the system folder picker.
- Validate hashes and required files before reusing a download. A folder existing is not proof that every game file is complete or intact.

## Current implementation status

The krl-storage crate currently provides and tests the filesystem layout, KRL marker, root validation, instance directory creation, and safe identifier handling. It does not yet contain the Android SAF adapter, folder-picker flow, version indexing, or download verification integration. Those remain required before the reinstall flow can be called complete.
