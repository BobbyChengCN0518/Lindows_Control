use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type TranslationMap = HashMap<String, String>;

pub struct Translations {
    pub current_lang: String,
    pub maps: HashMap<String, TranslationMap>,
    pub available_langs: Vec<String>,
}

impl Translations {
    pub fn tr(&self, key: &str) -> String {
        self.maps
            .get(&self.current_lang)
            .and_then(|map| map.get(key))
            .map(|s| s.to_string())
            .unwrap_or_else(|| key.to_string())
    }

    pub fn available_langs(&self) -> &[String] {
        &self.available_langs
    }
}

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

pub fn init_translations() -> Translations {
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