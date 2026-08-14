use eframe::egui::{self, FontData, FontDefinitions};

fn main() -> Result<(), eframe::Error> {
    // 强制使用 X11（避免 Wayland 兼容性问题）
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    // 1. 创建字体定义
    let mut fonts = FontDefinitions::default();

    // 2. 嵌入 STXIHEI.TTF（编译时直接打包进二进制）
    // 注意：文件放在 src/ 目录下，所以用 "./STXIHEI.TTF"；
    // 如果放在项目根目录，请改为 "../STXIHEI.TTF"
    let font_data: &'static [u8] = include_bytes!("./STXIHEI.TTF");
    fonts.font_data.insert(
        "stxihei".to_owned(),
        FontData::from_static(font_data),
    );

    // 3. 将新字体添加到字体族列表中（放在最前面，优先使用）
    let prop_family = fonts.families.entry(egui::FontFamily::Proportional).or_default();
    prop_family.insert(0, "stxihei".to_owned());

    let mono_family = fonts.families.entry(egui::FontFamily::Monospace).or_default();
    mono_family.insert(0, "stxihei".to_owned());

    // 4. 启动 eframe
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Lindows 控制面板",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            Box::new(ControlPanelApp::default())
        }),
    )
}

#[derive(Default)]
struct ControlPanelApp;

impl eframe::App for ControlPanelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lindows 控制面板");
            ui.separator();
            ui.add_space(10.0);

            // ========== 布局修改开始 ==========
            // 使用 Grid 实现每行 4 个按钮
            egui::Grid::new("button_grid")
                .spacing([10.0, 10.0])        // 水平和垂直间距
                .min_col_width(100.0)          // 每列最小宽度，让按钮更宽
                .show(ui, |ui| {
                    // 第一行：前四个按钮
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
                    ui.end_row(); // 换行

                    // 第二行：第五个按钮（可以占据整行或靠左，这里让它占第一列）
                    if ui.button("⚙️ 控制面板 (主界面)").clicked() {
                        open_control_panel_item("");
                    }
                    // 也可以在后边加空白列，但无所谓
                });
            // ========== 布局修改结束 ==========
        });
    }
}

/// 通过 cmd.exe 打开指定的控制面板项
fn open_control_panel_item(item: &str) {
    let command = if item.is_empty() {
        "control".to_string()
    } else {
        format!("control {}", item)
    };

    // 在 WSL 中调用 Windows 的 cmd.exe
    let status = std::process::Command::new("cmd.exe")
        .args(&["/c", &command])
        .status();

    match status {
        Ok(_) => println!("已打开: {}", command),
        Err(e) => eprintln!("打开失败: {}", e),
    }
}