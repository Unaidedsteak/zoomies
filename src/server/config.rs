use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub zoom_bin_path: String,
}
