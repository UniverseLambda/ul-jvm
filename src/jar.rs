use std::{
    collections::HashSet,
    fs::OpenOptions,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use log::debug;
use zip::ZipArchive;

#[derive(Clone, PartialEq)]
pub struct JarFile {
    original_path: PathBuf,
    units: HashSet<String>,
    other_files: HashSet<String>,
    // main_class: Option<String>,
}

impl JarFile {
    pub fn new(path: &Path) -> anyhow::Result<Self> {
        debug!("Analyzing jar file at {path:?}");

        let mut archive = ZipArchive::new(OpenOptions::new().read(true).open(path)?)?;

        let mut units = HashSet::new();
        let mut other_files = HashSet::new();

        for file_number in 0..archive.len() {
            let filename = archive.by_index(file_number)?.name().to_string();

            if filename.ends_with(".class") || filename.ends_with(".CLASS") {
                debug!("Found class file: {filename}");
                units.insert((&filename[..filename.len() - 6]).to_string());
            } else {
                debug!("Found other file: {filename}");
                other_files.insert(filename);
            }
        }

        Ok(Self {
            original_path: path.to_path_buf(),
            units,
            other_files,
        })
    }

    pub fn has_unit(&self, unit_name: &str) -> bool {
        self.units.contains(&unit_name.to_string())
    }

    pub fn read_class_file(&self, unit_name: &str) -> anyhow::Result<Cursor<Vec<u8>>> {
        let mut archive =
            ZipArchive::new(OpenOptions::new().read(true).open(&self.original_path)?)?;

        let mut content = vec![];

        archive
            .by_name_seek(&format!("{unit_name}.class"))?
            .read_to_end(&mut content)?;

        Ok(Cursor::new(content))
    }
}
