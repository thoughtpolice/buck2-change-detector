/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Context;

pub fn file_writer(file_path: &PathBuf) -> anyhow::Result<impl Write> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .with_context(|| format!("Unable to open file `{}` for writing", file_path.display()))?;
    Ok(BufWriter::new(file))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::path::PathBuf;

    use tempfile::TempDir;

    use crate::file_writer::file_writer;

    static DATA: &str = "Artifact data";

    #[test]
    pub fn test_write_success() {
        let out_dir = TempDir::new().unwrap();
        let out_path = out_dir.path().join("test_artifact.json");

        file_writer(&out_path)
            .unwrap()
            .write_all(DATA.as_bytes())
            .unwrap();

        let written = fs::read_to_string(&out_path).unwrap();
        assert_eq!(written, DATA);
    }

    #[test]
    pub fn test_write_error() {
        assert!(file_writer(&PathBuf::from("/invalid/file/path")).is_err());
    }
}
