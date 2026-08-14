use eframe::egui::{self, FontData, FontDefinitions};

fn main() -> Result<(), eframe::Error> {
    // 强制使用 X11（避免 Wayland 兼容性问题）
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    // 1. 创建字体定义
    let mut fonts = FontDefinitions::default();

    // 2. 嵌入 STXIHEI.TTF（编译时直接打包进二进制）
    // 注意：请确保 STXIHEI.TTF 文件位于 src/ 目录下
    let font_data: &'static [u8] = include_bytes!("./STXIHEI.TTF");
    fonts.font_data.insert(
        "stxihei".to_owned(),
        FontData::from_static(font_data),
    );

    // 3. 将新字体添加到字体族列表中（放在最前面）
    let prop_family = fonts.families.entry(egui::FontFamily::Proportional).or_default();
    prop_family.insert(0, "stxihei".to_owned());

    let mono_family = fonts.families.entry(egui::FontFamily::Monospace).or_default();
    mono_family.insert(0, "stxihei".to_owned());

    // 4. 启动 eframe
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

// ==================== 应用状态 ====================
struct ControlPanelApp {
    show_settings: bool,
    dark_mode: bool,
    scale_factor: f32,
}

// 手动实现 Default，使 scale_factor 默认为 1.0
impl Default for ControlPanelApp {
    fn default() -> Self {
        Self {
            show_settings: false,
            dark_mode: false,
            scale_factor: 1.0,
        }
    }
}

impl eframe::App for ControlPanelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 防御：确保 scale_factor 大于 0
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

        // 应用缩放
        ctx.set_pixels_per_point(self.scale_factor);

        // 主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(16.0, 10.0);

            ui.heading("Lindows 控制面板");
            ui.separator();
            ui.add_space(15.0);

            // 控制项网格（一行四个）
            egui::Grid::new("button_grid")
                .spacing([20.0, 15.0])
                .min_col_width(170.0)
                .show(ui, |ui| {
                    if ui.button("🖥️ 系统设置").clicked() {
                        open_control_panel_item("system");
                    }
                    if ui.button("🌐 网络设置").clicked() {
                        open_control_panel_item("network");
                    }
                    if ui.button("📦 程序和功能").clicked() {
                        open_control_panel_item("appwiz.cpl");
                    }
                    if ui.button("👤 用户账户").clicked() {
                        open_control_panel_item("useraccounts");
                    }
                    ui.end_row();

                    if ui.button("⚙️ 控制面板 (主界面)").clicked() {
                        open_control_panel_item("");
                    }
                    // 填充剩余列（保持布局）
                    for _ in 0..3 {
                        ui.label("");
                    }
                });

            ui.add_space(20.0);

            // 设置按钮
            ui.horizontal_centered(|ui| {
                if ui.button("⚙️ 设置").clicked() {
                    self.show_settings = !self.show_settings;
                }
            });
        });

        // 设置浮动窗口
        if self.show_settings {
            egui::Window::new("设置")
                .resizable(false)
                .collapsible(false)
                .default_size(egui::vec2(300.0, 180.0))
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
                        }
                    });

                    ui.add_space(10.0);

                    // 缩放滑块
                    ui.horizontal(|ui| {
                        ui.label("界面缩放：");
                        ui.add(egui::Slider::new(&mut self.scale_factor, 0.8..=2.0).step_by(0.05));
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

// ==================== 调用 Windows 控制面板 ====================
fn open_control_panel_item(item: &str) {
    let command = if item.is_empty() {
        "control".to_string()
    } else {
        format!("control {}", item)
    };

    let status = std::process::Command::new("cmd.exe")
        .args(&["/c", &command])
        .status();

    match status {
        Ok(_) => println!("已打开: {}", command),
        Err(e) => eprintln!("打开失败: {}", e),
    }
}