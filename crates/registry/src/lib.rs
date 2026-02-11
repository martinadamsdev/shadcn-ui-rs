//! Component registry definitions for shadcn-ui-rs

use serde::{Deserialize, Serialize};

/// Component metadata in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub gpui_version: String,
    pub files: Vec<String>,
    pub dependencies: Vec<String>,
    pub category: ComponentCategory,
}

/// Component categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentCategory {
    Input,
    Display,
    Feedback,
    Navigation,
    Layout,
    Special,
}

/// Registry containing all available components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub version: String,
    pub components: Vec<ComponentMeta>,
}

impl Registry {
    /// Get all component names
    pub fn component_names(&self) -> Vec<&str> {
        self.components.iter().map(|c| c.name.as_str()).collect()
    }

    /// Find a component by name
    pub fn find(&self, name: &str) -> Option<&ComponentMeta> {
        self.components.iter().find(|c| c.name == name)
    }

    /// Get components by category
    pub fn by_category(&self, category: ComponentCategory) -> Vec<&ComponentMeta> {
        self.components.iter().filter(|c| c.category == category).collect()
    }
}

/// Default registry with Phase 1 components
pub fn default_registry() -> Registry {
    Registry {
        version: "0.1.0".to_string(),
        components: vec![
            ComponentMeta {
                name: "button".to_string(),
                version: "0.1.0".to_string(),
                description: "A button component with multiple variants and sizes".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["button.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "input".to_string(),
                version: "0.1.0".to_string(),
                description: "A text input component".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["input.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            // TODO: Add more components
        ],
    }
}
