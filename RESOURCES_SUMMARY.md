# Vector Cursor Resources Summary

## Available Cursors

### SVG Cursors (6 total)

#### 1. Default Cursor (`default.svg`)
**Type**: Arrow pointer
**Purpose**: Default cursor for most interactions
**Visual**: White arrow with black outline, pointing top-left
**Hotspot**: (0, 0) - Tip of the arrow

#### 2. Move Cursor (`move.svg`)
**Type**: Move indicator
**Purpose**: When dragging windows or elements
**Visual**: White arrow with circle indicator at tip
**Hotspot**: (0, 0) - Tip of the arrow

#### 3. Text Cursor (`text.svg`)
**Type**: I-beam
**Purpose**: Text input and selection
**Visual**: Vertical bar
**Hotspot**: (10, 4) - Centered horizontally

#### 4. Wait Cursor (`wait.svg`)
**Type**: Loading/busy indicator
**Purpose**: During operations that take time
**Visual**: Rotating hourglass-style shape
**Hotspot**: (12, 12) - Center

#### 5. Crosshair (`crosshair.svg`)
**Type**: Precision cursor
**Purpose**: Screenshot, selection, precision tools
**Visual**: Cross shape with horizontal and vertical lines
**Hotspot**: (12, 12) - Center

#### 6. Resize Diagonal (`resize-diagonal.svg`)
**Type**: Window resize
**Purpose**: Resizing windows diagonally
**Visual**: Two triangles at opposite corners
**Hotspot**: (4, 4) - Top-left triangle

### Lottie Animations (1 total)

#### 1. Loading Animation (`loading.json`)
**Type**: Spinner
**Purpose**: Background loading/busy state
**Visual**: Rotating circle that scales up and down
**Duration**: 60 frames (1 second at 60fps)
**Loop**: Infinite
**Hotspot**: (12, 12) - Center

## File Details

```
resources/cursors/
├── theme.toml (1.9 KB) - Configuration file
│
├── vectors/ (1.4 KB total) - SVG cursors
│   ├── default.svg (241 B)
│   ├── move.svg (264 B)
│   ├── text.svg (185 B)
│   ├── wait.svg (206 B)
│   ├── crosshair.svg (241 B)
│   └── resize-diagonal.svg (276 B)
│
└── lottie/ (1.9 KB total) - Lottie animations
    └── loading.json (1.9 KB)
```

## Configuration Summary

### Defined Cursors
- `default` - SVG
- `move` - SVG
- `text` - SVG
- `wait` - SVG
- `progress` - Lottie
- `crosshair` - SVG
- `nwse-resize` - SVG

### Defined Transitions
- `default -> move` - Morph, 200ms, ease-in-out

## Usage

### Default State
Shows when no specific cursor is needed (most of the time).

### Text State
Shows when hovering over text inputs or editable areas.

### Move State
Shows when dragging windows or movable elements.

### Wait State
Shows during loading operations (progress spinner).

### Crosshair State
Shows during screenshot or precision selection.

### Resize State
Shows when resizing windows from corner.

## Design Guidelines

### SVG Cursors
- **Size**: 24x24 (scales automatically)
- **Colors**: White fill (#ffffff) with black stroke (#000000)
- **Stroke Width**: 1px for crisp edges
- **Hotspot**: Specified in theme.toml or defaults to (0,0)

### Lottie Animations
- **Frame Rate**: 60fps recommended
- **Duration**: 1-2 seconds for seamless loops
- **Size**: 24x24 (scales automatically)
- **Complexity**: Keep layers low for performance

## Scaling

All cursors automatically scale based on output scale:
- Scale 1x: 24x24 pixels
- Scale 2x: 48x48 pixels
- Scale 3x: 72x72 pixels

SVG vectors scale perfectly, Lottie rasterizes at the target size.

## Performance

### SVG Caching
- Renderers cached per cursor ID and scale
- First render creates cache, subsequent uses cached
- Memory efficient using same buffer pattern as XCursor

### Lottie Caching
- Frames rendered on-demand
- Can cache per frame for better performance
- Higher memory usage but smoother animation

## Customization

### Add New SVG Cursor
1. Create SVG file in `vectors/`
2. Add entry to `theme.toml`
3. Rebuild

### Add New Lottie Animation
1. Create JSON file in `lottie/`
2. Add entry to `theme.toml`
3. Rebuild

### Modify Existing Cursor
1. Edit SVG/Lottie file directly
2. Changes apply after rebuild

## Tools

### Creating SVG Cursors
- **Inkscape**: Free, powerful vector editor
- **Adobe Illustrator**: Professional vector editor
- **Figma**: Modern design tool
- **Code editor**: Write SVG directly

### Creating Lottie Animations
- **LottieFiles**: https://lottiefiles.com (animations and tools)
- **After Effects**: Export via Bodymovin plugin
- **Lottie Lab**: Online editor
- **Code editor**: Write JSON directly

## Examples

### Simple Arrow Cursor
```xml
<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M4.5 4.5 L19.5 14.5 L12 14.5 L9 21.5 Z" 
        fill="#ffffff" stroke="#000000" stroke-width="1"/>
</svg>
```

### Simple I-Beam Cursor
```xml
<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <rect x="10" y="4" width="4" height="16" 
        fill="#ffffff" stroke="#000000" stroke-width="1"/>
</svg>
```

## Troubleshooting

### Cursor Not Showing
1. Check file exists in `vectors/` or `lottie/`
2. Verify entry in `theme.toml`
3. Check logs for parsing errors

### Cursor Distorted
1. Verify viewBox is 0 0 24 24
2. Check stroke width isn't too thin
3. Ensure colors provide contrast

### Animation Choppy
1. Reduce Lottie complexity
2. Increase frame rate (60fps)
3. Use GPU rendering (future)

## Support

For detailed documentation:
- See `VECTOR_CURSORS_GUIDE.md` - Complete usage guide
- See `src/cursor/vector/README.md` - Technical documentation
- See `test_vector_cursors.sh` - Automated testing

For issues:
- Check compositor logs: `journalctl -xe -u niri`
- Run test script: `./test_vector_cursors.sh`
- Verify file paths and permissions

---

**Total Resources**: 7 cursor files + 1 configuration
**Total Size**: ~3.3 KB
**Status**: Ready to use ✅
