use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CursorThemeConfig {
    pub cursors: HashMap<String, CursorDefinition>,
    #[serde(default)]
    pub transitions: HashMap<String, TransitionConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CursorDefinition {
    pub format: CursorFormat,
    pub file: String,
    #[serde(default)]
    pub hotspot: Option<(i32, i32)>,
    #[serde(default)]
    pub loop_mode: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CursorFormat {
    Svg,
    Lottie,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransitionConfig {
    #[serde(default = "default_transition_type")]
    pub transition_type: TransitionType,
    #[serde(default = "default_duration")]
    pub duration_ms: u32,
    #[serde(default = "default_easing")]
    pub easing: EasingFunction,
    #[serde(default)]
    pub file: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransitionType {
    Morph,
    CrossFade,
    Transform,
    Lottie,
}

fn default_transition_type() -> TransitionType {
    TransitionType::Morph
}

fn default_duration() -> u32 {
    200
}

fn default_easing() -> EasingFunction {
    EasingFunction::EaseInOut
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    Elastic,
}

impl CursorThemeConfig {
    pub fn from_toml(toml_str: &str) -> Result<Self> {
        debug!("Parsing cursor theme config from TOML...");
        let config: CursorThemeConfig =
            toml::from_str(toml_str).context("Failed to parse cursor theme config")?;
        debug!(
            "Config parsed successfully with {} cursors defined",
            config.cursors.len()
        );
        debug!("Transitions defined: {:?}", config.transitions.keys());
        Ok(config)
    }

    pub fn get_cursor(&self, cursor_id: &str) -> Option<&CursorDefinition> {
        debug!("Looking up cursor: '{}'", cursor_id);
        let result = self.cursors.get(cursor_id);
        if result.is_some() {
            debug!("Found cursor: '{}'", cursor_id);
        } else {
            debug!("Cursor not found: '{}'", cursor_id);
        }
        result
    }

    pub fn get_transition(&self, from_id: &str, to_id: &str) -> Option<&TransitionConfig> {
        let key = format!("{}->{}", from_id, to_id);
        debug!("Looking up transition: '{}'", key);
        let result = self.transitions.get(&key);
        if result.is_some() {
            debug!("Found transition: '{}'", key);
        } else {
            debug!("Transition not found: '{}'", key);
        }
        result
    }
}
