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
    cmd_template: String, // 命令模板，如 "gnome-control-center {panel}"
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            dark_mode: false,
            scale_factor: 1.0,
            cmd_template: "gnome-control-center {panel}".to_string(),
        }
    }
}

// ==================== 应用状态 ====================
struct ControlPanelApp {
    show_settings: bool,
    dark_mode: bool,
    scale_factor: f32,
    cmd_template: String,
    config_path: PathBuf,
}

impl ControlPanelApp {
    fn load_config(&mut self) {
        if let Ok(content) = fs::read_to_string(&self.config_path) {
            if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                self.dark_mode = config.dark_mode;
                self.scale_factor = config.scale_factor;
                self.cmd_template = config.cmd_template;
                return;
            }
        }
        // 读取失败则使用默认值并保存
        let default_config = AppConfig::default();
        self.dark_mode = default_config.dark_mode;
        self.scale_factor = default_config.scale_factor;
        self.cmd_template = default_config.cmd_template;
        self.save_config();
    }

    fn save_config(&self) {
        let config = AppConfig {
            dark_mode: self.dark_mode,
            scale_factor: self.scale_factor,
            cmd_template: self.cmd_template.clone(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&config) {
            let _ = fs::write(&self.config_path, json);
        }
    }

    // 设置预设命令模板
    fn set_preset(&mut self, preset: &str) {
        self.cmd_template = match preset {
            "gnome" => "gnome-control-center {panel}".to_string(),
            "kde" => "systemsettings5 {panel}".to_string(),
            "xfce" => "xfce4-settings-manager".to_string(), // XFCE 不支持子面板，不加占位符
            _ => return,
        };
        self.save_config();
    }
}

impl Default for ControlPanelApp {
    fn default() -> Self {
        let config_path = PathBuf::from("config.json");
        let mut app = Self {
            show_settings: false,
            dark_mode: false,
            scale_factor: 1.0,
            cmd_template: String::new(),
            config_path,
        };
        app.load_config();
        app
    }
}

// ==================== 主函数 ====================
fn main() -> Result<(), eframe::Error> {
    // 强制使用 X11（避免 Wayland 兼容性问题）
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    // 字体配置
    let mut fonts = FontDefinitions::default();
    // 将 STXIHEI.TTF 放在 src/ 目录下
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

    // 窗口选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(800.0, 480.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Lindows 控制面板",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            cc.egui_ctx.set_style(egui::Style {
                visuals: egui::Visuals::light(),
                ..Default::default()
            });
            Box::new(ControlPanelApp::default())
        }),
    )
}

// ==================== 界面逻辑 ====================
impl eframe::App for ControlPanelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 防御：缩放不能为 0
        if self.scale_factor <= 0.0 {
            self.scale_factor = 1.0;
        }

        // 应用主题
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

        // 主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(16.0, 10.0);

            ui.heading("Lindows 控制面板");
            ui.separator();
            ui.add_space(15.0);

            // ---- 功能按钮网格（一行四个） ----
            egui::Grid::new("button_grid")
                .spacing([20.0, 15.0])
                .min_col_width(170.0)
                .show(ui, |ui| {
                    if ui.button("🖥️ 系统设置").clicked() {
                        open_linux_settings("system", &self.cmd_template);
                    }
                    if ui.button("🌐 网络设置").clicked() {
                        open_linux_settings("network", &self.cmd_template);
                    }
                    if ui.button("📦 程序和功能").clicked() {
                        open_linux_settings("applications", &self.cmd_template);
                    }
                    if ui.button("👤 用户账户").clicked() {
                        open_linux_settings("users", &self.cmd_template);
                    }
                    ui.end_row();

                    if ui.button("⚙️ 控制面板 (主界面)").clicked() {
                        open_linux_settings("", &self.cmd_template);
                    }
                    // 填充剩余列
                    for _ in 0..3 {
                        ui.label("");
                    }
                });

            ui.add_space(20.0);

            // ---- 设置按钮 ----
            ui.horizontal_centered(|ui| {
                if ui.button("⚙️ 设置").clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
        });

        // ---- 设置浮动窗口 ----
        if self.show_settings {
            egui::Window::new("设置")
                .resizable(false)
                .collapsible(false)
                .default_size(egui::vec2(400.0, 300.0))
                .show(ctx, |ui| {
                    ui.heading("设置");
                    ui.separator();
                    ui.add_space(10.0);

                    // 主题切换
                    ui.horizontal(|ui| {
                        ui.label("主题：");
                        let label = if self.dark_mode { "暗色" } else { "亮色" };
                        if ui.button(label).clicked() {
                            self.dark_mode = !self.dark_mode;
                            self.save_config();
                        }
                    });

                    ui.add_space(10.0);

                    // 缩放滑块
                    ui.horizontal(|ui| {
                        ui.label("界面缩放：");
                        let old_scale = self.scale_factor;
                        ui.add(egui::Slider::new(&mut self.scale_factor, 0.8..=2.0).step_by(0.05));
                        if (self.scale_factor - old_scale).abs() > 0.001 {
                            self.save_config();
                        }
                    });

                    ui.add_space(15.0);

                    // ---- 命令模板设置 ----
                    ui.label("系统设置命令模板：");
                    ui.label("提示：使用 {panel} 作为子面板占位符（留空表示主界面）");
                    ui.add_space(5.0);

                    // 文本框编辑命令模板
                    let mut tmp = self.cmd_template.clone();
                    let response = ui.add(TextEdit::singleline(&mut tmp).hint_text("输入命令模板"));
                    if response.changed() {
                        self.cmd_template = tmp;
                        self.save_config();
                    }

                    ui.add_space(10.0);

                    // 预设按钮
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

                    ui.add_space(10.0);
                    ui.separator();
                    if ui.button("关闭设置").clicked() {
                        self.show_settings = false;
                    }
                });
        }
    }
}

// ==================== 调用 Linux 系统设置 ====================
/// 根据命令模板和面板类型执行对应的系统设置命令
/// panel_type: "", "system", "network", "applications", "users"
fn open_linux_settings(panel_type: &str, template: &str) {
    // 将面板类型转换为子参数
    let panel_arg = match panel_type {
        "" => "",                          // 主界面，无参数
        "system" => "",
        "network" => "network",
        "applications" => "applications",
        "users" => "users",
        _ => "",
    };

    // 替换模板中的 {panel}
    let cmd = template.replace("{panel}", panel_arg).trim().to_string();
    if cmd.is_empty() {
        eprintln!("命令模板为空，无法执行");
        return;
    }

    // 执行命令（通过 sh -c 以支持复杂命令）
    let status = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .status();

    match status {
        Ok(_) => println!("已执行: {}", cmd),
        Err(e) => eprintln!("执行失败: {} (错误: {})", cmd, e),
    }
}