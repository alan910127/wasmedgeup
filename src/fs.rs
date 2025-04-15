use std::path::{Path, PathBuf};

use tokio::fs;
use walkdir::WalkDir;

pub async fn copy_tree(from_dir: &Path, to_dir: &Path) {
    let num_components = from_dir.components().count();

    for entry in WalkDir::new(from_dir).into_iter().filter_map(|e| e.ok()) {
        tracing::trace!(entry = %entry.path().display(), "Copying entry");
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }

        // Calculate the target location based on from_dir, to_dir, and entry
        // by first calculate the path of entry relative to from_dir, and then append it to to_dir
        //
        // # Example
        // from_dir = '/from/path
        // entry = '/from/path/foo/bar/something.txt'
        // to_dir = '/to/path'
        // => num_components = 3 ([RootDir, "from", "path"])
        // => chained = [RootDir, "to", "path"].chain(["foo", "bar", "something.txt"])
        // => target_loc = "/to/path/foo/bar/something.txt"
        let target_loc = to_dir
            .components()
            .chain(entry.path().components().skip(num_components))
            .collect::<PathBuf>();

        let Some(parent) = target_loc.parent() else {
            tracing::warn!(location = %target_loc.display(), "Missing parent for target location");
            continue;
        };
        if let Err(e) = fs::create_dir_all(parent).await {
            tracing::warn!(error = %e, directories = %parent.display(), "Failed to create directories");
            continue;
        };

        if let Err(e) = fs::copy(entry.path(), &target_loc).await {
            tracing::warn!(
                error = %e,
                entry = %entry.path().display(),
                target_loc = %target_loc.display(),
                "Failed to copy file to target location",
            );
        };
    }
}
