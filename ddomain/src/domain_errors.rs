#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Failed to read the file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse TOML data. The file may be invalid.")]
    TomlParse(#[from] toml::de::Error),

    #[error("Failed to serialize TOML data: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("The file is empty or cannot be processed.")]
    EmptyFile,

    #[error("The file at {0} was not found.")]
    FileNotFound(String),
}
