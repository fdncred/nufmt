//! Config module
//!
//! This keeps all the options, tweaks and dials of the configuration.

use anyhow::Result;

#[derive(Debug)]
/// Configurations available to the formatter
pub struct Config {
    /// Number of spaces of indent.
    pub tab_spaces: usize,
    /// Maximum width of each line.
    pub max_width: usize,
    /// Number of lines bafore and after a custom command.
    pub margin: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tab_spaces: 4,
            max_width: 80,
            margin: 1,
        }
    }
}

impl Config {
    /// Creates a new config. You need to pass every field to create the config.
    /// You cannot skip any field yet.
    pub fn new(tab_spaces: usize, max_width: usize, margin: usize) -> Self {
        Config {
            tab_spaces,
            max_width,
            margin,
        }
    }
}

/// Returns a default config.
pub fn load(/* file_path: Option<&Path> */) -> Result<Config> {
    Ok(Config::default())
}
