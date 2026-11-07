use eframe::egui;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Page {
    Play,
    Versions,
    Mods,
    Accounts,
    Settings,
}

impl Page {
    const ALL: [(Self, &'static str); 5] = [
        (Self::Play, "Play"),
        (Self::Versions, "Versions"),
        (Self::Mods, "Mods"),
        (Self::Accounts, "Accounts"),
        (Self::Settings, "Settings"),
    ];
}

pub struct KrlApp {
    page: Page,
}

impl KrlApp {
    pub fn new(context: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = egui::Color32::from_rgb(19, 25, 36);
        visuals.panel_fill = egui::Color32::from_rgb(13, 18, 28);
        visuals.faint_bg_color = egui::Color32::from_rgb(29, 38, 53);
        visuals.extreme_bg_color = egui::Color32::from_rgb(9, 13, 21);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(29, 38, 53);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(36, 49, 68);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(48, 66, 91);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(56, 112, 184);
        context.egui_ctx.set_visuals(visuals);

        let mut style = (*context.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 12.0);
        style.spacing.button_padding = egui::vec2(14.0, 10.0);
        context.egui_ctx.set_style(style);

        Self { page: Page::Play }
    }

    fn render_header(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new("KRL")
                    .size(28.0)
                    .strong()
                    .color(egui::Color32::from_rgb(121, 180, 255)),
            );
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Kerynth Rust Launcher").strong());
                ui.label(
                    egui::RichText::new("Minecraft: Java Edition · Android")
                        .small()
                        .weak(),
                );
            });
        });
        ui.add_space(8.0);
        ui.separator();
    }

    fn render_navigation(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            for (page, label) in Page::ALL {
                let selected = self.page == page;
                if ui
                    .selectable_label(selected, egui::RichText::new(label).strong())
                    .clicked()
                {
                    self.page = page;
                }
            }
        });
    }

    fn render_play(&mut self, ui: &mut egui::Ui) {
        ui.heading("Your game, your profiles");
        ui.label("A mobile-first Minecraft Java launcher built with Rust.");
        ui.add_space(10.0);

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(egui::RichText::new("GAME STATUS").small().weak().strong());
            ui.add_space(4.0);
            ui.heading("No game installation detected");
            ui.label("Install a Minecraft version and a compatible runtime before launching.");
            ui.add_space(8.0);
            ui.add_enabled(
                false,
                egui::Button::new(egui::RichText::new("Launch Minecraft").strong())
                    .min_size(egui::vec2(ui.available_width(), 48.0)),
            );
            ui.label(
                egui::RichText::new(
                    "Launch is disabled until game files and runtime are verified.",
                )
                .small()
                .weak(),
            );
        });

        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            if ui.button("Manage versions").clicked() {
                self.page = Page::Versions;
            }
            if ui.button("Manage mods").clicked() {
                self.page = Page::Mods;
            }
        });

        ui.add_space(8.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                egui::RichText::new("PROJECT STATUS")
                    .small()
                    .weak()
                    .strong(),
            );
            ui.label("Rust core: implemented foundation");
            ui.label("Android interface: initial native UI shell");
            ui.label("Game installation and launch: not connected yet");
        });
    }

    fn render_versions(&self, ui: &mut egui::Ui) {
        ui.heading("Version manager");
        ui.label("Choose and install the Minecraft version for a game instance.");
        ui.add_space(8.0);

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.heading("Version catalogue");
            ui.label(concat!(
                "The Rust core can parse version manifests and filter release channels. ",
                "The official manifest download, metadata resolver, file verifier, and installer ",
                "are still in the implementation queue."
            ));
            ui.add_space(8.0);
            ui.add_enabled(false, egui::Button::new("Refresh catalogue"));
            ui.add_enabled(false, egui::Button::new("Install selected version"));
        });
    }

    fn render_mods(&self, ui: &mut egui::Ui) {
        ui.heading("Mod manager");
        ui.label("Mods belong to individual game instances.");
        ui.add_space(8.0);

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.heading("Instance mods");
            ui.label(concat!(
                "The Rust core can validate JAR archives, detect duplicate files, inspect loader ",
                "metadata, list installed files, and remove a selected mod. The Android file ",
                "picker and instance storage connection are not ready yet."
            ));
            ui.add_space(8.0);
            ui.add_enabled(false, egui::Button::new("Import mod"));
            ui.add_enabled(false, egui::Button::new("Create game instance"));
        });
    }

    fn render_accounts(&self, ui: &mut egui::Ui) {
        ui.heading("Accounts");
        ui.label("Account access is handled separately from local game files.");
        ui.add_space(8.0);

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.heading("Microsoft account");
            ui.label(concat!(
                "Microsoft sign-in and secure session storage are not implemented yet. ",
                "Offline launch is planned only for a locally available installation after ",
                "a valid account sign-in."
            ));
            ui.add_enabled(false, egui::Button::new("Sign in with Microsoft"));
        });
    }

    fn render_settings(&self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("Launcher preferences and persistent game storage.");
        ui.add_space(8.0);

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.heading("Shared KRL folder");
            ui.label(concat!(
                "The Rust storage module defines the KRL directory layout. The Android system ",
                "folder picker, persistent folder permission, and restore flow after reinstall ",
                "are not connected yet."
            ));
            ui.add_enabled(false, egui::Button::new("Choose KRL folder"));
        });

        ui.add_space(8.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.heading("Performance");
            ui.label(concat!(
                "No FPS improvement is claimed until the launcher and game are tested on ",
                "physical Android devices."
            ));
        });
    }
}

impl eframe::App for KrlApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("krl_header")
            .frame(egui::Frame::new().inner_margin(egui::Margin::same(16)))
            .show(context, |ui| self.render_header(ui));

        egui::TopBottomPanel::bottom("krl_navigation")
            .frame(egui::Frame::new().inner_margin(egui::Margin::same(12)))
            .show(context, |ui| self.render_navigation(ui));

        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin::same(16)))
            .show(context, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| match self.page {
                    Page::Play => self.render_play(ui),
                    Page::Versions => self.render_versions(ui),
                    Page::Mods => self.render_mods(ui),
                    Page::Accounts => self.render_accounts(ui),
                    Page::Settings => self.render_settings(ui),
                });
            });
    }
}
