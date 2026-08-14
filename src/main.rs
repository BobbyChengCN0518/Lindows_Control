use eframe::egui::{self, FontData, FontDefinitions, TextEdit};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

// ==================== 配置结构体 ====================
#[derive(Serialize, Deserialize, Clone)]
struct AppConfig {
    dark_mode: bool,
    scale_factor: f32,
    cmd_template: String,
    window_width: f32,
    window_height: f32,
    font_size: f32,
    show_icons: bool,
    button_labels: ButtonLabels,
}

#[derive(Serialize, Deserialize, Clone)]
struct ButtonLabels {
    system: String,
    network: String,
    apps: String,
    users: String,
    main: String,
    settings: String,
}

impl Default for ButtonLabels {
    fn default() -> Self {
        Self {
            system: "系统设置".to_string(),
            network: "网络设置".to_string(),
            apps: "程序和功能".to_string(),
            users: "用户账户".to_string(),
            main: "控制面板 (主界面)".to_string(),
            settings: "设置".to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            dark_mode: false,
            scale_factor: 1.0,
            cmd_template: "gnome-control-center {panel}".to_string(),
            window_width: 800.0,
            window_height: 480.0,
            font_size: 16.0,
            show_icons: true,
            button_labels: ButtonLabels::default(),
        }
    }
}

// ==================== 应用状态 ====================
struct ControlPanelApp {
    show_settings: bool,
    dark_mode: bool,
    scale_factor: f32,
    cmd_template: String,
    window_width: f32,
    window_height: f32,
    font_size: f32,
    show_icons: bool,
    button_labels: ButtonLabels,
    config_path: PathBuf,
    temp_labels: ButtonLabels,
}

impl ControlPanelApp {
    fn load_config(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.config_path) {
            if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                self.dark_mode = config.dark_mode;
                self.scale_factor = config.scale_factor;
                self.cmd_template = config.cmd_template;
                self.window_width = config.window_width;
                self.window_height = config.window_height;
                self.font_size = config.font_size;
                self.show_icons = config.show_icons;
                self.button_labels = config.button_labels.clone();
                self.temp_labels = config.button_labels.clone();
                return;
            }
        }
        let default_config = AppConfig::default();
        self.dark_mode = default_config.dark_mode;
        self.scale_factor = default_config.scale_factor;
        self.cmd_template = default_config.cmd_template;
        self.window_width = default_config.window_width;
        self.window_height = default_config.window_height;
        self.font_size = default_config.font_size;
        self.show_icons = default_config.show_icons;
        self.button_labels = default_config.button_labels.clone();
        self.temp_labels = default_config.button_labels.clone();
        self.save_config();
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
            button_labels: self.button_labels.clone(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            let _ = fs::write(&self.config_path, json);
        }
    }

    fn apply_button_labels(&mut self) {
        self.button_labels = self.temp_labels.clone();
        self.save_config();
    }

    fn set_preset(&mut self, preset: &str) {
        self.cmd_template = match preset {
            "gnome" => "gnome-control-center {panel}".to_string(),
            "kde" => "systemsettings5 {panel}".to_string(),
            "xfce" => "xfce4-settings-manager".to_string(),
            _ => return,
        };
        self.save_config();
    }
}

impl Default for ControlPanelApp {
    fn default() -> Self {
        let config_path = PathBuf::from("config.json");
        let default_labels = ButtonLabels::default();
        let mut app = Self {
            show_settings: false,
            dark_mode: false,
            scale_factor: 1.0,
            cmd_template: String::new(),
            window_width: 800.0,
            window_height: 480.0,
            font_size: 16.0,
            show_icons: true,
            button_labels: default_labels.clone(),
            temp_labels: default_labels,
            config_path,
        };
        app.load_config();
        app
    }
}

// ==================== 主函数 ====================
fn main() -> Result<(), eframe::Error> {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    let app = ControlPanelApp::default();
    let window_size = egui::vec2(app.window_width, app.window_height);

    let mut fonts = FontDefinitions::default();
    let font_data: &'static [u8] = include_bytes!("./STXIHEI.TTF");
    fonts.font_data.insert(
        "stxihei".to_owned(),
        FontData::from_static(font_data),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "stxihei".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "stxihei".to_owned());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(window_size),
        ..Default::default()
    };

    eframe::run_native(
        "Lindows 控制面板",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.set_style(egui::Style {
                visuals: if app.dark_mode { egui::Visuals::dark() } else { egui::Visuals::light() },
                ..Default::default()
            });
            Box::new(app)
        }),
    )
}

// ==================== 界面逻辑 ====================
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
        ].into();
        ctx.set_style(style_override);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(16.0, 10.0);

            ui.heading("Lindows 控制面板");
            ui.separator();
            ui.add_space(15.0);

            egui::Grid::new("button_grid")
                .spacing([20.0, 15.0])
                .min_col_width(170.0)
                .show(ui, |ui| {
                    let icon1 = if self.show_icons { "🖥️ " } else { "" };
                    let icon2 = if self.show_icons { "🌐 " } else { "" };
                    let icon3 = if self.show_icons { "📦 " } else { "" };
                    let icon4 = if self.show_icons { "👤 " } else { "" };
                    let icon5 = if self.show_icons { "⚙️ " } else { "" };

                    if ui.button(format!("{}{}", icon1, self.button_labels.system)).clicked() {
                        open_linux_settings("system", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon2, self.button_labels.network)).clicked() {
                        open_linux_settings("network", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon3, self.button_labels.apps)).clicked() {
                        open_linux_settings("applications", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon4, self.button_labels.users)).clicked() {
                        open_linux_settings("users", &self.cmd_template);
                    }
                    ui.end_row();

                    if ui.button(format!("{}{}", icon5, self.button_labels.main)).clicked() {
                        open_linux_settings("", &self.cmd_template);
                    }
                    for _ in 0..3 {
                        ui.label("");
                    }
                });

            ui.add_space(20.0);

            ui.horizontal_centered(|ui| {
                let settings_label = if self.show_icons { "⚙️ " } else { "" };
                if ui.button(format!("{}{}", settings_label, self.button_labels.settings)).clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
        });

        // ========== 设置窗口（加宽版） ==========
        if self.show_settings {
            egui::Window::new("设置")
                .resizable(false)
                .collapsible(false)
                .default_size(egui::vec2(550.0, 500.0))  // 宽度增加到 550
                .show(ctx, |ui| {
                    ui.heading("设置");
                    ui.separator();
                    ui.add_space(10.0);

                    // 主题
                    ui.horizontal(|ui| {
                        ui.label("主题：");
                        let label = if self.dark_mode { "暗色" } else { "亮色" };
                        if ui.button(label).clicked() {
                            self.dark_mode = !self.dark_mode;
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 缩放
                    ui.horizontal(|ui| {
                        ui.label("界面缩放：");
                        let old_scale = self.scale_factor;
                        ui.add(egui::Slider::new(&mut self.scale_factor, 0.8..=2.0).step_by(0.05));
                        if (self.scale_factor - old_scale).abs() > 0.001 {
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 字体大小
                    ui.horizontal(|ui| {
                        ui.label("字体大小：");
                        let old_font = self.font_size;
                        ui.add(egui::Slider::new(&mut self.font_size, 10.0..=30.0).step_by(0.5));
                        if (self.font_size - old_font).abs() > 0.001 {
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 窗口大小
                    ui.label("窗口大小：");
                    ui.horizontal(|ui| {
                        ui.label("宽");
                        let old_w = self.window_width;
                        ui.add(egui::Slider::new(&mut self.window_width, 600.0..=1200.0).step_by(10.0));
                        if (self.window_width - old_w).abs() > 0.1 {
                            self.save_config();
                        }
                        ui.label("高");
                        let old_h = self.window_height;
                        ui.add(egui::Slider::new(&mut self.window_height, 400.0..=900.0).step_by(10.0));
                        if (self.window_height - old_h).abs() > 0.1 {
                            self.save_config();
                        }
                    });
                    ui.label("（更改窗口大小需要重启程序生效）");

                    ui.add_space(10.0);

                    // 显示图标
                    ui.horizontal(|ui| {
                        let mut show = self.show_icons;
                        ui.checkbox(&mut show, "显示图标");
                        if show != self.show_icons {
                            self.show_icons = show;
                            self.save_config();
                        }
                    });

                    ui.add_space(15.0);

                    // ---- 命令模板（加宽文本框） ----
                    ui.label("系统设置命令模板：");
                    ui.label("提示：使用 {panel} 作为子面板占位符（留空表示主界面）");
                    ui.add_space(5.0);
                    let mut tmp = self.cmd_template.clone();
                    // 让文本框占满宽度
                    let response = ui.add(
                        TextEdit::singleline(&mut tmp)
                            .hint_text("输入命令模板")
                            .desired_width(f32::INFINITY)  // 拉伸到最宽
                    );
                    if response.changed() {
                        self.cmd_template = tmp;
                        self.save_config();
                    }

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("GNOME").clicked() {
                            self.set_preset("gnome");
                        }
                        if ui.button("KDE").clicked() {
                            self.set_preset("kde");
                        }
                        if ui.button("XFCE").clicked() {
                            self.set_preset("xfce");
                        }
                    });

                    ui.add_space(15.0);

                    // ---- 按钮文本自定义（加宽文本框） ----
                    ui.label("自定义按钮文本：");
                    ui.add_space(5.0);
                    let mut labels_changed = false;

                    let sys = &mut self.temp_labels.system;
                    if ui.add(TextEdit::singleline(sys).hint_text("系统设置").desired_width(f32::INFINITY)).changed() { labels_changed = true; }
                    let net = &mut self.temp_labels.network;
                    if ui.add(TextEdit::singleline(net).hint_text("网络设置").desired_width(f32::INFINITY)).changed() { labels_changed = true; }
                    let apps = &mut self.temp_labels.apps;
                    if ui.add(TextEdit::singleline(apps).hint_text("程序和功能").desired_width(f32::INFINITY)).changed() { labels_changed = true; }
                    let users = &mut self.temp_labels.users;
                    if ui.add(TextEdit::singleline(users).hint_text("用户账户").desired_width(f32::INFINITY)).changed() { labels_changed = true; }
                    let main = &mut self.temp_labels.main;
                    if ui.add(TextEdit::singleline(main).hint_text("控制面板主界面").desired_width(f32::INFINITY)).changed() { labels_changed = true; }
                    let settings = &mut self.temp_labels.settings;
                    if ui.add(TextEdit::singleline(settings).hint_text("设置").desired_width(f32::INFINITY)).changed() { labels_changed = true; }

                    if labels_changed {
                        // 等待用户点击"应用"
                    }

                    ui.add_space(5.0);
                    if ui.button("应用按钮文本").clicked() {
                        self.apply_button_labels();
                    }

                    ui.add_space(15.0);
                    ui.separator();
                    if ui.button("关闭设置").clicked() {
                        self.show_settings = false;
                    }
                });
        }
    }
}

// ==================== 调用 Linux 系统设置 ====================
fn open_linux_settings(panel_type: &str, template: &str) {
    let panel_arg = match panel_type {
        "" => "",
        "system" => "",
        "network" => "network",
        "applications" => "applications",
        "users" => "users",
        _ => "",
    };

    let cmd = template.replace("{panel}", panel_arg).trim().to_string();
    if cmd.is_empty() {
        eprintln!("命令模板为空，无法执行");
        return;
    }

    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(_) => println!("已执行: {}", cmd),
        Err(e) => eprintln!("执行失败: {} (错误: {})", cmd, e),
    }
}