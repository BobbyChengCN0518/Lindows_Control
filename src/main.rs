use eframe::egui::{self, FontData, FontDefinitions, TextEdit, ComboBox};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

// ==================== 配置结构体 ====================
#[derive(Serialize, Deserialize, Clone)]
struct CustomButton {
    label: String,
    command: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppConfig {
    dark_mode: bool,
    scale_factor: f32,
    cmd_template: String,
    window_width: f32,
    window_height: f32,
    font_size: f32,
    show_icons: bool,
    language: String,
    custom_buttons: Vec<CustomButton>,
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
            language: "zh".to_string(),
            custom_buttons: vec![],
        }
    }
}

// ==================== 翻译系统 ====================
type TranslationMap = HashMap<String, String>;

struct Translations {
    current_lang: String,
    maps: HashMap<String, TranslationMap>,
    available_langs: Vec<String>,
}

impl Translations {
    // 返回 String 以避免生命周期问题
    fn tr(&self, key: &str) -> String {
        self.maps
            .get(&self.current_lang)
            .and_then(|map| map.get(key))
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string())
    }

    fn available_langs(&self) -> &[String] {
        &self.available_langs
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
    custom_buttons: Vec<CustomButton>,
    config_path: PathBuf,
    translations: Translations,
    new_btn_label: String,
    new_btn_cmd: String,
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
                self.custom_buttons = config.custom_buttons.clone();
                if self.translations.maps.contains_key(&config.language) {
                    self.translations.current_lang = config.language;
                } else if !self.translations.available_langs.is_empty() {
                    self.translations.current_lang = self.translations.available_langs[0].clone();
                }
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
        self.custom_buttons = default_config.custom_buttons.clone();
        if self.translations.maps.contains_key(&default_config.language) {
            self.translations.current_lang = default_config.language;
        } else if !self.translations.available_langs.is_empty() {
            self.translations.current_lang = self.translations.available_langs[0].clone();
        }
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
            language: self.translations.current_lang.clone(),
            custom_buttons: self.custom_buttons.clone(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            let _ = fs::write(&self.config_path, json);
        }
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

    // 方便调用，返回 String
    fn tr(&self, key: &str) -> String {
        self.translations.tr(key)
    }
}

// ==================== 初始化翻译系统 ====================
const DEFAULT_ZH: &str = r#"{
    "window_title": "Lindows 控制面板",
    "main_title": "Lindows 控制面板",
    "button_system": "系统设置",
    "button_network": "网络设置",
    "button_apps": "程序和功能",
    "button_users": "用户账户",
    "button_main": "控制面板 (主界面)",
    "button_settings": "设置",
    "settings_title": "设置",
    "settings_theme": "主题",
    "settings_theme_light": "亮色",
    "settings_theme_dark": "暗色",
    "settings_scale": "界面缩放",
    "settings_font_size": "字体大小",
    "settings_window_size": "窗口大小",
    "settings_width": "宽",
    "settings_height": "高",
    "settings_restart_hint": "（更改窗口大小需要重启程序生效）",
    "settings_show_icons": "显示图标",
    "settings_cmd_template": "系统设置命令模板",
    "settings_cmd_hint": "提示：使用 {panel} 作为子面板占位符（留空表示主界面）",
    "settings_cmd_placeholder": "输入命令模板",
    "settings_preset_gnome": "GNOME",
    "settings_preset_kde": "KDE",
    "settings_preset_xfce": "XFCE",
    "settings_custom_buttons": "自定义按钮管理",
    "settings_add_label": "标签",
    "settings_add_command": "命令",
    "settings_add_button": "添加",
    "settings_no_custom": "暂无自定义按钮",
    "settings_delete": "删除",
    "settings_close": "关闭设置",
    "settings_language": "语言"
}"#;

const DEFAULT_EN: &str = r#"{
    "window_title": "Lindows Control Panel",
    "main_title": "Lindows Control Panel",
    "button_system": "System Settings",
    "button_network": "Network Settings",
    "button_apps": "Apps & Features",
    "button_users": "User Accounts",
    "button_main": "Control Panel (Main)",
    "button_settings": "Settings",
    "settings_title": "Settings",
    "settings_theme": "Theme",
    "settings_theme_light": "Light",
    "settings_theme_dark": "Dark",
    "settings_scale": "UI Scale",
    "settings_font_size": "Font Size",
    "settings_window_size": "Window Size",
    "settings_width": "Width",
    "settings_height": "Height",
    "settings_restart_hint": "(Changing window size requires restart)",
    "settings_show_icons": "Show Icons",
    "settings_cmd_template": "System Settings Command Template",
    "settings_cmd_hint": "Hint: use {panel} as panel placeholder (leave empty for main)",
    "settings_cmd_placeholder": "Enter command template",
    "settings_preset_gnome": "GNOME",
    "settings_preset_kde": "KDE",
    "settings_preset_xfce": "XFCE",
    "settings_custom_buttons": "Custom Buttons",
    "settings_add_label": "Label",
    "settings_add_command": "Command",
    "settings_add_button": "Add",
    "settings_no_custom": "No custom buttons",
    "settings_delete": "Delete",
    "settings_close": "Close Settings",
    "settings_language": "Language"
}"#;

fn init_translations() -> Translations {
    let lang_dir = PathBuf::from("lang");
    if !lang_dir.exists() {
        let _ = fs::create_dir(&lang_dir);
    }
    let zh_path = lang_dir.join("zh.json");
    if !zh_path.exists() {
        let _ = fs::write(&zh_path, DEFAULT_ZH);
    }
    let en_path = lang_dir.join("en.json");
    if !en_path.exists() {
        let _ = fs::write(&en_path, DEFAULT_EN);
    }

    let mut maps = HashMap::new();
    let mut available = Vec::new();
    if let Ok(entries) = fs::read_dir(&lang_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(lang_code) = path.file_stem().and_then(|s| s.to_str()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&content) {
                            maps.insert(lang_code.to_string(), map);
                            available.push(lang_code.to_string());
                        }
                    }
                }
            }
        }
    }

    if maps.is_empty() {
        let zh_map: HashMap<String, String> = serde_json::from_str(DEFAULT_ZH).unwrap_or_default();
        let en_map: HashMap<String, String> = serde_json::from_str(DEFAULT_EN).unwrap_or_default();
        maps.insert("zh".to_string(), zh_map);
        maps.insert("en".to_string(), en_map);
        available = vec!["zh".to_string(), "en".to_string()];
    }

    let default_lang = if available.contains(&"zh".to_string()) {
        "zh".to_string()
    } else {
        available.first().cloned().unwrap_or_else(|| "zh".to_string())
    };

    Translations {
        current_lang: default_lang,
        maps,
        available_langs: available,
    }
}

// ==================== 主函数 ====================
fn main() -> Result<(), eframe::Error> {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    let translations = init_translations();
    let mut app = ControlPanelApp {
        show_settings: false,
        dark_mode: false,
        scale_factor: 1.0,
        cmd_template: String::new(),
        window_width: 800.0,
        window_height: 480.0,
        font_size: 16.0,
        show_icons: true,
        custom_buttons: vec![],
        config_path: PathBuf::from("config.json"),
        translations,
        new_btn_label: String::new(),
        new_btn_cmd: String::new(),
    };
    app.load_config();

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

    // 注意：run_native 需要 &str，我们传递 &app.tr(...) 临时值的引用
    let title = app.tr("window_title");
    eframe::run_native(
        &title,
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

                    if ui.button(format!("{}{}", icon1, self.tr("button_system"))).clicked() {
                        open_linux_settings("system", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon2, self.tr("button_network"))).clicked() {
                        open_linux_settings("network", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon3, self.tr("button_apps"))).clicked() {
                        open_linux_settings("applications", &self.cmd_template);
                    }
                    if ui.button(format!("{}{}", icon4, self.tr("button_users"))).clicked() {
                        open_linux_settings("users", &self.cmd_template);
                    }
                    ui.end_row();

                    if ui.button(format!("{}{}", icon5, self.tr("button_main"))).clicked() {
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
                if ui.button(format!("{}{}", settings_label, self.tr("button_settings"))).clicked() {
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
                        let label = if self.dark_mode { self.tr("settings_theme_dark") } else { self.tr("settings_theme_light") };
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
                        ui.add(egui::Slider::new(&mut self.scale_factor, 0.8..=2.0).step_by(0.05));
                        if (self.scale_factor - old_scale).abs() > 0.001 {
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 字体大小
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", self.tr("settings_font_size")));
                        let old_font = self.font_size;
                        ui.add(egui::Slider::new(&mut self.font_size, 10.0..=30.0).step_by(0.5));
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
                        ui.add(egui::Slider::new(&mut self.window_width, 600.0..=1200.0).step_by(10.0));
                        if (self.window_width - old_w).abs() > 0.1 {
                            self.save_config();
                        }
                        ui.label(self.tr("settings_height"));
                        let old_h = self.window_height;
                        ui.add(egui::Slider::new(&mut self.window_height, 400.0..=900.0).step_by(10.0));
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
                            .desired_width(f32::INFINITY)
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
                            ui.add(TextEdit::singleline(&mut self.new_btn_label).hint_text("").desired_width(150.0));
                        });
                        ui.vertical(|ui| {
                            ui.label(self.tr("settings_add_command"));
                            ui.add(TextEdit::singleline(&mut self.new_btn_cmd).hint_text("").desired_width(200.0));
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

// ==================== 调用 Linux 系统设置（非阻塞） ====================
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

    match Command::new("sh").arg("-c").arg(&cmd).spawn() {
        Ok(_) => println!("已启动: {}", cmd),
        Err(e) => eprintln!("启动失败: {} (错误: {})", cmd, e),
    }
}

// ==================== 执行自定义命令（非阻塞） ====================
fn execute_custom_command(command: &str) {
    if command.trim().is_empty() {
        eprintln!("命令为空");
        return;
    }
    match Command::new("sh").arg("-c").arg(command).spawn() {
        Ok(_) => println!("已启动: {}", command),
        Err(e) => eprintln!("启动失败: {} (错误: {})", command, e),
    }
}