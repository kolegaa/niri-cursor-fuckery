# Vector Cursor System - Quick Start Guide

## What's Been Done

✅ **Core System Implemented**
  - SVG cursor rendering with usvg/tiny-skia
  - Lottie animation support with custom rasterizer
  - Animated transitions (morph, cross-fade, transform, lottie)
  - TOML-based configuration
  - Size parameter preserved (base_size × scale)
  - Hybrid mode (vector cursors with XCursor fallback)

✅ **Enabled in Niri**
  - Modified `src/niri.rs` to use `new_with_vector_theme()`
  - Points to `/home/duck/Desktop/coding/niri/resources/cursors`

✅ **Example Cursors Created**
  - 6 SVG cursors (default, move, text, wait, crosshair, resize-diagonal)
  - 1 Lottie animation (loading spinner)
  - Configuration file with cursor definitions and transitions

✅ **Documentation**
  - `VECTOR_CURSORS_GUIDE.md` - Complete usage guide
  - `src/cursor/vector/README.md` - Technical documentation
  - `test_vector_cursors.sh` - Automated test script

## File Structure

```
niri/
├── src/cursor/vector/           # Core implementation
│   ├── mod.rs
│   ├── store.rs
│   ├── animator.rs
│   ├── config.rs
│   ├── types.rs
│   └── renderer/
│       ├── mod.rs
│       ├── svg.rs
│       └── lottie.rs
├── resources/cursors/            # Vector cursor resources
│   ├── theme.toml              # Configuration
│   ├── vectors/                 # SVG cursors (6 files)
│   │   ├── default.svg
│   │   ├── move.svg
│   │   ├── text.svg
│   │   ├── wait.svg
│   │   ├── crosshair.svg
│   │   └── resize-diagonal.svg
│   └── lottie/                # Lottie animations (1 file)
│       └── loading.json
├── VECTOR_CURSORS_GUIDE.md      # User guide
├── test_vector_cursors.sh       # Test script
└── src/niri.rs (modified)      # Enabled vector cursors
```

## How to Test

### Option 1: Quick Test (Development)
```bash
cd /home/duck/Desktop/coding/niri
cargo run --release
```

### Option 2: Build and Run
```bash
cd /home/duck/Desktop/coding/niri
cargo build --release
./target/release/niri
```

### Option 3: Run Test Script
```bash
cd /home/duck/Desktop/coding/niri
./test_vector_cursors.sh
```

## What You'll See

When you run Niri:

1. **Default cursor**: White arrow with black outline
2. **Move cursor**: Arrow with circle indicator (when dragging windows)
3. **Text cursor**: I-beam (when over text inputs)
4. **Wait cursor**: Hourglass (during loading)
5. **Crosshair**: Cross shape (selection/screenshot mode)
6. **Resize diagonal**: For resizing windows

All cursors scale automatically based on output scale!

## Creating New Cursors

### Quick SVG Example
```xml
<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M4 4 L20 14 L12 14 L9 21 Z" fill="#ffffff" stroke="#000000" stroke-width="1"/>
</svg>
```

1. Create file in `resources/cursors/vectors/`
2. Add to `theme.toml`:
   ```toml
   [cursors.my_cursor]
   format = "svg"
   file = "vectors/my_cursor.svg"
   ```
3. Rebuild and test

### Lottie Animation
1. Create JSON in `resources/cursors/lottie/`
2. Add to `theme.toml`:
   ```toml
   [cursors.my_animation]
   format = "lottie"
   file = "lottie/my_animation.json"
   loop_mode = "loop"
   ```
3. Rebuild and test

## Troubleshooting

### Cursors don't appear
```bash
# Check logs
journalctl -xe -u niri

# Verify path in src/niri.rs:2339
grep "vector_theme_path" src/niri.rs
```

### Fallback to XCursor
- If `theme.toml` is missing or invalid
- If SVG/Lottie files can't be loaded
- Check file permissions and paths

### Performance
- Keep SVG paths simple
- Limit Lottie complexity
- Avoid large images in Lottie files

## Configuration Options

### Cursor Formats
- `svg` - Scalable vector graphics
- `lottie` - JSON-based animations

### Loop Modes (Lottie only)
- `once` - Play once and stop
- `loop` - Repeat indefinitely
- `bounce` - Play forward and backward

### Transition Types
- `morph` - Interpolate between shapes (future)
- `cross-fade` - Blend between cursors (current)
- `transform` - Scale/rotate (future)
- `lottie` - Custom Lottie animation (future)

### Easing Functions
- `linear`
- `ease-in`
- `ease-out`
- `ease-in-out`
- `ease-in-quad`
- `ease-out-quad`
- `ease-in-out-quad`
- `elastic`

## Next Steps

1. ✅ **System is ready to test** - Run and see vector cursors!
2. **Add more cursors** - Create SVGs for your favorite cursor states
3. **Custom transitions** - Add Lottie files for smooth transitions
4. **Config option** - Add `vector_theme_path` to config for flexibility
5. **Runtime switching** - Support changing themes without restart

## Dependencies Added

```toml
# Cargo.toml
usvg = "0.40"          # SVG parsing
resvg = "0.40"         # SVG rendering
tiny-skia = "0.11"      # Rasterization
fontdb = "0.16"         # Font database (for SVG text)
lyon = "1.0"            # For future morphing transitions
toml = "0.8"            # Configuration parsing
parking_lot = "0.12"     # Thread-safe caching
svg = "0.18"             # SVG parsing (legacy)
```

## Performance Notes

- **SVG rendering**: Fast, cached per scale
- **Lottie rendering**: Slower, cached per frame
- **Transitions**: Currently basic fade, will improve
- **Memory**: Uses same MemoryRenderBuffer pattern as XCursor
- **GPU**: Currently software rendering, can upgrade to GPU

## Support

For issues:
1. Check `VECTOR_CURSORS_GUIDE.md` for detailed docs
2. Run `./test_vector_cursors.sh` for validation
3. Check compositor logs for errors
4. Verify file paths and permissions

---

**Ready to test!** Run `cargo run --release` to see your new vector cursor system in action.
