use crate::config::{self, AppConfig, CustomButton};
use crate::commands::{execute_custom_command, open_linux_settings};
use crate::translations::Translations;
use eframe::egui::{self, ComboBox, TextEdit};
use std::path::PathBuf;

pub struct ControlPanelApp {
    pub show_settings: bool,
    pub dark_mode: bool,
    pub scale_factor: f32,
    pub cmd_template: String,
    pub window_width: f32,
    pub window_height: f32,
    pub font_size: f32,
    pub show_icons: bool,
    pub custom_buttons: Vec<CustomButton>,
    config_path: PathBuf,
    translations: Translations,
    new_btn_label: String,
    new_btn_cmd: String,
}

impl ControlPanelApp {
    pub fn new() -> Self {
        let config_path = PathBuf::from("config.json");
        let translations = crate::translations::init_translations();
        let mut app = Self {
            show_settings: false,
            dark_mode: false,
            scale_factor: 1.0,
            cmd_template: String::new(),
            window_width: 800.0,
            window_height: 480.0,
            font_size: 16.0,
            show_icons: true,
            custom_buttons: vec![],
            config_path,
            translations,
            new_btn_label: String::new(),
            new_btn_cmd: String::new(),
        };
        app.load_config();
        app
    }

    fn load_config(&mut self) {
        let config = config::load_config(&self.config_path);
        self.dark_mode = config.dark_mode;
        self.scale_factor = config.scale_factor;
        self.cmd_template = config.cmd_template;
        self.window_width = config.window_width;
        self.window_height = config.window_height;
        self.font_size = config.font_size;
        self.show_icons = config.show_icons;
        self.custom_buttons = config.custom_buttons.clone();
        if self.translations.maps.contains_key(&config.language) {
            self.translations.current_lang = config.language;
        } else if !self.translations.available_langs.is_empty() {
            self.translations.current_lang = self.translations.available_langs[0].clone();
        }
    }

    fn save_config(&self) {
        let config = AppConfig {
            dark_mode: self.dark_mode,
            scale_factor: self.scale_factor,
            cmd_template: self.cmd_template.clone(),
            window_width: self.window_width,
            window_height: self.window_height,
            font_size: self.font_size,
            show_icons: self.show_icons,
            language: self.translations.current_lang.clone(),
            custom_buttons: self.custom_buttons.clone(),
        };
        config::save_config(&self.config_path, &config);
    }

    pub fn tr(&self, key: &str) -> String {
        self.translations.tr(key)
    }

    fn set_preset(&mut self, preset: &str) {
        self.cmd_template = match preset {
            "gnome" => "gnome-control-center {panel}".to_string(),
            "kde" => "systemsettings {panel}".to_string(),      // KDE 使用 systemsettings（不带数字）
            "xfce" => "xfce4-settings-manager".to_string(),
            _ => return,
        };
        self.save_config();
    }

    fn add_custom_button(&mut self) {
        if self.new_btn_label.trim().is_empty() || self.new_btn_cmd.trim().is_empty() {
            return;
        }
        self.custom_buttons.push(CustomButton {
            label: self.new_btn_label.trim().to_string(),
            command: self.new_btn_cmd.trim().to_string(),
        });
        self.new_btn_label.clear();
        self.new_btn_cmd.clear();
        self.save_config();
    }

    fn remove_custom_button(&mut self, index: usize) {
        if index < self.custom_buttons.len() {
            self.custom_buttons.remove(index);
            self.save_config();
        }
    }
}

impl eframe::App for ControlPanelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.scale_factor <= 0.0 {
            self.scale_factor = 1.0;
        }

        let style = if self.dark_mode {
            egui::Style {
                visuals: egui::Visuals::dark(),
                ..Default::default()
            }
        } else {
            egui::Style {
                visuals: egui::Visuals::light(),
                ..Default::default()
            }
        };
        ctx.set_style(style);
        ctx.set_pixels_per_point(self.scale_factor);

        let mut style_override = (*ctx.style()).clone();
        style_override.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::proportional(self.font_size * 1.5)),
            (egui::TextStyle::Body, egui::FontId::proportional(self.font_size)),
            (egui::TextStyle::Button, egui::FontId::proportional(self.font_size)),
            (egui::TextStyle::Monospace, egui::FontId::monospace(self.font_size * 0.9)),
            (egui::TextStyle::Small, egui::FontId::proportional(self.font_size * 0.8)),
        ]
            .into();
        ctx.set_style(style_override);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(16.0, 10.0);

            ui.heading(self.tr("main_title"));
            ui.separator();
            ui.add_space(15.0);

            // ---- 固定按钮网格 ----
            egui::Grid::new("button_grid")
                .spacing([20.0, 15.0])
                .min_col_width(170.0)
                .show(ui, |ui| {
                    let icon1 = if self.show_icons { "🖥️ " } else { "" };
                    let icon2 = if self.show_icons { "🌐 " } else { "" };
                    let icon3 = if self.show_icons { "📦 " } else { "" };
                    let icon4 = if self.show_icons { "👤 " } else { "" };
                    let icon5 = if self.show_icons { "⚙️ " } else { "" };

                    if ui.button(format!("{}{}", icon1, self.tr("button_system")))
                        .clicked()
                    {
                        open_linux_settings("system", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon2, self.tr("button_network")))
                        .clicked()
                    {
                        open_linux_settings("network", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon3, self.tr("button_apps")))
                        .clicked()
                    {
                        open_linux_settings("applications", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon4, self.tr("button_users")))
                        .clicked()
                    {
                        open_linux_settings("users", &self.cmd_template);
                    }
                    ui.end_row();

                    if ui.button(format!("{}{}", icon5, self.tr("button_main")))
                        .clicked()
                    {
                        open_linux_settings("", &self.cmd_template);
                    }
                    for _ in 0..3 {
                        ui.label("");
                    }
                });

            // ---- 自定义按钮区域 ----
            if !self.custom_buttons.is_empty() {
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);

                egui::Grid::new("custom_button_grid")
                    .spacing([20.0, 15.0])
                    .min_col_width(170.0)
                    .show(ui, |ui| {
                        let mut col_count = 0;
                        for btn in &self.custom_buttons {
                            let label = if self.show_icons { "🔹 " } else { "" };
                            let full_label = format!("{}{}", label, btn.label);
                            if ui.button(full_label).clicked() {
                                execute_custom_command(&btn.command);
                            }
                            col_count += 1;
                            if col_count % 4 == 0 {
                                ui.end_row();
                            }
                        }
                        let remaining = (4 - col_count % 4) % 4;
                        for _ in 0..remaining {
                            ui.label("");
                        }
                    });
            }

            ui.add_space(20.0);

            // ---- 设置按钮 ----
            ui.horizontal_centered(|ui| {
                let settings_label = if self.show_icons { "⚙️ " } else { "" };
                if ui
                    .button(format!("{}{}", settings_label, self.tr("button_settings")))
                    .clicked()
                {
                    self.show_settings = !self.show_settings;
                }
            });
        });

        // ---- 设置窗口 ----
        if self.show_settings {
            egui::Window::new(self.tr("settings_title"))
                .resizable(false)
                .collapsible(false)
                .default_size(egui::vec2(550.0, 750.0))
                .show(ctx, |ui| {
                    ui.heading(self.tr("settings_title"));
                    ui.separator();
                    ui.add_space(10.0);

                    // ---- 语言选择 ----
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", self.tr("settings_language")));
                        let mut lang = self.translations.current_lang.clone();
                        ComboBox::from_id_source("language_combobox")
                            .selected_text(&lang)
                            .show_ui(ui, |ui| {
                                for code in self.translations.available_langs() {
                                    ui.selectable_value(&mut lang, code.clone(), code);
                                }
                            });
                        if lang != self.translations.current_lang {
                            if self.translations.maps.contains_key(&lang) {
                                self.translations.current_lang = lang;
                                self.save_config();
                            }
                        }
                    });

                    ui.add_space(10.0);

                    // 主题
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", self.tr("settings_theme")));
                        let label = if self.dark_mode {
                            self.tr("settings_theme_dark")
                        } else {
                            self.tr("settings_theme_light")
                        };
                        if ui.button(label).clicked() {
                            self.dark_mode = !self.dark_mode;
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 缩放
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", self.tr("settings_scale")));
                        let old_scale = self.scale_factor;
                        ui.add(
                            egui::Slider::new(&mut self.scale_factor, 0.8..=2.0).step_by(0.05),
                        );
                        if (self.scale_factor - old_scale).abs() > 0.001 {
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 字体大小
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", self.tr("settings_font_size")));
                        let old_font = self.font_size;
                        ui.add(
                            egui::Slider::new(&mut self.font_size, 10.0..=30.0).step_by(0.5),
                        );
                        if (self.font_size - old_font).abs() > 0.001 {
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 窗口大小
                    ui.label(format!("{}:", self.tr("settings_window_size")));
                    ui.horizontal(|ui| {
                        ui.label(self.tr("settings_width"));
                        let old_w = self.window_width;
                        ui.add(
                            egui::Slider::new(&mut self.window_width, 600.0..=1200.0)
                                .step_by(10.0),
                        );
                        if (self.window_width - old_w).abs() > 0.1 {
                            self.save_config();
                        }
                        ui.label(self.tr("settings_height"));
                        let old_h = self.window_height;
                        ui.add(
                            egui::Slider::new(&mut self.window_height, 400.0..=900.0)
                                .step_by(10.0),
                        );
                        if (self.window_height - old_h).abs() > 0.1 {
                            self.save_config();
                        }
                    });
                    ui.label(self.tr("settings_restart_hint"));

                    ui.add_space(10.0);

                    // 显示图标
                    ui.horizontal(|ui| {
                        let mut show = self.show_icons;
                        ui.checkbox(&mut show, self.tr("settings_show_icons"));
                        if show != self.show_icons {
                            self.show_icons = show;
                            self.save_config();
                        }
                    });

                    ui.add_space(15.0);

                    // ---- 命令模板 ----
                    ui.label(format!("{}:", self.tr("settings_cmd_template")));
                    ui.label(self.tr("settings_cmd_hint"));
                    ui.add_space(5.0);
                    let mut tmp = self.cmd_template.clone();
                    let response = ui.add(
                        TextEdit::singleline(&mut tmp)
                            .hint_text(self.tr("settings_cmd_placeholder"))
                            .desired_width(f32::INFINITY),
                    );
                    if response.changed() {
                        self.cmd_template = tmp;
                        self.save_config();
                    }

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button(self.tr("settings_preset_gnome")).clicked() {
                            self.set_preset("gnome");
                        }
                        if ui.button(self.tr("settings_preset_kde")).clicked() {
                            self.set_preset("kde");
                        }
                        if ui.button(self.tr("settings_preset_xfce")).clicked() {
                            self.set_preset("xfce");
                        }
                    });

                    ui.add_space(15.0);

                    // ---- 自定义按钮管理 ----
                    ui.label(format!("{}:", self.tr("settings_custom_buttons")));
                    ui.separator();

                    // 添加新按钮
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(self.tr("settings_add_label"));
                            ui.add(
                                TextEdit::singleline(&mut self.new_btn_label)
                                    .hint_text("")
                                    .desired_width(150.0),
                            );
                        });
                        ui.vertical(|ui| {
                            ui.label(self.tr("settings_add_command"));
                            ui.add(
                                TextEdit::singleline(&mut self.new_btn_cmd)
                                    .hint_text("")
                                    .desired_width(200.0),
                            );
                        });
                        if ui.button(self.tr("settings_add_button")).clicked() {
                            self.add_custom_button();
                        }
                    });

                    ui.add_space(10.0);

                    // 已有按钮列表
                    if self.custom_buttons.is_empty() {
                        ui.label(self.tr("settings_no_custom"));
                    } else {
                        let buttons = self.custom_buttons.clone();
                        egui::Grid::new("custom_list")
                            .spacing([10.0, 5.0])
                            .show(ui, |ui| {
                                for (i, btn) in buttons.iter().enumerate() {
                                    ui.label(&btn.label);
                                    ui.label(&btn.command);
                                    if ui.button(self.tr("settings_delete")).clicked() {
                                        self.remove_custom_button(i);
                                    }
                                    ui.end_row();
                                }
                            });
                    }

                    ui.add_space(15.0);
                    ui.separator();
                    if ui.button(self.tr("settings_close")).clicked() {
                        self.show_settings = false;
                    }
                });
        }
    }
}