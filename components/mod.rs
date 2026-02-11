//! shadcn-ui-rs components
//!
//! This module re-exports all components. In a typical user project,
//! individual component files are copied into the project instead of being
//! used as a library dependency.

// Phase 1
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

// Phase 2
pub mod alert;
pub mod alert_dialog;
pub mod dropdown_menu;
pub mod drawer;
pub mod hover_card;
pub mod popover;
pub mod sheet;
pub mod sonner;
pub mod toast;
pub mod tooltip;

// Phase 1 re-exports
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
pub use toggle_group::{
    ToggleGroup, ToggleGroupItem, ToggleGroupSize, ToggleGroupType, ToggleGroupVariant,
};

// Phase 2 re-exports
pub use alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
pub use alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent,
    AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
};
pub use dropdown_menu::{
    DropdownMenu, DropdownMenuEntry, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuSeparator,
};
pub use drawer::{
    Drawer, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle,
};
pub use hover_card::{HoverCard, HoverCardSide};
pub use popover::{Popover, PopoverAlign, PopoverSide};
pub use sheet::{Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetTitle};
pub use sonner::{Sonner, SonnerPosition, SonnerToast, SonnerVariant};
pub use toast::{Toast, ToastVariant};
pub use tooltip::{Tooltip, TooltipSide};
