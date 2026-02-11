//! Component showcase demonstrating all 33 Phase 1-3 shadcn-ui-rs components.

use gpui::prelude::*;
use gpui::*;

mod components;
mod theme;

use components::ui::empty::Empty;
use components::ui::*;

struct ShowcaseView {
    name: String,
    email: String,
    notifications_enabled: bool,
    terms_accepted: bool,
    plan: String,
    volume: f32,
    bold_pressed: bool,
    italic_pressed: bool,
    alignment: String,
    fruit: String,
    fruit_select_open: bool,
    dialog_open: bool,
}

impl ShowcaseView {
    fn new() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            notifications_enabled: true,
            terms_accepted: false,
            plan: "pro".to_string(),
            volume: 65.0,
            bold_pressed: true,
            italic_pressed: false,
            alignment: "left".to_string(),
            fruit: String::new(),
            fruit_select_open: false,
            dialog_open: false,
        }
    }
}

impl Render for ShowcaseView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<theme::Theme>();
        let colors = &theme.colors;
        let bg = colors.background;
        let fg = colors.foreground;
        let muted_fg = colors.muted_foreground;

        let dialog_open = self.dialog_open;

        div()
            .id("showcase-root")
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .overflow_y_scroll()
            .bg(bg)
            .text_color(fg)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(680.0))
                    .p(px(32.0))
                    .gap(px(24.0))
                    // ── Page Header ──
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(4.0))
                            .mb(px(8.0))
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .child("shadcn-ui-rs Component Showcase"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(muted_fg)
                                    .child("All 33 components from Phase 1\u{2013}3"),
                            ),
                    )
                    // ══════════════════════════════════════════
                    // Phase 1: Form Inputs (12 components)
                    // ══════════════════════════════════════════
                    .child(section_heading(
                        "Phase 1: Form Inputs",
                        "Button \u{b7} Input \u{b7} Label \u{b7} Checkbox \u{b7} Radio \u{b7} Switch \u{b7} Slider \u{b7} Select \u{b7} Toggle \u{b7} ToggleGroup \u{b7} Card \u{b7} Dialog",
                    ))
                    .child(
                        Card::new()
                            .child(
                                CardHeader::new()
                                    .child(CardTitle::new("Account Settings"))
                                    .child(CardDescription::new(
                                        "Configure your account preferences below.",
                                    )),
                            )
                            .child(
                                CardContent::new().child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(20.0))
                                        .max_w(px(480.0))
                                        // Name field (Label + Input)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(6.0))
                                                .child(Label::new("Full Name").required(true))
                                                .child(
                                                    Input::new("name-input")
                                                        .placeholder("Enter your name")
                                                        .value(self.name.clone()),
                                                ),
                                        )
                                        // Email field (Label + Input)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(6.0))
                                                .child(Label::new("Email"))
                                                .child(
                                                    Input::new("email-input")
                                                        .placeholder("you@example.com")
                                                        .value(self.email.clone()),
                                                ),
                                        )
                                        // Notifications (Label + Switch)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .justify_between()
                                                .child(Label::new("Enable Notifications"))
                                                .child({
                                                    let entity = cx.entity().downgrade();
                                                    Switch::new("notifications-switch")
                                                        .checked(self.notifications_enabled)
                                                        .on_change(move |checked, _window, cx| {
                                                            _ = entity.update(cx, |this, cx| {
                                                                this.notifications_enabled =
                                                                    checked;
                                                                cx.notify();
                                                            });
                                                        })
                                                }),
                                        )
                                        // Plan selection (Label + RadioGroup)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(6.0))
                                                .child(Label::new("Plan"))
                                                .child({
                                                    let entity = cx.entity().downgrade();
                                                    RadioGroup::new("plan-group")
                                                        .value(self.plan.clone())
                                                        .on_change(move |value, _window, cx| {
                                                            _ = entity.update(cx, |this, cx| {
                                                                this.plan = value.to_string();
                                                                cx.notify();
                                                            });
                                                        })
                                                        .child(RadioItem::new("free", "Free"))
                                                        .child(RadioItem::new("pro", "Pro"))
                                                        .child(
                                                            RadioItem::new(
                                                                "enterprise",
                                                                "Enterprise",
                                                            )
                                                            .disabled(true),
                                                        )
                                                }),
                                        )
                                        // Fruit selection (Label + Select)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(6.0))
                                                .child(Label::new("Favorite Fruit"))
                                                .child({
                                                    let entity_open = cx.entity().downgrade();
                                                    let entity_change = cx.entity().downgrade();
                                                    let mut select = Select::new("fruit-select")
                                                        .placeholder("Pick a fruit...")
                                                        .open(self.fruit_select_open)
                                                        .on_open_change(
                                                            move |open, _window, cx| {
                                                                _ = entity_open.update(
                                                                    cx,
                                                                    |this, cx| {
                                                                        this.fruit_select_open =
                                                                            open;
                                                                        cx.notify();
                                                                    },
                                                                );
                                                            },
                                                        )
                                                        .on_change(move |value, _window, cx| {
                                                            _ = entity_change.update(
                                                                cx,
                                                                |this, cx| {
                                                                    this.fruit =
                                                                        value.to_string();
                                                                    cx.notify();
                                                                },
                                                            );
                                                        })
                                                        .child(SelectItem::new("apple", "Apple"))
                                                        .child(SelectItem::new(
                                                            "banana", "Banana",
                                                        ))
                                                        .child(SelectItem::new(
                                                            "cherry", "Cherry",
                                                        ))
                                                        .child(
                                                            SelectItem::new("durian", "Durian")
                                                                .disabled(true),
                                                        );
                                                    if !self.fruit.is_empty() {
                                                        select =
                                                            select.value(self.fruit.clone());
                                                    }
                                                    select
                                                }),
                                        )
                                        // Volume (Label + Slider)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(6.0))
                                                .child(Label::new(format!(
                                                    "Volume: {:.0}%",
                                                    self.volume
                                                )))
                                                .child(
                                                    Slider::new("volume-slider")
                                                        .min(0.0)
                                                        .max(100.0)
                                                        .value(self.volume)
                                                        .step(5.0),
                                                ),
                                        )
                                        // Text formatting (Label + Toggle)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(6.0))
                                                .child(Label::new("Text Formatting"))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .gap(px(4.0))
                                                        .child({
                                                            let entity = cx.entity().downgrade();
                                                            Toggle::new("bold-toggle")
                                                                .pressed(self.bold_pressed)
                                                                .variant(ToggleVariant::Outline)
                                                                .on_press_change(
                                                                    move |pressed, _window, cx| {
                                                                        _ = entity.update(
                                                                            cx,
                                                                            |this, cx| {
                                                                                this.bold_pressed =
                                                                                    pressed;
                                                                                cx.notify();
                                                                            },
                                                                        );
                                                                    },
                                                                )
                                                                .child("B")
                                                        })
                                                        .child({
                                                            let entity = cx.entity().downgrade();
                                                            Toggle::new("italic-toggle")
                                                                .pressed(self.italic_pressed)
                                                                .variant(ToggleVariant::Outline)
                                                                .on_press_change(
                                                                    move |pressed, _window, cx| {
                                                                        _ = entity.update(
                                                                            cx,
                                                                            |this, cx| {
                                                                                this.italic_pressed = pressed;
                                                                                cx.notify();
                                                                            },
                                                                        );
                                                                    },
                                                                )
                                                                .child("I")
                                                        }),
                                                ),
                                        )
                                        // Alignment (Label + ToggleGroup)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(6.0))
                                                .child(Label::new("Alignment"))
                                                .child({
                                                    let entity_l = cx.entity().downgrade();
                                                    let entity_c = cx.entity().downgrade();
                                                    let entity_r = cx.entity().downgrade();
                                                    ToggleGroup::new("alignment-group")
                                                        .type_(ToggleGroupType::Single)
                                                        .child(
                                                            ToggleGroupItem::new(
                                                                "align-left",
                                                                "left",
                                                            )
                                                            .pressed(self.alignment == "left")
                                                            .variant(ToggleGroupVariant::Outline)
                                                            .on_click(
                                                                move |_event, _window, cx| {
                                                                    _ = entity_l.update(
                                                                        cx,
                                                                        |this, cx| {
                                                                            this.alignment =
                                                                                "left".to_string();
                                                                            cx.notify();
                                                                        },
                                                                    );
                                                                },
                                                            )
                                                            .child("L"),
                                                        )
                                                        .child(
                                                            ToggleGroupItem::new(
                                                                "align-center",
                                                                "center",
                                                            )
                                                            .pressed(self.alignment == "center")
                                                            .variant(ToggleGroupVariant::Outline)
                                                            .on_click(
                                                                move |_event, _window, cx| {
                                                                    _ = entity_c.update(
                                                                        cx,
                                                                        |this, cx| {
                                                                            this.alignment =
                                                                                "center"
                                                                                    .to_string();
                                                                            cx.notify();
                                                                        },
                                                                    );
                                                                },
                                                            )
                                                            .child("C"),
                                                        )
                                                        .child(
                                                            ToggleGroupItem::new(
                                                                "align-right",
                                                                "right",
                                                            )
                                                            .pressed(self.alignment == "right")
                                                            .variant(ToggleGroupVariant::Outline)
                                                            .on_click(
                                                                move |_event, _window, cx| {
                                                                    _ = entity_r.update(
                                                                        cx,
                                                                        |this, cx| {
                                                                            this.alignment =
                                                                                "right".to_string();
                                                                            cx.notify();
                                                                        },
                                                                    );
                                                                },
                                                            )
                                                            .child("R"),
                                                        )
                                                }),
                                        )
                                        // Terms (Checkbox + Label)
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap(px(8.0))
                                                .child({
                                                    let entity = cx.entity().downgrade();
                                                    Checkbox::new("terms-checkbox")
                                                        .checked(self.terms_accepted)
                                                        .on_toggle(move |checked, _window, cx| {
                                                            _ = entity.update(cx, |this, cx| {
                                                                this.terms_accepted = checked;
                                                                cx.notify();
                                                            });
                                                        })
                                                })
                                                .child(Label::new(
                                                    "I accept the terms and conditions",
                                                )),
                                        ),
                                ),
                            )
                            .child(
                                CardFooter::new().child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap(px(8.0))
                                        .child(
                                            Button::new("Cancel")
                                                .variant(ButtonVariant::Outline),
                                        )
                                        .child(
                                            Button::new("Save")
                                                .disabled(!self.terms_accepted),
                                        )
                                        .child({
                                            let entity = cx.entity().downgrade();
                                            Button::new("Open Dialog")
                                                .variant(ButtonVariant::Secondary)
                                                .on_click(move |_event, _window, cx| {
                                                    _ = entity.update(cx, |this, cx| {
                                                        this.dialog_open = true;
                                                        cx.notify();
                                                    });
                                                })
                                        })
                                        .child(
                                            Button::new("Delete")
                                                .variant(ButtonVariant::Destructive),
                                        ),
                                ),
                            ),
                    )
                    // ══════════════════════════════════════════
                    // Phase 2: Overlays & Feedback (10 components)
                    // ══════════════════════════════════════════
                    .child(section_heading(
                        "Phase 2: Overlays & Feedback",
                        "Alert \u{b7} AlertDialog \u{b7} Tooltip \u{b7} Popover \u{b7} HoverCard \u{b7} DropdownMenu \u{b7} Sheet \u{b7} Drawer \u{b7} Toast \u{b7} Sonner",
                    ))
                    // Alert demos
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(12.0))
                            .w_full()
                            .child(
                                Alert::new()
                                    .child(AlertTitle::new("Heads up!"))
                                    .child(AlertDescription::new(
                                        "You can add components to your project using the CLI.",
                                    )),
                            )
                            .child(
                                Alert::new()
                                    .variant(AlertVariant::Destructive)
                                    .child(AlertTitle::new("Error"))
                                    .child(AlertDescription::new(
                                        "Your session has expired. Please log in again.",
                                    )),
                            ),
                    )
                    // Tooltip demo
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(16.0))
                            .child(
                                Tooltip::new("demo-tooltip")
                                    .text("This is a tooltip")
                                    .side(TooltipSide::Top)
                                    .open(true)
                                    .child(
                                        Button::new("Hover me")
                                            .variant(ButtonVariant::Outline),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(muted_fg)
                                    .child("Overlay components: AlertDialog, Popover, HoverCard, DropdownMenu, Sheet, Drawer, Toast, Sonner"),
                            ),
                    )
                    // ══════════════════════════════════════════
                    // Phase 3: Visual Display (11 components)
                    // ══════════════════════════════════════════
                    .child(section_heading(
                        "Phase 3: Visual Display",
                        "Badge \u{b7} Avatar \u{b7} Separator \u{b7} Skeleton \u{b7} Progress \u{b7} Kbd \u{b7} Typography \u{b7} Table \u{b7} ScrollArea \u{b7} Textarea \u{b7} Empty",
                    ))
                    // Badge
                    .child(
                        subsection("Badge")
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .gap(px(8.0))
                                    .child(Badge::new("Default"))
                                    .child(
                                        Badge::new("Secondary")
                                            .variant(BadgeVariant::Secondary),
                                    )
                                    .child(
                                        Badge::new("Outline").variant(BadgeVariant::Outline),
                                    )
                                    .child(
                                        Badge::new("Destructive")
                                            .variant(BadgeVariant::Destructive),
                                    ),
                            ),
                    )
                    // Avatar
                    .child(
                        subsection("Avatar")
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(Avatar::new("SM").size(AvatarSize::Sm))
                                    .child(Avatar::new("MD"))
                                    .child(Avatar::new("LG").size(AvatarSize::Lg)),
                            ),
                    )
                    // Separator
                    .child(subsection("Separator").child(Separator::new()))
                    // Skeleton
                    .child(
                        subsection("Skeleton")
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(
                                        Skeleton::new()
                                            .width(px(40.0))
                                            .height(px(40.0))
                                            .rounded(true),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap(px(6.0))
                                            .child(
                                                Skeleton::new()
                                                    .width(px(200.0))
                                                    .height(px(16.0)),
                                            )
                                            .child(
                                                Skeleton::new()
                                                    .width(px(160.0))
                                                    .height(px(12.0)),
                                            ),
                                    ),
                            ),
                    )
                    // Progress
                    .child(
                        subsection("Progress")
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .w_full()
                                    .child(Progress::new(33.0))
                                    .child(Progress::new(66.0)),
                            ),
                    )
                    // Kbd
                    .child(
                        subsection("Kbd")
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(Kbd::new("\u{2318}K"))
                                    .child(Kbd::new("Ctrl+C"))
                                    .child(Kbd::new("Esc")),
                            ),
                    )
                    // Typography
                    .child(
                        subsection("Typography")
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.0))
                                    .child(H1::new("Heading 1"))
                                    .child(H2::new("Heading 2"))
                                    .child(H3::new("Heading 3"))
                                    .child(H4::new("Heading 4"))
                                    .child(Lead::new(
                                        "Lead paragraph with larger, muted text.",
                                    ))
                                    .child(Paragraph::new(
                                        "This is a paragraph demonstrating the typography component with base text size and normal line height.",
                                    ))
                                    .child(
                                        Blockquote::new().child(Paragraph::new(
                                            "Everything should be as simple as it can be, but not simpler.",
                                        )),
                                    )
                                    .child(InlineCode::new("cargo add gpui")),
                            ),
                    )
                    // Table
                    .child(
                        subsection("Table")
                            .child(
                                Table::new()
                                    .child(
                                        TableHeader::new().child(
                                            TableRow::new()
                                                .child(TableHead::new("Name"))
                                                .child(TableHead::new("Email"))
                                                .child(TableHead::new("Role")),
                                        ),
                                    )
                                    .child(
                                        TableBody::new()
                                            .child(
                                                TableRow::new()
                                                    .child(
                                                        TableCell::new()
                                                            .child("Alice Johnson"),
                                                    )
                                                    .child(
                                                        TableCell::new()
                                                            .child("alice@example.com"),
                                                    )
                                                    .child(TableCell::new().child("Admin")),
                                            )
                                            .child(
                                                TableRow::new()
                                                    .child(
                                                        TableCell::new().child("Bob Smith"),
                                                    )
                                                    .child(
                                                        TableCell::new()
                                                            .child("bob@example.com"),
                                                    )
                                                    .child(TableCell::new().child("Editor")),
                                            )
                                            .child(
                                                TableRow::new()
                                                    .child(
                                                        TableCell::new()
                                                            .child("Carol Davis"),
                                                    )
                                                    .child(
                                                        TableCell::new()
                                                            .child("carol@example.com"),
                                                    )
                                                    .child(TableCell::new().child("Viewer")),
                                            ),
                                    )
                                    .child(TableCaption::new("A list of team members")),
                            ),
                    )
                    // ScrollArea
                    .child(
                        subsection("ScrollArea").child(
                            ScrollArea::new("scroll-demo").max_height(px(120.0)).child(
                                div().flex().flex_col().gap(px(4.0)).children(
                                    (1..=10).map(|i| {
                                        div()
                                            .py(px(4.0))
                                            .text_sm()
                                            .child(format!("Scrollable item {i}"))
                                    }),
                                ),
                            ),
                        ),
                    )
                    // Textarea
                    .child(
                        subsection("Textarea").child(
                            Textarea::new("demo-textarea")
                                .placeholder("Enter a description...")
                                .min_rows(3),
                        ),
                    )
                    // Empty
                    .child(
                        subsection("Empty").child(
                            Empty::new("No results found")
                                .description("Try adjusting your search or filter criteria.")
                                .action(
                                    Button::new("Clear filters")
                                        .variant(ButtonVariant::Outline),
                                ),
                        ),
                    )
                    // Bottom padding
                    .child(div().h(px(32.0))),
            )
            // Dialog (renders on top when open)
            .when(dialog_open, |el: Stateful<Div>| {
                el.child({
                    let entity_close = cx.entity().downgrade();
                    let entity_cancel = cx.entity().downgrade();
                    let entity_save = cx.entity().downgrade();
                    Dialog::new("settings-dialog")
                        .open(true)
                        .on_close(move |_window, cx| {
                            _ = entity_close.update(cx, |this, cx| {
                                this.dialog_open = false;
                                cx.notify();
                            });
                        })
                        .child(
                            DialogContent::new()
                                .child(
                                    DialogHeader::new()
                                        .child(DialogTitle::new("Edit Profile"))
                                        .child(DialogDescription::new(
                                            "Make changes to your profile here. Click save when you're done.",
                                        )),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(12.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.0))
                                                .child(Label::new("Display Name"))
                                                .child(
                                                    Input::new("dialog-name")
                                                        .placeholder("Your display name"),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.0))
                                                .child(Label::new("Bio"))
                                                .child(
                                                    Input::new("dialog-bio")
                                                        .placeholder("Tell us about yourself"),
                                                ),
                                        ),
                                )
                                .child(
                                    DialogFooter::new()
                                        .child(
                                            Button::new("dialog-cancel")
                                                .variant(ButtonVariant::Outline)
                                                .on_click(move |_event, _window, cx| {
                                                    _ = entity_cancel.update(cx, |this, cx| {
                                                        this.dialog_open = false;
                                                        cx.notify();
                                                    });
                                                })
                                                .id("dialog-cancel-btn"),
                                        )
                                        .child(
                                            Button::new("dialog-save")
                                                .on_click(move |_event, _window, cx| {
                                                    _ = entity_save.update(cx, |this, cx| {
                                                        this.dialog_open = false;
                                                        cx.notify();
                                                    });
                                                })
                                                .id("dialog-save-btn"),
                                        ),
                                ),
                        )
                })
            })
    }
}

fn section_heading(title: &str, components: &str) -> Div {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .child(title.to_string()),
        )
        .child(
            div()
                .text_xs()
                .opacity(0.5)
                .child(components.to_string()),
        )
}

fn subsection(title: &str) -> Div {
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::SEMIBOLD)
                .child(title.to_string()),
        )
}

fn main() {
    Application::new().run(|cx| {
        cx.set_global(theme::zinc(theme::ThemeMode::Light));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(700.0), px(1000.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| ShowcaseView::new()),
        )
        .unwrap();
    });
}
