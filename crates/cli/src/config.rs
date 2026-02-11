//! Configuration file management for shadcn-ui projects.
//!
//! Handles reading and writing `shadcn-ui.toml` configuration files.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// The configuration file name used by shadcn-ui projects.
pub const CONFIG_FILE_NAME: &str = "shadcn-ui.toml";

/// Top-level configuration for a shadcn-ui project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub theme: ThemeConfig,
    pub registry: RegistryConfig,
}

/// Project-level settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub components_dir: String,
    pub theme_file: String,
}

/// Theme settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub base_color: String,
    pub radius: String,
    pub dark_mode: bool,
}

/// Registry settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig {
                components_dir: "src/components/ui".to_string(),
                theme_file: "src/theme.rs".to_string(),
            },
            theme: ThemeConfig {
                base_color: "zinc".to_string(),
                radius: "md".to_string(),
                dark_mode: true,
            },
            registry: RegistryConfig {
                url: "https://shadcn-ui-rs.dev/registry".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from a `shadcn-ui.toml` file in the given directory.
    pub fn load(project_dir: &Path) -> Result<Self> {
        let path = config_path(project_dir);
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config: Config = toml::from_str(&content).context("Failed to parse shadcn-ui.toml")?;
        Ok(config)
    }

    /// Save configuration to a `shadcn-ui.toml` file in the given directory.
    pub fn save(&self, project_dir: &Path) -> Result<()> {
        let path = config_path(project_dir);
        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        Ok(())
    }

    /// Check whether a `shadcn-ui.toml` exists in the given directory.
    pub fn exists(project_dir: &Path) -> bool {
        config_path(project_dir).exists()
    }
}

/// Return the full path to the config file for a project directory.
pub fn config_path(project_dir: &Path) -> PathBuf {
    project_dir.join(CONFIG_FILE_NAME)
}
