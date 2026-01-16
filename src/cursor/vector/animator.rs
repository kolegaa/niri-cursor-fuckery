use crate::cursor::vector::config::{CursorThemeConfig, EasingFunction};
use crate::cursor::vector::types::{LoopMode, TransitionState};
use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

pub struct CursorAnimator {
    config: Rc<CursorThemeConfig>,
    current_state: RefCell<TransitionState>,
    _last_update: RefCell<Instant>,
    base_size: u8,
}

impl CursorAnimator {
    pub fn new(config: CursorThemeConfig, base_size: u8) -> Self {
        let mut state = TransitionState::Static;

        debug!("CursorAnimator created, initializing with default cursor");

        // Initialize with default cursor if available
        if let Some(default_def) = config.cursors.get("default") {
            let loop_mode = match default_def.loop_mode.as_deref() {
                Some("once") => LoopMode::Once,
                Some("loop") => LoopMode::Loop,
                Some("bounce") => LoopMode::Bounce,
                _ => LoopMode::Loop,
            };

            state = TransitionState::Animated {
                cursor_id: "default".to_string(),
                start_time_ms: 0,
                loop_mode,
            };

            debug!("Initialized CursorAnimator with default cursor");
        } else {
            debug!("No default cursor defined, keeping Static state");
        }

        Self {
            config: Rc::new(config),
            current_state: RefCell::new(state),
            _last_update: RefCell::new(Instant::now()),
            base_size,
        }
    }

    pub fn set_cursor(&self, cursor_id: &str) -> Result<()> {
        debug!(
            "CursorAnimator::set_cursor called with cursor_id: '{}'",
            cursor_id
        );

        let mut state = self.current_state.borrow_mut();
        let from_id = match &*state {
            TransitionState::Static => {
                debug!("Current state is Static");
                None
            }
            TransitionState::Animated { cursor_id, .. } => {
                debug!("Current state is Animated with cursor: '{}'", cursor_id);
                Some(cursor_id.clone())
            }
            TransitionState::Transitioning { to_id, .. } => {
                debug!("Current state is Transitioning to cursor: '{}'", to_id);
                Some(to_id.clone())
            }
        };

        if let Some(from) = from_id {
            if from == cursor_id {
                debug!("Already showing cursor '{}', no change needed", cursor_id);
                return Ok(());
            }

            debug!("Checking for transition from '{}' to '{}'", from, cursor_id);
            if self.config.get_transition(&from, cursor_id).is_some() {
                debug!("Found transition, setting state to Transitioning");
                *state = TransitionState::Transitioning {
                    from_id: from.clone(),
                    to_id: cursor_id.to_string(),
                    progress: 0.0,
                };
                return Ok(());
            }
        }

        debug!("Looking up cursor definition for '{}'", cursor_id);
        if let Some(cursor_def) = self.config.get_cursor(cursor_id) {
            let loop_mode = match cursor_def.loop_mode.as_deref() {
                Some("once") => LoopMode::Once,
                Some("loop") => LoopMode::Loop,
                Some("bounce") => LoopMode::Bounce,
                _ => LoopMode::Loop,
            };

            debug!(
                "Found cursor definition, setting state to Animated with loop_mode: {:?}",
                loop_mode
            );
            *state = TransitionState::Animated {
                cursor_id: cursor_id.to_string(),
                start_time_ms: 0,
                loop_mode,
            };
        } else {
            debug!("No cursor definition found, setting state to Static");
            *state = TransitionState::Static;
        }

        Ok(())
    }

    pub fn update(&self, elapsed_ms: u32) {
        let mut state = self.current_state.borrow_mut();
        let mut new_state = None;

        match &*state {
            TransitionState::Transitioning {
                from_id,
                to_id,
                progress,
            } => {
                let config = match self.config.get_transition(from_id, to_id) {
                    Some(c) => c,
                    None => {
                        *state = TransitionState::Static;
                        return;
                    }
                };

                let duration_ms = config.duration_ms;
                let delta_ms = elapsed_ms;

                let new_progress = *progress + (delta_ms as f32 / duration_ms as f32);

                if new_progress >= 1.0 {
                    new_state = Some(TransitionState::Animated {
                        cursor_id: to_id.clone(),
                        start_time_ms: 0,
                        loop_mode: LoopMode::Loop,
                    });
                } else {
                    let eased_progress = Self::apply_easing(new_progress, &config.easing);
                    *state = TransitionState::Transitioning {
                        from_id: from_id.clone(),
                        to_id: to_id.clone(),
                        progress: eased_progress,
                    };
                }
            }
            TransitionState::Animated {
                cursor_id,
                start_time_ms,
                loop_mode,
            } => {
                if let Some(cursor_def) = self.config.get_cursor(cursor_id) {
                    if cursor_def.format == crate::cursor::vector::config::CursorFormat::Lottie {
                        let new_start = *start_time_ms + elapsed_ms;
                        *state = TransitionState::Animated {
                            cursor_id: cursor_id.clone(),
                            start_time_ms: new_start,
                            loop_mode: loop_mode.clone(),
                        };
                    }
                }
            }
            TransitionState::Static => {}
        }

        if let Some(s) = new_state {
            *state = s;
        }
    }

    fn apply_easing(t: f32, easing: &EasingFunction) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match easing {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t).powi(2)
                }
            }
            EasingFunction::EaseInQuad => t * t,
            EasingFunction::EaseOutQuad => 1.0 - (1.0 - t).powi(2),
            EasingFunction::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t).powi(2)
                }
            }
            EasingFunction::Elastic => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    (2.0f32).powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
                }
            }
        }
    }

    pub fn get_base_size(&self) -> u8 {
        self.base_size
    }

    pub fn current_state(&self) -> std::cell::Ref<'_, TransitionState> {
        self.current_state.borrow()
    }
}
