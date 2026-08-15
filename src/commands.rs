use std::process::Command;

pub fn open_linux_settings(panel_type: &str, template: &str) {
    let is_kde = template.contains("systemsettings") || template.contains("kcmshell");

    let panel_arg = if is_kde {
        match panel_type {
            "system" => "kcm_systeminformation",
            "network" => "kcm_networkmanagement",
            "applications" => "",      // 打开主界面
            "users" => "kcm_users",
            _ => "",
        }
    } else {
        match panel_type {
            "" => "",
            "system" => "",
            "network" => "network",
            "applications" => "applications",
            "users" => "users",
            _ => "",
        }
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

pub fn execute_custom_command(command: &str) {
    if command.trim().is_empty() {
        eprintln!("命令为空");
        return;
    }
    match Command::new("sh").arg("-c").arg(command).spawn() {
        Ok(_) => println!("已启动: {}", command),
        Err(e) => eprintln!("启动失败: {} (错误: {})", command, e),
    }
}