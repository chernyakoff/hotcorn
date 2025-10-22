use serde::{Serialize, Deserialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub monitor_name: Option<String>,
    pub triggers: Vec<Trigger>,
    pub timeout_ms: u64,
    pub sticky_ms: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trigger {
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,      // Enum вместо строки
    pub position: Option<Position>,     // Enum вместо строки для Corner и Edge
    pub radius: Option<u32>,            // для Corner
    pub width: Option<u32>,             // для Edge и Rect
    pub height: Option<u32>,            // для Edge и Rect
    pub x: Option<u32>,                 // для Rect
    pub y: Option<u32>,                 // для Rect
    pub action: Action,

    #[serde(skip)]
    pub last_trigger: Option<Instant>,
    #[serde(skip)]
    pub inside: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub dispatcher: String,
    pub args: String,
}

// Тип триггера
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerType {
    Corner,
    Edge,
    Rect,
}

// Позиции для Corner и Edge
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum Position {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}


impl Default for Config { fn default() -> Self { 
    Self { 
        triggers: vec![],
        timeout_ms: 30, 
        sticky_ms: Some(300),
        monitor_name: Some("eDP-1".to_string()),
 } } }


// --- загрузка конфигурации с фильтрацией дубликатов ---
pub fn load_config() -> Result<Config, confy::ConfyError> {
    let mut cfg: Config = confy::load(env!("CARGO_PKG_NAME"), Some("config"))?;

    use std::collections::HashSet;
    let mut corners_used = HashSet::new();
    let mut edges_used = HashSet::new();

    cfg.triggers.retain(|t| match t.trigger_type {
        TriggerType::Corner => {
            if let Some(pos) = &t.position {
                if corners_used.contains(pos) {
                    false
                } else {
                    corners_used.insert(pos.clone());
                    true
                }
            } else { false }
        }
        TriggerType::Edge => {
            if let Some(pos) = &t.position {
                if edges_used.contains(pos) {
                    false
                } else {
                    edges_used.insert(pos.clone());
                    true
                }
            } else { false }
        }
        TriggerType::Rect => true,
    });

    Ok(cfg)
}


impl Trigger {
    pub fn check(
        &mut self,
        cursor_x: u32,
        cursor_y: u32,
        screen_width: u32,
        screen_height: u32,
        timeout_ms: u64,
    ) -> bool {
        let now = Instant::now();
        let mut in_zone = false;

        match self.trigger_type {
            TriggerType::Corner => {
                let radius = self.radius.unwrap_or(0);
                match self.position.as_ref().unwrap() {
                    Position::TopLeft => in_zone = cursor_x < radius && cursor_y < radius,
                    Position::TopRight => in_zone = cursor_x > screen_width - radius && cursor_y < radius,
                    Position::BottomLeft => in_zone = cursor_x < radius && cursor_y > screen_height - radius,
                    Position::BottomRight => in_zone = cursor_x > screen_width - radius && cursor_y > screen_height - radius,
                    _ => {}
                }
            }
            TriggerType::Edge => {
                let width = self.width.unwrap_or(0);
                let height = self.height.unwrap_or(0);
                match self.position.as_ref().unwrap() {
                    Position::Top => in_zone = cursor_x >= (screen_width / 2 - width / 2)
                        && cursor_x <= (screen_width / 2 + width / 2)
                        && cursor_y <= height,
                    Position::Bottom => in_zone = cursor_x >= (screen_width / 2 - width / 2)
                        && cursor_x <= (screen_width / 2 + width / 2)
                        && cursor_y >= screen_height - height,
                    Position::Left => in_zone = cursor_x <= width
                        && cursor_y >= (screen_height / 2 - height / 2)
                        && cursor_y <= (screen_height / 2 + height / 2),
                    Position::Right => in_zone = cursor_x >= screen_width - width
                        && cursor_y >= (screen_height / 2 - height / 2)
                        && cursor_y <= (screen_height / 2 + height / 2),
                    _ => {}
                }
            }
            TriggerType::Rect => {
                let x = self.x.unwrap_or(0);
                let y = self.y.unwrap_or(0);
                let w = self.width.unwrap_or(0);
                let h = self.height.unwrap_or(0);
                in_zone = cursor_x >= x && cursor_x <= x + w && cursor_y >= y && cursor_y <= y + h;
            }
        }

        let elapsed_ok = self
            .last_trigger
            .map_or(true, |t| now.duration_since(t).as_millis() >= timeout_ms as u128);

        if in_zone && !self.inside && elapsed_ok {
            self.last_trigger = Some(now);
            self.inside = true;
            true
        } else if !in_zone {
            self.inside = false;
            false
        } else {
            false
        }
    }
}
