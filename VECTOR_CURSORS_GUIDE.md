# Testing Vector Cursor System

## Quick Start

### 1. Build Niri

```bash
cd /home/duck/Desktop/coding/niri
cargo build --release
```

### 2. Run Niri

```bash
# Run the compiled binary
./target/release/niri

# Or run with cargo
cargo run --release
```

### 3. Test Cursor Changes

Once Niri is running, you should see the vector cursors:
- **Arrow cursor** (default.svg) - Simple white arrow with black outline
- **Move cursor** (move.svg) - Arrow with circle indicator
- **Text cursor** (text.svg) - I-beam for text input
- **Wait cursor** (wait.svg) - Animated hourglass-style shape
- **Crosshair** (crosshair.svg) - Cross-shaped cursor
- **Resize diagonal** (resize-diagonal.svg) - For resizing windows

## Creating Your Own Cursors

### SVG Cursors

Create SVG files in `resources/cursors/vectors/`:

```xml
<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M4.5 4.5 L19.5 14.5 L12 14.5 L9 21.5 Z" fill="#ffffff" stroke="#000000" stroke-width="1"/>
</svg>
```

Important:
- Use 24x24 viewBox (scales automatically)
- Use white fill (`#ffffff`) and black stroke (`#000000`) for visibility
- Keep paths simple for performance
- Define hotspot in theme.toml if not at (0,0)

### Lottie Animations

Create JSON files in `resources/cursors/lottie/`:

```json
{
  "v": "5.9.0",
  "fr": 60,
  "w": 24,
  "h": 24,
  "layers": [...]
}
```

Example: See `loading.json` for a spinning circle animation

### Configuration

Add entries to `theme.toml`:

```toml
[cursors.your_cursor]
format = "svg"  # or "lottie"
file = "vectors/your_cursor.svg"
hotspot = [0, 0]  # Optional, defaults to (0,0)
loop_mode = "loop"  # For lottie: "once", "loop", or "bounce"

[transitions."default->your_cursor"]
transition_type = "morph"  # "morph", "cross-fade", "transform", or "lottie"
duration_ms = 200
easing = "ease-in-out"
```

## Testing Transitions

Transitions occur automatically when cursor state changes. To test:

1. **Default to Move**: Hover over a draggable element
2. **Default to Text**: Click in a text input field
3. **Default to Wait**: Trigger a loading state
4. **Default to Crosshair**: Use screenshot or selection tools

Currently transitions are basic (fade between states). Full morphing transitions require additional implementation.

## Troubleshooting

### Cursor doesn't change
- Check logs: `journalctl -xe -u niri` or view compositor logs
- Verify `theme.toml` syntax is correct
- Check that SVG/Lottie files exist and are valid

### Fallback to XCursor
- Vector cursor loading errors are silent - it falls back to XCursor
- Check that `/home/duck/Desktop/coding/niri/resources/cursors/theme.toml` exists
- Verify path in `src/niri.rs` line 2339

### Performance issues
- SVG files with many paths are slower to render
- Lottie animations with many layers are resource-intensive
- Consider simplifying artwork for better performance

## Next Steps

1. **Create more cursors**: Add common cursor types from your theme
2. **Custom transitions**: Add Lottie files for smooth transitions
3. **Per-theme configuration**: Move cursor path to config file
4. **Runtime switching**: Support switching cursor themes without restart
5. **GPU rendering**: Implement velato for hardware-accelerated Lottie

## Example Workflow

1. Design cursor in Inkscape/Illustrator
2. Export as SVG (24x24 or larger)
3. Place in `resources/cursors/vectors/`
4. Add entry to `theme.toml`
5. Rebuild and test
6. Iterate!

## Resources

- **SVG format**: https://developer.mozilla.org/en-US/docs/Web/SVG
- **Lottie format**: https://lottiefiles.github.io/lottie-docs/
- **usvg docs**: https://docs.rs/usvg/
- **Example cursors**: Check `resources/cursors/` directory
