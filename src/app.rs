use crate::config::{Config};
use hyprland::shared::HyprData;
use hyprland::data::CursorPosition;
use std::time::Duration;
use hyprland::data::Monitors;
use hyprland::dispatch::{Dispatch, DispatchType};

pub struct App;

async fn get_monitor_size(monitor_name: Option<&str>) -> anyhow::Result<(u32, u32)> {
    let monitors = Monitors::get_async().await?; // Monitors
    let monitor = if let Some(name) = monitor_name {
        monitors.into_iter().find(|m| m.name == name)
            .ok_or_else(|| anyhow::anyhow!("Monitor '{}' not found", name))?
    } else {
        monitors.into_iter().next()
            .ok_or_else(|| anyhow::anyhow!("No monitors found"))?
    };
    Ok((monitor.width as u32, monitor.height as u32))
}

impl App {
    pub async fn run(mut config: Config) -> anyhow::Result<()> {
        let (screen_width, screen_height) = get_monitor_size(config.monitor_name.as_deref()).await?;
        loop {
            let cursor = CursorPosition::get_async().await?;
            let (x, y) = (cursor.x as u32, cursor.y as u32);
            for trigger in config.triggers.iter_mut() {
                if trigger.check(x, y, screen_width, screen_height,config.sticky_ms.unwrap_or(config.timeout_ms)) {
                    println!("Dispatching {} {}", trigger.action.dispatcher, trigger.action.args);
                    
                    Dispatch::call_async(DispatchType::Custom(&trigger.action.dispatcher, &trigger.action.args)).await?;
                }
            }
            tokio::time::sleep(Duration::from_millis(config.timeout_ms)).await;
        }
    }
}