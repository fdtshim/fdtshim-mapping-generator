/* SPDX-License-Identifier: GPL-3.0-only */

use flat_device_tree::Fdt;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Simplified overview of a kernel-flavoured dtb file.
#[derive(Debug)]
pub struct DtbData {
    /// Relative path of the dtb filename.
    pub path: String,
    /// The model string.
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
        let compatibles: Vec<String> = root.compatible().all().map(std::string::ToString::to_string).collect();

        Self { path, model, compatibles }
    }

    /// Returns the main compatible from the compatibles list.
    pub fn compatible(&self) -> &String {
        self.compatibles.first().unwrap()
    }

    /// Produces a valid node name from the file path.
    /// 
    ///  - The extension is removed.
    ///  - The first slash is replaced with an `@`.
    ///  - Any further slashes are replaced with `_`.
    ///
    /// NOTE: The node names should not be relied on. It is an opaque arbitrary unique identifier.
    pub fn node_name(&self) -> String {
        self.path.replacen('/', "@", 1).replace('/', "_").replacen(".dtb", "", 1)
    }
}

impl PartialEq for DtbData {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
impl Eq for DtbData {}

impl PartialOrd for DtbData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DtbData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}
