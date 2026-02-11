//! Basic form example demonstrating all 12 Phase 1 shadcn-ui-rs components.

use gpui::prelude::FluentBuilder as _;
use gpui::*;

mod components;
mod theme;

use components::ui::*;

struct FormView {
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

impl FormView {
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

impl Render for FormView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<theme::Theme>();
        let colors = &theme.colors;
        let bg = colors.background;
        let fg = colors.foreground;
        let muted_fg = colors.muted_foreground;

        let dialog_open = self.dialog_open;

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .bg(bg)
            .text_color(fg)
            .p(px(32.0))
            .overflow_hidden()
            // Page title
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .mb(px(8.0))
                    .child("shadcn-ui-rs Basic Form"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(muted_fg)
                    .mb(px(24.0))
                    .child("Demonstrating all 12 Phase 1 components"),
            )
            // Card wrapping the form
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
                                                        this.notifications_enabled = checked;
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
                                                    RadioItem::new("enterprise", "Enterprise")
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
                                                .on_open_change(move |open, _window, cx| {
                                                    _ = entity_open.update(cx, |this, cx| {
                                                        this.fruit_select_open = open;
                                                        cx.notify();
                                                    });
                                                })
                                                .on_change(move |value, _window, cx| {
                                                    _ = entity_change.update(cx, |this, cx| {
                                                        this.fruit = value.to_string();
                                                        cx.notify();
                                                    });
                                                })
                                                .child(SelectItem::new("apple", "Apple"))
                                                .child(SelectItem::new("banana", "Banana"))
                                                .child(SelectItem::new("cherry", "Cherry"))
                                                .child(
                                                    SelectItem::new("durian", "Durian")
                                                        .disabled(true),
                                                );
                                            if !self.fruit.is_empty() {
                                                select = select.value(self.fruit.clone());
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
                                        .child(Label::new(format!("Volume: {:.0}%", self.volume)))
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
                                                                _ = entity.update(cx, |this, cx| {
                                                                    this.bold_pressed = pressed;
                                                                    cx.notify();
                                                                });
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
                                                                _ = entity.update(cx, |this, cx| {
                                                                    this.italic_pressed = pressed;
                                                                    cx.notify();
                                                                });
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
                                                    ToggleGroupItem::new("align-left", "left")
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
                                                    ToggleGroupItem::new("align-center", "center")
                                                        .pressed(self.alignment == "center")
                                                        .variant(ToggleGroupVariant::Outline)
                                                        .on_click(
                                                            move |_event, _window, cx| {
                                                                _ = entity_c.update(
                                                                    cx,
                                                                    |this, cx| {
                                                                        this.alignment =
                                                                            "center".to_string();
                                                                        cx.notify();
                                                                    },
                                                                );
                                                            },
                                                        )
                                                        .child("C"),
                                                )
                                                .child(
                                                    ToggleGroupItem::new("align-right", "right")
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
                                        .child(Label::new("I accept the terms and conditions")),
                                ),
                        ),
                    )
                    .child(
                        CardFooter::new().child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(8.0))
                                .child(Button::new("Cancel").variant(ButtonVariant::Outline))
                                .child(Button::new("Save").disabled(!self.terms_accepted))
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
                                .child(Button::new("Delete").variant(ButtonVariant::Destructive)),
                        ),
                    ),
            )
            // Dialog (renders on top when open)
            .when(dialog_open, |el: Div| {
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

fn main() {
    Application::new().run(|cx| {
        cx.set_global(theme::zinc(theme::ThemeMode::Light));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(600.0), px(800.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| FormView::new()),
        )
        .unwrap();
    });
}
