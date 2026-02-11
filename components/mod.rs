//! shadcn-ui-rs components
//!
//! This module re-exports all Phase 1 components. In a typical user project,
//! individual component files are copied into the project instead of being
//! used as a library dependency.

pub mod button;
pub mod card;
pub mod checkbox;
pub mod dialog;
pub mod input;
pub mod label;
pub mod radio;
pub mod select;
pub mod slider;
pub mod switch;
pub mod toggle;
pub mod toggle_group;

pub use button::{Button, ButtonSize, ButtonVariant};
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use checkbox::Checkbox;
pub use dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger,
};
pub use input::Input;
pub use label::Label;
pub use radio::{RadioGroup, RadioItem};
pub use select::{Select, SelectItem};
pub use slider::Slider;
pub use switch::Switch;
pub use toggle::{Toggle, ToggleSize, ToggleVariant};
pub use toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupSize, ToggleGroupType, ToggleGroupVariant};
