//! Component registry definitions for shadcn-ui-rs.
//!
//! Provides metadata about available components, their dependencies,
//! and categorization.

use serde::{Deserialize, Serialize};

/// Component metadata in the registry.
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

/// Component categories.
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

impl ComponentCategory {
    /// Human-readable display name for the category.
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentCategory::Input => "Input",
            ComponentCategory::Display => "Display",
            ComponentCategory::Feedback => "Feedback",
            ComponentCategory::Navigation => "Navigation",
            ComponentCategory::Layout => "Layout",
            ComponentCategory::Special => "Special",
        }
    }
}

/// Registry containing all available components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub version: String,
    pub components: Vec<ComponentMeta>,
}

impl Registry {
    /// Get all component names.
    pub fn component_names(&self) -> Vec<&str> {
        self.components.iter().map(|c| c.name.as_str()).collect()
    }

    /// Find a component by name.
    pub fn find(&self, name: &str) -> Option<&ComponentMeta> {
        self.components.iter().find(|c| c.name == name)
    }

    /// Get components by category.
    pub fn by_category(&self, category: ComponentCategory) -> Vec<&ComponentMeta> {
        self.components
            .iter()
            .filter(|c| c.category == category)
            .collect()
    }

    /// Resolve all transitive dependencies for a set of component names.
    ///
    /// Returns the original names plus any dependencies, in installation order
    /// (dependencies first).
    pub fn resolve_dependencies(&self, names: &[&str]) -> Vec<String> {
        let mut resolved: Vec<String> = Vec::new();
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();

        for name in names {
            self.resolve_recursive(name, &mut resolved, &mut visited);
        }

        resolved
    }

    fn resolve_recursive(
        &self,
        name: &str,
        resolved: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        if visited.contains(name) {
            return;
        }
        visited.insert(name.to_string());

        if let Some(component) = self.find(name) {
            for dep in &component.dependencies {
                self.resolve_recursive(dep, resolved, visited);
            }
        }

        resolved.push(name.to_string());
    }
}

/// Default registry with all components.
pub fn default_registry() -> Registry {
    Registry {
        version: "0.2.0".to_string(),
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
            ComponentMeta {
                name: "label".to_string(),
                version: "0.1.0".to_string(),
                description: "A label component for form fields".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["label.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "checkbox".to_string(),
                version: "0.1.0".to_string(),
                description: "A checkbox input with checked/unchecked/indeterminate states"
                    .to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["checkbox.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "radio".to_string(),
                version: "0.1.0".to_string(),
                description: "A radio group component for single selection".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["radio.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "switch".to_string(),
                version: "0.1.0".to_string(),
                description: "A toggle switch component".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["switch.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "slider".to_string(),
                version: "0.1.0".to_string(),
                description: "A slider input for selecting a value from a range".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["slider.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "select".to_string(),
                version: "0.1.0".to_string(),
                description: "A select dropdown for choosing from a list of options".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["select.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "toggle".to_string(),
                version: "0.1.0".to_string(),
                description: "A toggle button that can be on or off".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["toggle.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "toggle_group".to_string(),
                version: "0.1.0".to_string(),
                description: "A group of toggle buttons with single or multiple selection"
                    .to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["toggle_group.rs".to_string()],
                dependencies: vec!["toggle".to_string()],
                category: ComponentCategory::Input,
            },
            ComponentMeta {
                name: "card".to_string(),
                version: "0.1.0".to_string(),
                description: "A card container with header, content, and footer sections"
                    .to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["card.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Display,
            },
            ComponentMeta {
                name: "dialog".to_string(),
                version: "0.1.0".to_string(),
                description: "A modal dialog overlay with backdrop".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["dialog.rs".to_string()],
                dependencies: vec!["button".to_string()],
                category: ComponentCategory::Feedback,
            },
            // Phase 2
            ComponentMeta {
                name: "alert".to_string(),
                version: "0.2.0".to_string(),
                description: "A static alert box with icon, title, and description".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["alert.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Feedback,
            },
            ComponentMeta {
                name: "alert_dialog".to_string(),
                version: "0.2.0".to_string(),
                description: "A modal confirmation dialog with action and cancel buttons"
                    .to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["alert_dialog.rs".to_string()],
                dependencies: vec!["dialog".to_string()],
                category: ComponentCategory::Feedback,
            },
            ComponentMeta {
                name: "tooltip".to_string(),
                version: "0.2.0".to_string(),
                description: "A hover-triggered overlay with text content".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["tooltip.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Display,
            },
            ComponentMeta {
                name: "popover".to_string(),
                version: "0.2.0".to_string(),
                description: "A click-triggered overlay with arbitrary content".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["popover.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Display,
            },
            ComponentMeta {
                name: "hover_card".to_string(),
                version: "0.2.0".to_string(),
                description: "A hover-triggered card overlay with rich content".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["hover_card.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Display,
            },
            ComponentMeta {
                name: "dropdown_menu".to_string(),
                version: "0.2.0".to_string(),
                description: "A click-triggered menu with items, separators, and labels"
                    .to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["dropdown_menu.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Navigation,
            },
            ComponentMeta {
                name: "sheet".to_string(),
                version: "0.2.0".to_string(),
                description: "A slide-in overlay panel from screen edge".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["sheet.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Layout,
            },
            ComponentMeta {
                name: "drawer".to_string(),
                version: "0.2.0".to_string(),
                description: "A bottom sheet variant with drag handle".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["drawer.rs".to_string()],
                dependencies: vec!["sheet".to_string()],
                category: ComponentCategory::Layout,
            },
            ComponentMeta {
                name: "toast".to_string(),
                version: "0.2.0".to_string(),
                description: "A temporary notification with auto-dismiss support".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["toast.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Feedback,
            },
            ComponentMeta {
                name: "sonner".to_string(),
                version: "0.2.0".to_string(),
                description: "A stacked toast notification system".to_string(),
                gpui_version: ">=0.2.0".to_string(),
                files: vec!["sonner.rs".to_string()],
                dependencies: vec![],
                category: ComponentCategory::Feedback,
            },
        ],
    }
}
