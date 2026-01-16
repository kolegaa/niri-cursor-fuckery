pub mod lottie;
pub mod svg;

pub use lottie::LottieRenderer;
pub use svg::SvgRenderer;

use anyhow::Result;
use smithay::backend::renderer::element::memory::MemoryRenderBuffer;
use smithay::utils::Point;

pub trait VectorRenderer: Send + Sync {
    fn render_frame(&self, frame: u32, scale: i32) -> Result<RenderedFrameData>;
    fn hotspot(&self) -> Point<i32, smithay::utils::Physical>;
    fn total_frames(&self) -> u32;
    fn frame_duration_ms(&self) -> u32;
}

pub struct RenderedFrameData {
    pub buffer: MemoryRenderBuffer,
    pub hotspot: Point<i32, smithay::utils::Physical>,
}
