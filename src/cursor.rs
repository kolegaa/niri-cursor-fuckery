use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::{anyhow, Context};
use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::element::memory::MemoryRenderBuffer;
use smithay::input::pointer::{CursorIcon, CursorImageStatus, CursorImageSurfaceData};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::{IsAlive, Logical, Physical, Point, Transform};
use smithay::wayland::compositor::with_states;
use xcursor::parser::{parse_xcursor, Image};
use xcursor::CursorTheme;

use crate::cur_buf::{get_cursor_hotspot, get_cursor_surface};
use crate::cursor::vector::{CursorAnimator, VectorCursorStore};

pub mod vector;

/// Some default looking `left_ptr` icon.
static FALLBACK_CURSOR_DATA: &[u8] = include_bytes!("../resources/cursor.rgba");

type XCursorCache = HashMap<(CursorIcon, i32), Option<Rc<XCursor>>>;

pub struct CursorManager {
    theme: CursorTheme,
    size: u8,
    current_cursor: CursorImageStatus,
    named_cursor_cache: RefCell<XCursorCache>,
    vector_system: Option<VectorCursorSystem>,
    icon_to_vector_id: HashMap<CursorIcon, String>,
}

struct VectorCursorSystem {
    store: VectorCursorStore,
    animator: CursorAnimator,
}

impl CursorManager {
    pub fn new(theme: &str, size: u8) -> Self {
        Self::new_with_vector_theme(theme, size, None)
    }

    pub fn new_with_vector_theme(
        theme: &str,
        size: u8,
        vector_theme_path: Option<PathBuf>,
    ) -> Self {
        Self::ensure_env(theme, size);

        let theme = CursorTheme::load(theme);

        let vector_system = if let Some(path) = vector_theme_path {
            debug!("Loading vector cursor system from path: {}", path.display());
            let result = Self::load_vector_system(&path, size);
            match &result {
                Ok(_) => info!("Vector cursor system loaded successfully"),
                Err(e) => warn!(
                    "Failed to load vector cursor system: {:?}, will use XCursor fallback",
                    e
                ),
            }
            result.ok()
        } else {
            debug!("No vector theme path provided, using XCursor only");
            None
        };

        let icon_to_vector_id = if vector_system.is_some() {
            info!("Vector system available, mapping CursorIcon to vector cursor IDs");
            let vs = vector_system.as_ref().unwrap();
            let config = vs.store.get_config();

            debug!("Available cursors in config: {:?}", config.cursors.keys());

            let mut mapping = HashMap::new();

            // Map CursorIcon enum variants to vector cursor IDs
            // Use CursorIcon::name() to get the xcursor name
            for (cursor_id, _) in &config.cursors {
                debug!("Processing cursor ID: '{}'", cursor_id);

                // Try to find matching CursorIcon by name
                // Common cursor names in XCursor themes
                let icon_name = cursor_id.to_lowercase();

                let icon = match icon_name.as_str() {
                    "default" | "left_ptr" => CursorIcon::Default,
                    "move" | "fleur" | "move" => CursorIcon::AllScroll,
                    "text" | "xterm" | "ibeam" => CursorIcon::Text,
                    "wait" | "watch" => CursorIcon::Wait,
                    "progress" | "left_ptr_watch" => CursorIcon::Progress,
                    "crosshair" | "cross_reverse" => CursorIcon::Crosshair,
                    "nwse-resize" | "top_left_corner" => CursorIcon::NwResize,
                    "pointer" | "hand" | "hand1" | "hand2" => CursorIcon::Pointer,
                    "grab" | "openhand" => CursorIcon::Grab,
                    "grabbing" | "grabbing" | "closedhand" => CursorIcon::Grabbing,
                    "not-allowed" | "circle" | "dnd-none" => CursorIcon::NotAllowed,
                    "help" | "question_arrow" => CursorIcon::Help,
                    "copy" => CursorIcon::Copy,
                    "alias" => CursorIcon::Alias,
                    "cell" => CursorIcon::Cell,
                    "vertical-text" => CursorIcon::VerticalText,
                    "context-menu" => CursorIcon::ContextMenu,
                    "no-drop" => CursorIcon::NoDrop,
                    "col-resize" | "sb_h_double_arrow" => CursorIcon::WResize,
                    "row-resize" | "sb_v_double_arrow" => CursorIcon::NResize,
                    "ew-resize" => CursorIcon::WResize,
                    "ns-resize" => CursorIcon::NResize,
                    "nesw-resize" | "top_right_corner" => CursorIcon::NeResize,
                    "swne-resize" | "bottom_left_corner" => CursorIcon::SwResize,
                    "sene-resize" | "bottom_right_corner" => CursorIcon::SeResize,
                    "zoom-in" => CursorIcon::ZoomIn,
                    "zoom-out" => CursorIcon::ZoomOut,
                    _ => {
                        debug!("No CursorIcon match for cursor ID: '{}'", cursor_id);
                        continue;
                    }
                };

                mapping.insert(icon, cursor_id.clone());
                info!(
                    "Mapped cursor icon {:?} (name: '{}') to vector cursor '{}'",
                    icon, cursor_id, cursor_id
                );
            }

            info!("Mapped {} cursor icons to vector cursors", mapping.len());
            mapping
        } else {
            info!("No vector system available, no cursor icon mapping");
            HashMap::new()
        };

        Self {
            theme,
            size,
            current_cursor: CursorImageStatus::default_named(),
            named_cursor_cache: Default::default(),
            vector_system,
            icon_to_vector_id,
        }
    }

    /// Reload the cursor theme.
    pub fn reload(&mut self, theme: &str, size: u8) {
        Self::ensure_env(theme, size);
        self.theme = CursorTheme::load(theme);
        self.size = size;
        self.named_cursor_cache.get_mut().clear();
    }

    fn load_vector_system(path: &PathBuf, size: u8) -> anyhow::Result<VectorCursorSystem> {
        use crate::cursor::vector::CursorThemeConfig;
        use std::fs;

        debug!(
            "load_vector_system called with path: {}, size: {}",
            path.display(),
            size
        );
        let config_path = path.join("theme.toml");
        debug!("Config path: {}", config_path.display());

        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        debug!("Config file read successfully, parsing TOML...");

        let config = CursorThemeConfig::from_toml(&config_str)
            .with_context(|| "Failed to parse TOML config")?;
        debug!(
            "TOML parsed successfully, {} cursors defined",
            config.cursors.len()
        );

        let store = VectorCursorStore::new(path.clone(), config, size)?;
        let animator = CursorAnimator::new(store.get_config().clone(), size);

        Ok(VectorCursorSystem { store, animator })
    }

    /// Checks if the cursor WlSurface is alive, and if not, cleans it up.
    pub fn check_cursor_image_surface_alive(&mut self) {
        if let CursorImageStatus::Surface(surface) = &self.current_cursor {
            if !surface.alive() {
                self.current_cursor = CursorImageStatus::default_named();
            }
        }
    }

    /// Get the current rendering cursor.
    pub fn get_render_cursor(&self, scale: i32) -> RenderCursor {
        // Try vector system first
        if let Some(vector) = &self.vector_system {
            if let Ok(render_cursor) = self.get_vector_cursor(vector, scale) {
                return render_cursor;
            }
        }

        // Try to get the custom cursor surface from curBuf
        if let Some(surface) = get_cursor_surface() {
            let hotspot = get_cursor_hotspot();
            return RenderCursor::Surface { hotspot, surface };
        }

        // Fallback to original logic if no custom surface is available
        match self.current_cursor.clone() {
            CursorImageStatus::Hidden => RenderCursor::Hidden,
            CursorImageStatus::Surface(surface) => {
                let hotspot = with_states(&surface, |states| {
                    states
                        .data_map
                        .get::<CursorImageSurfaceData>()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .hotspot
                });

                RenderCursor::Surface { hotspot, surface }
            }
            CursorImageStatus::Named(icon) => self.get_render_cursor_named(icon, scale),
        }
    }

    fn get_vector_cursor(
        &self,
        vector: &VectorCursorSystem,
        scale: i32,
    ) -> Result<RenderCursor, anyhow::Error> {
        use crate::cursor::vector::types::TransitionState;

        debug!("get_vector_cursor called with scale: {}", scale);
        let state = vector.animator.current_state();
        debug!("Current animator state: {:?}", state);

        let cursor_id = match &*state {
            TransitionState::Static => {
                debug!("State is Static, returning error");
                return Err(anyhow::anyhow!("No active cursor"));
            }
            TransitionState::Animated { cursor_id, .. } => {
                debug!("State is Animated with cursor: '{}'", cursor_id);
                cursor_id.clone()
            }
            TransitionState::Transitioning { to_id, .. } => {
                debug!("State is Transitioning to cursor: '{}'", to_id);
                to_id.clone()
            }
        };

        debug!("Getting renderer for cursor: '{}'", cursor_id);
        let renderer = vector.store.get_renderer(&cursor_id)?;
        debug!("Renderer obtained, rendering frame 0");
        let frame_data = renderer.render_frame(0, scale)?;
        debug!("Frame rendered successfully");

        Ok(RenderCursor::Vector {
            hotspot: frame_data.hotspot,
            buffer: frame_data.buffer,
        })
    }

    fn get_render_cursor_named(&self, icon: CursorIcon, scale: i32) -> RenderCursor {
        self.get_cursor_with_name(icon, scale)
            .map(|cursor| RenderCursor::Named {
                icon,
                scale,
                cursor,
            })
            .unwrap_or_else(|| RenderCursor::Named {
                icon: Default::default(),
                scale,
                cursor: self.get_default_cursor(scale),
            })
    }

    pub fn is_current_cursor_animated(&self, scale: i32) -> bool {
        match &self.current_cursor {
            CursorImageStatus::Hidden => false,
            CursorImageStatus::Surface(_) => false,
            CursorImageStatus::Named(icon) => self
                .get_cursor_with_name(*icon, scale)
                .unwrap_or_else(|| self.get_default_cursor(scale))
                .is_animated_cursor(),
        }
    }

    /// Get named cursor for the given `icon` and `scale`.
    pub fn get_cursor_with_name(&self, icon: CursorIcon, scale: i32) -> Option<Rc<XCursor>> {
        self.named_cursor_cache
            .borrow_mut()
            .entry((icon, scale))
            .or_insert_with_key(|(icon, scale)| {
                let size = self.size as i32 * scale;
                let mut cursor = Self::load_xcursor(&self.theme, icon.name(), size);

                // Check alternative names to account for non-compliant themes.
                if cursor.is_err() {
                    for name in icon.alt_names() {
                        cursor = Self::load_xcursor(&self.theme, name, size);
                        if cursor.is_ok() {
                            break;
                        }
                    }
                }

                if let Err(err) = &cursor {
                    warn!("error loading xcursor {}@{size}: {err:?}", icon.name());
                }

                // The default cursor must always have a fallback.
                if *icon == CursorIcon::Default && cursor.is_err() {
                    cursor = Ok(Self::fallback_cursor());
                }

                cursor.ok().map(Rc::new)
            })
            .clone()
    }

    /// Get default cursor.
    pub fn get_default_cursor(&self, scale: i32) -> Rc<XCursor> {
        // The default cursor always has a fallback.
        self.get_cursor_with_name(CursorIcon::Default, scale)
            .unwrap()
    }

    /// Currently used cursor_image as a cursor provider.
    pub fn cursor_image(&self) -> &CursorImageStatus {
        &self.current_cursor
    }

    /// Set new cursor image provider.
    pub fn set_cursor_image(&mut self, cursor: CursorImageStatus) {
        debug!("set_cursor_image called with cursor: {:?}", cursor);

        // Update vector animator if we have a vector system
        if let Some(vector) = &mut self.vector_system {
            if let CursorImageStatus::Named(icon) = &cursor {
                if let Some(vector_id) = self.icon_to_vector_id.get(icon) {
                    debug!("Updating vector animator to cursor: {}", vector_id);
                    match vector.animator.set_cursor(vector_id) {
                        Ok(()) => debug!("Vector animator updated successfully"),
                        Err(err) => warn!("Failed to update vector animator: {:?}", err),
                    }
                } else {
                    debug!("No vector cursor mapping for icon: {:?}", icon);
                }
            }
        }

        self.current_cursor = cursor;
    }

    /// Load the cursor with the given `name` from the file system picking the closest
    /// one to the given `size`.
    fn load_xcursor(theme: &CursorTheme, name: &str, size: i32) -> anyhow::Result<XCursor> {
        let _span = tracy_client::span!("load_xcursor");

        let path = theme
            .load_icon(name)
            .ok_or_else(|| anyhow!("no default icon"))?;

        let mut file = File::open(path).context("error opening cursor icon file")?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)
            .context("error reading cursor icon file")?;

        let mut images = parse_xcursor(&buf).context("error parsing cursor icon file")?;

        let (width, height) = images
            .iter()
            .min_by_key(|image| (size - image.size as i32).abs())
            .map(|image| (image.width, image.height))
            .unwrap();

        images.retain(move |image| image.width == width && image.height == height);

        let animation_duration = images.iter().fold(0, |acc, image| acc + image.delay);

        Ok(XCursor {
            images,
            animation_duration,
        })
    }

    /// Set the common XCURSOR env variables.
    fn ensure_env(theme: &str, size: u8) {
        env::set_var("XCURSOR_THEME", theme);
        env::set_var("XCURSOR_SIZE", size.to_string());
    }

    fn fallback_cursor() -> XCursor {
        let images = vec![Image {
            size: 32,
            width: 64,
            height: 64,
            xhot: 1,
            yhot: 1,
            delay: 0,
            pixels_rgba: Vec::from(FALLBACK_CURSOR_DATA),
            pixels_argb: vec![],
        }];

        XCursor {
            images,
            animation_duration: 0,
        }
    }
}

/// The cursor prepared for renderer.
pub enum RenderCursor {
    Hidden,
    Surface {
        hotspot: Point<i32, Logical>,
        surface: WlSurface,
    },
    Named {
        icon: CursorIcon,
        scale: i32,
        cursor: Rc<XCursor>,
    },
    Vector {
        hotspot: Point<i32, Physical>,
        buffer: MemoryRenderBuffer,
    },
}

type TextureCache = HashMap<(CursorIcon, i32), Vec<MemoryRenderBuffer>>;

#[derive(Default)]
pub struct CursorTextureCache {
    cache: RefCell<TextureCache>,
}

impl CursorTextureCache {
    pub fn clear(&mut self) {
        self.cache.get_mut().clear();
    }

    pub fn get(
        &self,
        icon: CursorIcon,
        scale: i32,
        cursor: &XCursor,
        idx: usize,
    ) -> MemoryRenderBuffer {
        self.cache
            .borrow_mut()
            .entry((icon, scale))
            .or_insert_with(|| {
                cursor
                    .frames()
                    .iter()
                    .map(|frame| {
                        MemoryRenderBuffer::from_slice(
                            &frame.pixels_rgba,
                            Fourcc::Argb8888,
                            (frame.width as i32, frame.height as i32),
                            scale,
                            Transform::Normal,
                            None,
                        )
                    })
                    .collect()
            })[idx]
            .clone()
    }
}

// The XCursorBuffer implementation is inspired by `wayland-rs`, thus provided under MIT license.

/// The state of the `NamedCursor`.
pub struct XCursor {
    /// The image for the underlying named cursor.
    images: Vec<Image>,
    /// The total duration of the animation.
    animation_duration: u32,
}

impl XCursor {
    /// Given a time, calculate which frame to show, and how much time remains until the next frame.
    ///
    /// Time will wrap, so if for instance the cursor has an animation lasting 100ms,
    /// then calling this function with 5ms and 105ms as input gives the same output.
    pub fn frame(&self, mut millis: u32) -> (usize, &Image) {
        if self.animation_duration == 0 {
            return (0, &self.images[0]);
        }

        millis %= self.animation_duration;

        let mut res = 0;
        for (i, img) in self.images.iter().enumerate() {
            if millis < img.delay {
                res = i;
                break;
            }
            millis -= img.delay;
        }

        (res, &self.images[res])
    }

    /// Get the frames for the given `XCursor`.
    pub fn frames(&self) -> &[Image] {
        &self.images
    }

    /// Check whether the cursor is animated.
    pub fn is_animated_cursor(&self) -> bool {
        self.images.len() > 1
    }

    /// Get hotspot for the given `image`.
    pub fn hotspot(image: &Image) -> Point<i32, Physical> {
        (image.xhot as i32, image.yhot as i32).into()
    }
}
