#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to read the file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse TOML data. The file may be invalid.")]
    TomlParse(#[from] toml::de::Error),

    #[error("Failed to serialize TOML data: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Error")]
    EmptyFile,
}
