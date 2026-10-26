use {
    crate::ensure,
    anyhow::{Error, Result},
    semver::Version,
    serde::Serialize,
    std::{
        convert::AsRef,
        fs::{DirEntry, FileType, read_dir, write},
        io::Result as IOResult,
        path::{Path, PathBuf},
        string::String,
        vec::Vec,
    },
    toml::to_string_pretty,
};

pub const CRANK_FILE_NAME: &'static str = "Crank.toml";

#[derive(Serialize)]
pub struct TargetMetadata<'t> {
    pub name: &'t str,
    pub version: Version,
    pub author: &'t str,
    pub bundle_id: &'t str,
    pub assets: Vec<PathBuf>,
}

impl<'t> TargetMetadata<'t> {
    fn try_new<P: AsRef<Path>>(
        name: &'t str,
        version: Version,
        author: &'t str,
        bundle_id: &'t str,
        assets_path: P,
    ) -> IOResult<Self> {
        Self::try_assets_from_path(assets_path).map(|assets| Self {
            name,
            version,
            author,
            bundle_id,
            assets,
        })
    }

    fn try_assets_from_path<P: AsRef<Path>>(assets_path: P) -> IOResult<Vec<PathBuf>> {
        let mut asset_files: Vec<PathBuf> = Vec::new();
        let mut asset_dirs: Vec<PathBuf> = vec![assets_path.as_ref().to_path_buf()];

        while {
            for dir_entry in read_dir(asset_dirs.pop().unwrap())? {
                let dir_entry: DirEntry = dir_entry?;
                let file_type: FileType = dir_entry.file_type()?;

                if file_type.is_dir() {
                    asset_dirs.push(dir_entry.path());
                } else if file_type.is_file() {
                    asset_files.push(dir_entry.path());
                }
            }

            !asset_dirs.is_empty()
        } {}

        asset_files.sort();

        Ok(asset_files)
    }
}

#[derive(Serialize)]
pub struct Target<'t> {
    pub name: &'t str,
    pub metadata: TargetMetadata<'t>,
}

impl<'t> Target<'t> {
    fn try_new<P: AsRef<Path>>(
        name: &'t str,
        version: Version,
        author: &'t str,
        bundle_id: &'t str,
        assets_path: P,
    ) -> IOResult<Self> {
        TargetMetadata::try_new(name, version, author, bundle_id, assets_path)
            .map(|metadata| Self { name, metadata })
    }
}

#[derive(Serialize)]
pub struct Document<'d> {
    pub target: Vec<Target<'d>>,
}

impl<'d> Document<'d> {
    pub fn try_new<P: AsRef<Path>>(
        name: &'d str,
        version: Version,
        author: &'d str,
        bundle_id: &'d str,
        assets_path: P,
    ) -> IOResult<Self> {
        Target::try_new(name, version, author, bundle_id, assets_path).map(|target| Self {
            target: vec![target],
        })
    }

    pub fn try_write<P: AsRef<Path>>(&self, dir: P) -> Result<()> {
        ensure!(dir.as_ref().is_dir());

        let crank_string: String = to_string_pretty(self)?;

        let mut crank_path: PathBuf = dir.as_ref().to_path_buf();

        crank_path.push(CRANK_FILE_NAME);

        write(crank_path, crank_string).map_err(Error::new)
    }
}
