use std::{path::Path, thread, time::Duration};
use systray::Application;

fn agent_running() -> bool {
    let paths = std::fs::read_dir("/proc").unwrap_or_default();

    for entry in paths.flatten() {
        let pid_path = entry.path();
        if !pid_path.join("cmdline").exists() {
            continue;
        }

        if let Ok(cmdline) = std::fs::read_to_string(pid_path.join("cmdline")) {
            if cmdline.contains("agent-client") {
                return true;
            }
        }
    }

    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Application::new()?;

    app.set_icon_from_file("/usr/share/icons/agent/icon-cyber-red.png")?;

    app.add_menu_item("Esci", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    let handle = app.clone();
    thread::spawn(move || loop {
        let icon = if agent_running() {
            "/usr/share/icons/agent/icon-cyber-green.png"
        } else {
            "/usr/share/icons/agent/icon-cyber-red.png"
        };

        if Path::new(icon).exists() {
            let _ = handle.set_icon_from_file(icon);
        }

        thread::sleep(Duration::from_secs(5));
    });

    app.wait_for_message()?;
    Ok(())
}
