use std::path::PathBuf;

pub enum Backend{
    QBE,
    C,
}

impl Backend{
    pub fn map(s: &str) -> Self{
        match s{
            "qbe" => Self::QBE,
            "c" => Self::C,
            _ => panic!("unknown backend flag value: {}", s)
        }
    }
}

pub struct Config{
    pub path: PathBuf,
    pub debug_mode: bool,
    pub backend: Backend,
    pub version: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { path: PathBuf::new(), debug_mode: false, backend: Backend::QBE, version: env!("CARGO_PKG_VERSION").to_string() }
    }
}