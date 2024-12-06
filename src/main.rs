use eframe::egui::{
    scroll_area::{ScrollBarVisibility, ScrollSource},
    Button, CentralPanel, Color32, Context, CursorIcon, DragValue, Pos2, ScrollArea, Sense, Stroke,
    TopBottomPanel, Vec2,
};

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Scroll Area App",
        native_options,
        Box::new(|cc| Ok(Box::new(ScrollAreaApp::new(cc)))),
    )
    .unwrap();
}

struct ScrollAreaApp {
    drag_enabled: bool,
    wheel_enabled: bool,
    bar_enabled: bool,
    clicked_within: bool,
    wheel_scroll_multiplier: Vec2,
}

impl ScrollAreaApp {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            drag_enabled: true,
            wheel_enabled: true,
            bar_enabled: true,
            clicked_within: false,
            wheel_scroll_multiplier: Vec2::splat(1.0),
        }
    }
}

impl eframe::App for ScrollAreaApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        TopBottomPanel::top("top controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let button = Button::new("Drag").selected(self.drag_enabled);
                if ui.add(button).clicked() {
                    self.drag_enabled ^= true;
                }

                let button = Button::new("Wheel").selected(self.wheel_enabled);
                if ui.add(button).clicked() {
                    self.wheel_enabled ^= true;
                }

                let button = Button::new("Bar").selected(self.bar_enabled);
                if ui.add(button).clicked() {
                    self.bar_enabled ^= true;
                }
                ui.add_space(5.0);

                ui.label("X multiplier:");
                let dv = DragValue::new(&mut self.wheel_scroll_multiplier.x)
                    .clamp_existing_to_range(true)
                    .range(-2.0..=2.0)
                    .fixed_decimals(1)
                    .suffix("x");
                ui.add(dv);
                ui.add_space(5.0);

                ui.label("Y multiplier:");
                let dv = DragValue::new(&mut self.wheel_scroll_multiplier.y)
                    .clamp_existing_to_range(true)
                    .range(-2.0..=2.0)
                    .fixed_decimals(1)
                    .suffix("x");
                ui.add(dv);
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            let source = {
                let mut source = ScrollSource::NONE;
                if self.drag_enabled {
                    source |= ScrollSource::DRAG;
                }
                if self.wheel_enabled {
                    source |= ScrollSource::MOUSE_WHEEL;
                }
                if self.bar_enabled {
                    source |= ScrollSource::SCROLL_BAR;
                }
                source
            };
            ScrollArea::both()
                .auto_shrink([false, false])
                .scroll_source(source)
                .on_hover_cursor(CursorIcon::Grab)
                .on_drag_cursor(CursorIcon::Grabbing)
                .scroll_bar_visibility(if self.bar_enabled {
                    ScrollBarVisibility::default()
                } else {
                    ScrollBarVisibility::AlwaysHidden
                })
                .wheel_scroll_multiplier(self.wheel_scroll_multiplier)
                .show(ui, |ui| {
                    let size = 3000.0;
                    let (response, painter) = {
                        let size = Vec2::splat(size);
                        ui.allocate_painter(size, Sense::click())
                    };
                    if response.clicked() {
                        self.clicked_within ^= true;
                    }
                    let response = if self.clicked_within {
                        response.on_hover_cursor(CursorIcon::Crosshair)
                    } else {
                        response
                    };

                    let canvas_origin = response.rect.min;
                    let canvas_width = response.rect.width();
                    let canvas_height = response.rect.height();

                    // draw some shapes
                    let lines = 5;
                    let coords = (-lines..=lines)
                        .map(|i| {
                            let l = i as f32 / lines as f32;
                            (l.powf(2.0) * l.signum() + 1.0) * size / 2.0
                        })
                        .collect::<Vec<_>>();

                    for c in coords {
                        painter.vline(
                            c + canvas_origin.x,
                            0.0..=size,
                            Stroke::new(2.0, Color32::DARK_RED),
                        );
                        painter.hline(
                            0.0..=size,
                            c + canvas_origin.y,
                            Stroke::new(2.0, Color32::DARK_RED),
                        );

                        painter.line_segment(
                            [
                                Pos2::new(canvas_origin.x, c + canvas_origin.y),
                                Pos2::new(c + canvas_origin.x, canvas_origin.y),
                            ],
                            Stroke::new(2.0, Color32::DARK_BLUE),
                        );
                        painter.line_segment(
                            [
                                Pos2::new(canvas_origin.x + canvas_width, c + canvas_origin.y),
                                Pos2::new(canvas_width - c + canvas_origin.x, canvas_origin.y),
                            ],
                            Stroke::new(2.0, Color32::DARK_BLUE),
                        );
                        painter.line_segment(
                            [
                                Pos2::new(canvas_origin.x, c + canvas_origin.y),
                                Pos2::new(
                                    canvas_width - c + canvas_origin.x,
                                    canvas_origin.y + canvas_height,
                                ),
                            ],
                            Stroke::new(2.0, Color32::DARK_BLUE),
                        );
                        painter.line_segment(
                            [
                                Pos2::new(
                                    canvas_origin.x + canvas_width,
                                    canvas_height - c + canvas_origin.y,
                                ),
                                Pos2::new(
                                    canvas_width - c + canvas_origin.x,
                                    canvas_origin.y + canvas_height,
                                ),
                            ],
                            Stroke::new(2.0, Color32::DARK_BLUE),
                        );

                        painter.circle_stroke(
                            Pos2::new(canvas_origin.x + size / 2.0, canvas_origin.y + size / 2.0),
                            c,
                            Stroke::new(3.0, Color32::DARK_GREEN),
                        );
                    }
                });
        });
    }
}
