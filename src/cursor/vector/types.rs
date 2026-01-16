use smithay::backend::renderer::element::memory::MemoryRenderBuffer;
use smithay::utils::{Physical, Point};

pub struct RenderedFrame {
    pub buffer: MemoryRenderBuffer,
    pub hotspot: Point<i32, Physical>,
}

#[derive(Clone, Copy, Debug)]
pub enum LoopMode {
    Once,
    Loop,
    Bounce,
}

pub struct VectorCursorData {
    pub cursor_id: String,
    pub format: VectorFormat,
}

pub enum VectorFormat {
    Svg,
    Lottie,
}

#[derive(Debug)]
pub enum TransitionState {
    Static,
    Transitioning {
        from_id: String,
        to_id: String,
        progress: f32,
    },
    Animated {
        cursor_id: String,
        start_time_ms: u32,
        loop_mode: LoopMode,
    },
}
