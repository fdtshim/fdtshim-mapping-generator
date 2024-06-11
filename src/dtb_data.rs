/* SPDX-License-Identifier: GPL-3.0-only */

use flat_device_tree::Fdt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Simplified overview of a kernel-flavoured dtb file.
#[derive(Debug, Eq, Ord, PartialOrd)]
pub struct DtbData {
    /// Path of the dtb filename, relative to the kernel-flavoured dtbs folder.
    /// This is the path to the file that will be loaded by fdtshim.
    pub path: String,
    /// The model string.
    /// Saved to the produced dts.
    pub model: String,
    /// Components of the compatible field.
    pub compatibles: Vec<String>,
}

impl DtbData {
    pub fn new(file_path: PathBuf, prefix: &Path) -> Self {
        let path: String = file_path
            .strip_prefix(prefix)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let data = fs::read(file_path).unwrap();
        let dtb = Fdt::new(data.as_slice()).unwrap();
        let root = dtb.root().unwrap();
        let model = root.model().to_string();
        let compatibles: Vec<String> = root
            .compatible()
            .all()
            .map(std::string::ToString::to_string)
            .collect();

        Self {
            path,
            model,
            compatibles,
        }
    }

    /// Returns the main compatible from the compatibles list.
    /// This is used to drop duplicated compatible devices.
    pub fn compatible(&self) -> &String {
        self.compatibles.first().unwrap()
    }

    /// Format the compatibles list as a valid dts value.
    pub fn compatibles_source(&self) -> String {
        self.compatibles
            .iter()
            // Borrow the pretty-printed format of a string.
            .map(|part| format!("{part:?}"))
            .collect::<Vec<String>>()
            .join(", ")
    }

    /// Produces a valid node name from the file path.
    ///
    ///  - The extension is removed.
    ///  - The first slash is replaced with an `@`.
    ///  - Any further slashes are replaced with `_`.
    ///
    /// NOTE: The node names should not be relied on. It is an opaque arbitrary unique identifier.
    pub fn node_name(&self) -> String {
        self.path
            .replacen('/', "@", 1)
            .replace('/', "_")
            .replacen(".dtb", "", 1)
    }
}

impl PartialEq for DtbData {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
