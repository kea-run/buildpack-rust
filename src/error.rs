#[derive(Debug)]
pub enum RustBuildPackError {
    MetadataRead(std::io::Error),
    BuildpackDetection(std::io::Error),
    MetadataParse(toml::de::Error),
    RustupDownload(ureq::Error),
    RustupSave(std::io::Error),
    RustupExec(std::io::Error),
    CargoExec(std::io::Error),
    CopyOutput(std::io::Error),
    CopyToRuntime(std::io::Error),
    RuntimePrepareBin(std::io::Error),
    RuntimeCleanAppDir(std::io::Error),
    RuntimeCreateBin(std::io::Error),
    RuntimeCleanLayersDir(std::io::Error),
    StripPrefix(std::path::StripPrefixError),
    Symlink(std::io::Error),
}
impl From<RustBuildPackError> for libcnb::Error<RustBuildPackError> {
    fn from(error: RustBuildPackError) -> Self {
        Self::BuildpackError(error)
    }
}
