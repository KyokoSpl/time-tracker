use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

pub struct MaterialUI;

impl MaterialUI {
    pub fn setup_style(ctx: &egui::Context) {
        let mut style = egui::Style::default();
        
        // Material Design color scheme
        style.visuals.button_frame = true;
        style.visuals.collapsing_header_frame = true;
        style.spacing.button_padding = Vec2::new(16.0, 8.0);
        style.spacing.indent = 20.0;
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        
        // Material Design rounding
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);
        
        ctx.set_style(style);
    }

    pub fn toggle_theme(ctx: &egui::Context, dark_mode: &mut bool) {
        *dark_mode = !*dark_mode;
        
        let mut style = ctx.style().as_ref().clone();
        if *dark_mode {
            style.visuals = egui::Visuals::dark();
        } else {
            style.visuals = egui::Visuals::light();
        }
        
        // Keep Material Design styling
        style.visuals.button_frame = true;
        style.visuals.collapsing_header_frame = true;
        style.spacing.button_padding = Vec2::new(16.0, 8.0);
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);
        
        ctx.set_style(style);
    }

    pub fn material_button(ui: &mut egui::Ui, text: &str, filled: bool) -> egui::Response {
        let button = if filled {
            egui::Button::new(text)
                .fill(Color32::from_rgb(103, 80, 164)) // Material Purple
                .stroke(Stroke::NONE)
        } else {
            egui::Button::new(text)
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::new(1.0, Color32::from_rgb(103, 80, 164)))
        };
        
        ui.add_sized([80.0, 32.0], button)
    }

    pub fn material_card<R>(
        ui: &mut egui::Ui,
        elevated: bool,
        content: impl FnOnce(&mut egui::Ui) -> R,
    ) -> R {
        let frame = if elevated {
            egui::Frame::default()
                .fill(ui.visuals().panel_fill)
                .stroke(Stroke::NONE)
                .rounding(Rounding::same(12.0))
                .shadow(egui::epaint::Shadow {
                    offset: Vec2::new(0.0, 2.0),
                    blur: 4.0,
                    spread: 0.0,
                    color: Color32::from_black_alpha(25),
                })
                .inner_margin(16.0)
        } else {
            egui::Frame::default()
                .fill(ui.visuals().panel_fill)
                .stroke(Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                .rounding(Rounding::same(12.0))
                .inner_margin(16.0)
        };
        
        frame.show(ui, content).inner
    }
}