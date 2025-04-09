use std::{
    collections::HashSet,
    ffi::OsStr,
    fs::OpenOptions,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use log::{debug, trace};
use zip::ZipArchive;

#[derive(Clone, PartialEq)]
pub struct ClassContainer {
    original_path: PathBuf,
    units: HashSet<String>,
    other_files: HashSet<String>,
    is_jmod: bool,
    // main_class: Option<String>,
}

impl ClassContainer {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        debug!("Analyzing jar file at {path:?}");

        let mut archive = ZipArchive::new(OpenOptions::new().read(true).open(path)?)?;

        let mut units = HashSet::new();
        let mut other_files = HashSet::new();
        let is_jmod = if let Some(ext) = path.extension() {
            ext.to_string_lossy() == "jmod"
        } else {
            false
        };

        for file_number in 0..archive.len() {
            let filename = archive.by_index(file_number)?.name().to_string();

            if (!is_jmod || filename.starts_with("classes/")) && (filename.ends_with(".class")) {
                let local_fullname = (filename
                    .trim_start_matches("classes/")
                    .trim_end_matches(".class"))
                .to_string();

                trace!("Found class file for {local_fullname} at {filename}");

                units.insert(local_fullname);
            } else {
                trace!("Found other file: {filename}");
                other_files.insert(filename);
            }
        }

        Ok(Self {
            original_path: path.to_path_buf(),
            units,
            other_files,
            is_jmod,
        })
    }

    pub fn has_unit(&self, unit_name: &str) -> bool {
        self.units.contains(&unit_name.to_string())
    }

    pub fn read_class_file(&self, unit_name: &str) -> anyhow::Result<Cursor<Vec<u8>>> {
        let mut archive =
            ZipArchive::new(OpenOptions::new().read(true).open(&self.original_path)?)?;

        let mut content = vec![];

        let unit_path = if self.is_jmod {
            format!("classes/{unit_name}.class")
        } else {
            format!("{unit_name}.class")
        };

        archive.by_name(&unit_path)?.read_to_end(&mut content)?;

        Ok(Cursor::new(content))
    }
}
