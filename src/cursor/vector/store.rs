use crate::cursor::vector::config::CursorFormat;
use crate::cursor::vector::config::CursorThemeConfig;
use crate::cursor::vector::renderer::{LottieRenderer, SvgRenderer, VectorRenderer};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

pub struct VectorCursorStore {
    base_path: PathBuf,
    config: Arc<CursorThemeConfig>,
    svg_cache: Arc<parking_lot::RwLock<HashMap<String, Rc<SvgRenderer>>>>,
    lottie_cache: Arc<parking_lot::RwLock<HashMap<String, Rc<LottieRenderer>>>>,
    base_size: u8,
}

impl VectorCursorStore {
    pub fn new(base_path: PathBuf, config: CursorThemeConfig, base_size: u8) -> Result<Self> {
        Ok(Self {
            base_path,
            config: Arc::new(config),
            svg_cache: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            lottie_cache: Arc::new(parking_lot::RwLock::new(HashMap::new())),
            base_size,
        })
    }

    pub fn get_renderer(&self, cursor_id: &str) -> Result<Rc<dyn VectorRenderer>> {
        debug!(
            "VectorCursorStore::get_renderer called for cursor: '{}'",
            cursor_id
        );

        let cursor_def = self
            .config
            .get_cursor(cursor_id)
            .context(format!("Cursor '{}' not found in config", cursor_id))?;

        let renderer: Rc<dyn VectorRenderer> = match cursor_def.format {
            CursorFormat::Svg => {
                let mut cache = self.svg_cache.write();
                if let Some(cached) = cache.get(cursor_id) {
                    return Ok(cached.clone() as Rc<dyn VectorRenderer>);
                }

                let renderer = Rc::new(self.load_svg_renderer(cursor_id, cursor_def)?);
                cache.insert(cursor_id.to_string(), renderer.clone());
                renderer
            }
            CursorFormat::Lottie => {
                let mut cache = self.lottie_cache.write();
                if let Some(cached) = cache.get(cursor_id) {
                    return Ok(cached.clone() as Rc<dyn VectorRenderer>);
                }

                let renderer = Rc::new(self.load_lottie_renderer(cursor_id, cursor_def)?);
                cache.insert(cursor_id.to_string(), renderer.clone());
                renderer
            }
        };

        Ok(renderer)
    }

    fn load_svg_renderer(
        &self,
        cursor_id: &str,
        cursor_def: &crate::cursor::vector::config::CursorDefinition,
    ) -> Result<SvgRenderer> {
        debug!("Loading SVG renderer for cursor: '{}'", cursor_id);
        let file_path = self.base_path.join(&cursor_def.file);
        debug!("SVG file path: {}", file_path.display());

        let svg_data = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read SVG file: {}", file_path.display()))?;

        SvgRenderer::new(
            cursor_id.to_string(),
            svg_data,
            cursor_def.hotspot,
            self.base_size,
        )
    }

    fn load_lottie_renderer(
        &self,
        cursor_id: &str,
        cursor_def: &crate::cursor::vector::config::CursorDefinition,
    ) -> Result<LottieRenderer> {
        debug!("Loading Lottie renderer for cursor: '{}'", cursor_id);
        let file_path = self.base_path.join(&cursor_def.file);
        debug!("Lottie file path: {}", file_path.display());

        let lottie_data = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read Lottie file: {}", file_path.display()))?;

        LottieRenderer::new(
            cursor_id.to_string(),
            lottie_data,
            cursor_def.hotspot,
            self.base_size,
        )
    }

    pub fn get_base_size(&self) -> u8 {
        self.base_size
    }

    pub fn get_config(&self) -> &CursorThemeConfig {
        &self.config
    }
}
