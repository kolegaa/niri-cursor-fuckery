# Vector Cursor System

A flexible cursor rendering system for Niri that supports SVG and Lottie formats with animated transitions.

## Overview

The vector cursor system provides:

- **SVG Support**: Render scalable vector cursors from SVG files
- **Lottie Support**: Render complex animations from JSON-based Lottie files
- **Animated Transitions**: Smooth morphing, cross-fade, and transform-based transitions between cursor states
- **Configurable**: TOML-based configuration for easy customization
- **Size-Aware**: Preserves the base `size` parameter for consistent cursor sizing across different output scales

## Architecture

```
src/cursor/vector/
├── mod.rs           # Main module exports
├── store.rs         # SVG & Lottie loading and caching
├── animator.rs      # Transition state management
├── config.rs        # TOML configuration parsing
├── types.rs         # Shared type definitions
└── renderer/
    ├── mod.rs       # Renderer trait
    ├── svg.rs       # SVG rendering via usvg/tiny-skia
    └── lottie.rs   # Lottie rendering via custom rasterizer
```

## Usage

### Basic Setup

```rust
use niri::cursor::{CursorManager, VectorCursorSystem};
use std::path::PathBuf;

// Create cursor manager with vector theme
let vector_theme_path = PathBuf::from("/path/to/cursor/theme");
let cursor_manager = CursorManager::new_with_vector_theme(
    "default",  // xcursor theme name (fallback)
    24,          // base size
    Some(vector_theme_path),
);
```

### Configuration File (`theme.toml`)

```toml
[cursors.default]
format = "svg"
file = "vectors/default.svg"

[cursors.wait]
format = "lottie"
file = "lottie/loading.json"
loop_mode = "loop"

[transitions."default->wait"]
transition_type = "lottie"
file = "transitions/default_to_wait.json"
duration_ms = 300
easing = "ease-out"
```

### Transition Types

- **Morph**: Interpolate vertex positions between shapes
- **CrossFade**: Blend alpha between two cursors
- **Transform**: Scale/rotate between states
- **Lottie**: Use a Lottie animation for the transition

### Easing Functions

- `linear`
- `ease-in`
- `ease-out`
- `ease-in-out`
- `ease-in-quad`
- `ease-out-quad`
- `ease-in-out-quad`
- `elastic`

## Rendering Pipeline

1. **SVG Rendering**:
   - Parse SVG with usvg
   - Render to tiny-skia pixmap
   - Convert to MemoryRenderBuffer
   - Apply scale factor from base size

2. **Lottie Rendering**:
   - Parse Lottie JSON
   - Extract shapes and properties
   - Rasterize frames with custom software renderer
   - Apply hotspot and scale

3. **Transition Animation**:
   - Update progress based on elapsed time
   - Apply easing function
   - Blend or morph between states
   - Update cursor surface

## Size Handling

The system preserves the `base_size` parameter throughout:

```rust
// CursorManager stores base_size (e.g., 24)
// Scale is applied at render time (e.g., 1, 2, 3)
let actual_size = base_size * scale;

// SVG: viewBox is scaled to actual_size
// Lottie: width/height are scaled to actual_size
// Hotspot: multiplied by scale
```

## Resource Directory Structure

```
resources/cursors/
├── theme.toml                    # Configuration file
├── vectors/                      # SVG cursors
│   ├── default.svg
│   ├── move.svg
│   └── ...
└── lottie/                       # Lottie animations
    ├── loading.json
    ├── busy.json
    └── transitions/
        ├── default_to_busy.json
        └── ...
```

## Examples

See `resources/cursors/` for example cursors:
- `default.svg` - Simple arrow cursor
- `move.svg` - Move cursor with indicator
- `loading.json` - Animated loading spinner

## Integration with CursorManager

The vector system integrates seamlessly with existing XCursor support:

1. Priority: Vector > Custom Surface > XCursor > Fallback
2. Same `RenderCursor` enum for all types
3. Compatible with existing `CursorTextureCache`
4. No breaking changes to existing API

## Performance Considerations

- **Caching**: Renderers are cached per cursor ID and scale
- **Lazy Loading**: SVG/Lottie files loaded on demand
- **Memory**: Uses `MemoryRenderBuffer` for efficient GPU upload
- **Animation**: Updates triggered by smithay's frame timing

## Future Enhancements

- GPU-accelerated Lottie rendering (velato/wgpu)
- Morphing transitions between SVG paths
- Custom easing functions via Lua/Ron config
- Runtime cursor theme switching
- Per-cursor animation speed control
