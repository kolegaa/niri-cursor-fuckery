use anyhow::{Context, Result};
use serde_json::Value;
use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::element::memory::MemoryRenderBuffer;
use smithay::utils::{Physical, Point, Transform};
use std::sync::Arc;

use super::RenderedFrameData;
use super::VectorRenderer;

pub struct LottieRenderer {
    _cursor_id: String,
    _lottie_data: String,
    hotspot: Option<(i32, i32)>,
    _base_size: u8,
    width: f32,
    height: f32,
    frame_rate: f32,
    total_frames: u32,
    composition: Arc<Value>,
}

impl LottieRenderer {
    pub fn new(
        cursor_id: String,
        lottie_data: String,
        hotspot: Option<(i32, i32)>,
        base_size: u8,
    ) -> Result<Self> {
        let json: Value =
            serde_json::from_str(&lottie_data).context("Failed to parse Lottie JSON")?;

        let width = json.get("w").and_then(|v| v.as_f64()).unwrap_or(24.0) as f32;

        let height = json.get("h").and_then(|v| v.as_f64()).unwrap_or(24.0) as f32;

        let frame_rate = json.get("fr").and_then(|v| v.as_f64()).unwrap_or(60.0) as f32;

        let total_frames = json.get("op").and_then(|v| v.as_f64()).unwrap_or(0.0) as u32;

        Ok(Self {
            _cursor_id: cursor_id,
            _lottie_data: lottie_data,
            hotspot,
            _base_size: base_size,
            width,
            height,
            frame_rate,
            total_frames,
            composition: Arc::new(json),
        })
    }

    fn parse_layer(&self, layer: &Value, frame: f32) -> Result<Vec<RenderPrimitive>> {
        let mut primitives = Vec::new();

        if let Some(shapes) = layer.get("shapes") {
            if let Some(shapes_array) = shapes.as_array() {
                for shape in shapes_array {
                    if let Some(shape_type) = shape.get("ty") {
                        if let Some(ty) = shape_type.as_str() {
                            match ty {
                                "gr" => {
                                    if let Some(items) = shape.get("it") {
                                        if let Some(items_array) = items.as_array() {
                                            for item in items_array {
                                                if let Some(item_type) = item.get("ty") {
                                                    if let Some(item_ty) = item_type.as_str() {
                                                        if item_ty == "sh" {
                                                            if let Ok(path_prims) =
                                                                self.parse_shape_path(item, frame)
                                                            {
                                                                primitives.extend(path_prims);
                                                            }
                                                        } else if item_ty == "fl" {
                                                            if let Some(color) = item.get("c") {
                                                                if let Some(color_array) =
                                                                    color.as_array()
                                                                {
                                                                    if color_array.len() >= 4 {
                                                                        let fill_color = [
                                                                            (color_array[0]
                                                                                .as_f64()
                                                                                .unwrap_or(0.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                            (color_array[1]
                                                                                .as_f64()
                                                                                .unwrap_or(0.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                            (color_array[2]
                                                                                .as_f64()
                                                                                .unwrap_or(0.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                            (color_array[3]
                                                                                .as_f64()
                                                                                .unwrap_or(1.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                        ];
                                                                        for prim in &mut primitives
                                                                        {
                                                                            if matches!(prim, RenderPrimitive::Path { .. }) {
                                                                                if let RenderPrimitive::Path { fill: None, .. } = prim {
                                                                                    *prim = RenderPrimitive::Path {
                                                                                        vertices: prim.get_vertices().to_vec(),
                                                                                        indices: prim.get_indices().to_vec(),
                                                                                        fill: Some(fill_color),
                                                                                        stroke: None,
                                                                                    };
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else if item_ty == "st" {
                                                            if let Some(color) = item.get("c") {
                                                                if let Some(color_array) =
                                                                    color.as_array()
                                                                {
                                                                    let stroke_width = item
                                                                        .get("w")
                                                                        .and_then(|v| v.as_f64())
                                                                        .unwrap_or(1.0)
                                                                        as f32;
                                                                    if color_array.len() >= 4 {
                                                                        let stroke_color = [
                                                                            (color_array[0]
                                                                                .as_f64()
                                                                                .unwrap_or(0.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                            (color_array[1]
                                                                                .as_f64()
                                                                                .unwrap_or(0.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                            (color_array[2]
                                                                                .as_f64()
                                                                                .unwrap_or(0.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                            (color_array[3]
                                                                                .as_f64()
                                                                                .unwrap_or(1.0)
                                                                                * 255.0)
                                                                                as u8,
                                                                        ];
                                                                        for prim in &mut primitives
                                                                        {
                                                                            if matches!(prim, RenderPrimitive::Path { .. }) {
                                                                                if let RenderPrimitive::Path { stroke: None, .. } = prim {
                                                                                    *prim = RenderPrimitive::Path {
                                                                                        vertices: prim.get_vertices().to_vec(),
                                                                                        indices: prim.get_indices().to_vec(),
                                                                                        fill: None,
                                                                                        stroke: Some((stroke_width, stroke_color)),
                                                                                    };
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        Ok(primitives)
    }

    fn parse_shape_path(&self, shape: &Value, _frame: f32) -> Result<Vec<RenderPrimitive>> {
        let mut primitives = Vec::new();

        if let Some(path_data) = shape.get("ks") {
            if let Some(ks) = path_data.as_object() {
                if let Some(a) = ks.get("a") {
                    if let Some(_anchors) = a.as_array() {
                        if let Some(k) = ks.get("k") {
                            if let Some(values) = k.as_array() {
                                if values.len() >= 1 {
                                    if let Some(k_value) = values[0].as_array() {
                                        if k_value.len() >= 6 {
                                            let (sx, sy, ex, ey, cx1, cy1, cx2, cy2) = (
                                                k_value
                                                    .get(0)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                                k_value
                                                    .get(1)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                                k_value
                                                    .get(2)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                                k_value
                                                    .get(3)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                                k_value
                                                    .get(4)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                                k_value
                                                    .get(5)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                                k_value
                                                    .get(6)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                                k_value
                                                    .get(7)
                                                    .and_then(|v| v.as_f64())
                                                    .unwrap_or(0.0),
                                            );

                                            let vertices = vec![
                                                [sx as f32, sy as f32],
                                                [ex as f32, ey as f32],
                                                [cx1 as f32, cy1 as f32],
                                                [cx2 as f32, cy2 as f32],
                                            ];

                                            let indices = vec![0u16, 1, 2, 2, 1, 3];

                                            primitives.push(RenderPrimitive::Path {
                                                vertices,
                                                indices,
                                                fill: None,
                                                stroke: None,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(primitives)
    }

    fn render_frame_to_buffer(&self, frame: u32, scale: i32) -> Result<RenderedFrameData> {
        let frame_float = frame as f32;
        let scaled_width = (self.width * scale as f32).ceil() as i32;
        let scaled_height = (self.height * scale as f32).ceil() as i32;

        let size = scaled_width as usize * scaled_height as usize;
        let mut pixels = vec![0u8; size * 4];

        if let Some(layers) = self.composition.get("layers") {
            if let Some(layers_array) = layers.as_array() {
                for layer in layers_array {
                    if let Ok(primitives) = self.parse_layer(layer, frame_float) {
                        for prim in primitives {
                            self.render_primitive(
                                &prim,
                                &mut pixels,
                                scaled_width,
                                scaled_height,
                                scale,
                            );
                        }
                    }
                }
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

    fn render_primitive(
        &self,
        prim: &RenderPrimitive,
        pixels: &mut [u8],
        width: i32,
        height: i32,
        scale: i32,
    ) {
        match prim {
            RenderPrimitive::Path {
                vertices,
                indices,
                fill,
                stroke,
            } => {
                if let Some(color) = fill {
                    for chunk in indices.chunks(3) {
                        if chunk.len() == 3 {
                            let v0 = vertices.get(chunk[0] as usize);
                            let v1 = vertices.get(chunk[1] as usize);
                            let v2 = vertices.get(chunk[2] as usize);

                            if let (Some(v0), Some(v1), Some(v2)) = (v0, v1, v2) {
                                self.rasterize_triangle(
                                    [*v0, *v1, *v2],
                                    *color,
                                    pixels,
                                    width,
                                    height,
                                    scale,
                                );
                            }
                        }
                    }
                }

                if let Some((stroke_width, color)) = stroke {
                    for vertex in vertices.iter() {
                        let x = (vertex[0] * scale as f32) as i32;
                        let y = (vertex[1] * scale as f32) as i32;

                        let radius = (stroke_width * scale as f32 / 2.0) as i32;
                        for dy in -radius..=radius {
                            for dx in -radius..=radius {
                                if dx * dx + dy * dy <= radius * radius {
                                    let px = x + dx;
                                    let py = y + dy;
                                    if px >= 0 && px < width && py >= 0 && py < height {
                                        let offset = ((py * width + px) * 4) as usize;
                                        if offset + 4 <= pixels.len() {
                                            pixels[offset] = color[0];
                                            pixels[offset + 1] = color[1];
                                            pixels[offset + 2] = color[2];
                                            pixels[offset + 3] = color[3];
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn rasterize_triangle(
        &self,
        vertices: [[f32; 2]; 3],
        color: [u8; 4],
        pixels: &mut [u8],
        width: i32,
        height: i32,
        scale: i32,
    ) {
        let v0 = [
            (vertices[0][0] * scale as f32) as i32,
            (vertices[0][1] * scale as f32) as i32,
        ];
        let v1 = [
            (vertices[1][0] * scale as f32) as i32,
            (vertices[1][1] * scale as f32) as i32,
        ];
        let v2 = [
            (vertices[2][0] * scale as f32) as i32,
            (vertices[2][1] * scale as f32) as i32,
        ];

        let min_x = v0[0].min(v1[0]).min(v2[0]).max(0);
        let max_x = v0[0].max(v1[0]).max(v2[0]).min(width - 1);
        let min_y = v0[1].min(v1[1]).min(v2[1]).max(0);
        let max_y = v0[1].max(v1[1]).max(v2[1]).min(height - 1);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.point_in_triangle(x, y, v0, v1, v2) {
                    let offset = ((y * width + x) * 4) as usize;
                    if offset + 4 <= pixels.len() {
                        pixels[offset] = color[0];
                        pixels[offset + 1] = color[1];
                        pixels[offset + 2] = color[2];
                        pixels[offset + 3] = color[3];
                    }
                }
            }
        }
    }

    fn point_in_triangle(
        &self,
        px: i32,
        py: i32,
        v0: [i32; 2],
        v1: [i32; 2],
        v2: [i32; 2],
    ) -> bool {
        let det = (v1[1] - v2[1]) * (v0[0] - v2[0]) + (v2[0] - v1[0]) * (v0[1] - v2[1]);
        let lambda1 =
            ((v1[1] - v2[1]) * (px - v2[0]) + (v2[0] - v1[0]) * (py - v2[1])) as f32 / det as f32;
        let lambda2 =
            ((v2[1] - v0[1]) * (px - v2[0]) + (v0[0] - v2[0]) * (py - v2[1])) as f32 / det as f32;
        let lambda3 = 1.0 - lambda1 - lambda2;

        lambda1 >= 0.0 && lambda2 >= 0.0 && lambda3 >= 0.0
    }
}

#[derive(Clone)]
enum RenderPrimitive {
    Path {
        vertices: Vec<[f32; 2]>,
        indices: Vec<u16>,
        fill: Option<[u8; 4]>,
        stroke: Option<(f32, [u8; 4])>,
    },
}

impl RenderPrimitive {
    fn get_vertices(&self) -> &[[f32; 2]] {
        match self {
            RenderPrimitive::Path { vertices, .. } => vertices,
        }
    }

    fn get_indices(&self) -> &[u16] {
        match self {
            RenderPrimitive::Path { indices, .. } => indices,
        }
    }
}

impl VectorRenderer for LottieRenderer {
    fn render_frame(&self, frame: u32, scale: i32) -> Result<RenderedFrameData> {
        let actual_frame = if self.total_frames > 0 {
            frame % self.total_frames
        } else {
            0
        };
        self.render_frame_to_buffer(actual_frame, scale)
    }

    fn hotspot(&self) -> Point<i32, Physical> {
        let (hx, hy) = self.hotspot.unwrap_or((0, 0));
        Point::from((hx, hy))
    }

    fn total_frames(&self) -> u32 {
        self.total_frames
    }

    fn frame_duration_ms(&self) -> u32 {
        if self.frame_rate > 0.0 {
            (1000.0 / self.frame_rate) as u32
        } else {
            16
        }
    }
}
