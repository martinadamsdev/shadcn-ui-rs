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

// Phase 3
pub mod avatar;
pub mod badge;
pub mod empty;
pub mod kbd;
pub mod progress;
pub mod scroll_area;
pub mod separator;
pub mod skeleton;
pub mod table;
pub mod textarea;
pub mod typography;

// Phase 3 re-exports
pub use avatar::{Avatar, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use empty::Empty;
pub use kbd::Kbd;
pub use progress::Progress;
pub use scroll_area::{ScrollArea, ScrollOrientation};
pub use separator::{Separator, SeparatorOrientation};
pub use skeleton::Skeleton;
pub use table::{Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow};
pub use textarea::Textarea;
pub use typography::{Blockquote, H1, H2, H3, H4, InlineCode, Lead, Paragraph};

// Phase 4
pub mod accordion;
pub mod breadcrumb;
pub mod button_group;
pub mod collapsible;
pub mod context_menu;
pub mod field;
pub mod item;
pub mod menubar;
pub mod navigation_menu;
pub mod pagination;
pub mod sidebar;
pub mod tabs;

// Phase 4 re-exports
pub use accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem, BreadcrumbSeparator};
pub use button_group::{ButtonGroup, ButtonGroupOrientation};
pub use collapsible::Collapsible;
pub use context_menu::ContextMenu;
pub use field::Field;
pub use item::Item;
pub use menubar::{Menubar, MenubarMenu, MenubarSeparator};
pub use navigation_menu::{NavigationMenu, NavigationMenuItem, NavigationMenuLink};
pub use pagination::Pagination;
pub use sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarHeader, SidebarSide, SidebarTrigger,
};
pub use tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
