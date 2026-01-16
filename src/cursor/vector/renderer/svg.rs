use anyhow::{Context, Result};
use fontdb::Database;
use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::element::memory::MemoryRenderBuffer;
use smithay::utils::{Physical, Point, Transform};
use tiny_skia::Pixmap;
use usvg::Tree;

use super::{RenderedFrameData, VectorRenderer};

pub struct SvgRenderer {
    _cursor_id: String,
    tree: Tree,
    hotspot: Option<(i32, i32)>,
    _base_size: u8,
    width: f32,
    height: f32,
}

impl SvgRenderer {
    pub fn new(
        cursor_id: String,
        svg_data: String,
        hotspot: Option<(i32, i32)>,
        base_size: u8,
    ) -> Result<Self> {
        let fontdb = Database::default();
        let tree = Tree::from_str(&svg_data, &usvg::Options::default(), &fontdb)
            .context("Failed to parse SVG")?;

        let size = tree.size();
        let width = size.width() as f32;
        let height = size.height() as f32;

        Ok(Self {
            _cursor_id: cursor_id,
            tree,
            hotspot,
            _base_size: base_size,
            width,
            height,
        })
    }

    fn render_to_buffer(&self, scale: i32) -> Result<RenderedFrameData> {
        let scaled_width = (self.width * scale as f32).ceil() as i32;
        let scaled_height = (self.height * scale as f32).ceil() as i32;

        let size = scaled_width as usize * scaled_height as usize;
        let mut pixels = vec![0u8; size * 4];

        let mut pixmap = Pixmap::new(scaled_width as u32, scaled_height as u32)
            .context("Failed to create pixmap")?;

        let transform = usvg::Transform::from_scale(scale as f32, scale as f32);
        resvg::render(&self.tree, transform, &mut pixmap.as_mut());

        let pixmap_data = pixmap.data();
        for (i, chunk) in pixmap_data.chunks(4).enumerate() {
            if i * 4 + 4 <= pixels.len() {
                pixels[i * 4] = chunk[2];
                pixels[i * 4 + 1] = chunk[1];
                pixels[i * 4 + 2] = chunk[0];
                pixels[i * 4 + 3] = chunk[3];
            }
        }

        let buffer = MemoryRenderBuffer::from_slice(
            &pixels,
            Fourcc::Argb8888,
            (scaled_width, scaled_height),
            scale,
            Transform::Normal,
            None,
        );

        let hotspot = if let Some((hx, hy)) = self.hotspot {
            Point::new(hx * scale, hy * scale)
        } else {
            Point::new(0, 0)
        };

        Ok(RenderedFrameData {
            buffer,
            hotspot: hotspot.to_physical(scale),
        })
    }
}

impl VectorRenderer for SvgRenderer {
    fn render_frame(&self, frame: u32, scale: i32) -> Result<RenderedFrameData> {
        let _ = frame;
        self.render_to_buffer(scale)
    }

    fn hotspot(&self) -> Point<i32, Physical> {
        let (hx, hy) = self.hotspot.unwrap_or((0, 0));
        Point::from((hx, hy))
    }

    fn total_frames(&self) -> u32 {
        1
    }

    fn frame_duration_ms(&self) -> u32 {
        0
    }
}
