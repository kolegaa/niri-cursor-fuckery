# Vector Cursor System - Fixed with Enhanced Logging

## What Was Fixed

### ✅ Cursor Icons Not Showing
**Root Cause**: The CursorAnimator started in `Static` state, which means no cursor was active. Vector cursors only render when in `Animated` or `Transitioning` state.

**Solution**: Initialize CursorAnimator with a default cursor from the config. This sets the initial state to `Animated` instead of `Static`.

### ✅ Added Comprehensive Logging
Added debug/info/warn logging throughout the vector cursor system:

**In `src/cursor.rs`:**
- Logs when vector system loads successfully or fails
- Logs all CursorIcon to vector cursor ID mappings
- Logs when `set_cursor_image()` is called
- Logs when vector cursor rendering succeeds or fails
- Logs current animator state for debugging

**In `src/cursor/vector/animator.rs`:**
- Logs when animator is created and initialized
- Logs when `set_cursor()` is called with cursor_id
- Logs current state before changes
- Logs when transitions are found/not found
- Logs when cursor definitions are looked up
- Logs new state after changes

**In `src/cursor/vector/store.rs`:**
- Logs when SVG renderer is being loaded
- Logs SVG file paths
- Logs when Lottie renderer is being loaded
- Logs Lottie file paths
- Logs when renderer is requested

**In `src/cursor/vector/config.rs`:**
- Logs when TOML config is being parsed
- Logs number of cursors defined
- Logs transitions available
- Logs cursor lookups
- Logs transition lookups

## How to Test and Debug

### 1. Build Project
```bash
cd /home/duck/Desktop/coding/niri
cargo build --release
```

### 2. Run with Debug Logging
```bash
# Run with full debug output
RUST_LOG=debug ./target/release/niri

# Or run just cursor module debugging
RUST_LOG=niri::cursor=debug ./target/release/niri

# Or run all niri debug output
RUST_LOG=niri=debug ./target/release/niri
```

### 3. View Logs
```bash
# View logs in real-time
journalctl -xe -u niri -f

# Save logs for review
journalctl -u niri > /tmp/niri.log

# Follow logs while testing
journalctl -xeu niri -f
```

## What You Should See in Logs

### Successful Initialization
```
INFO load_vector_system called with path: /home/duck/Desktop/coding/niri/resources/cursors, size: 24
INFO Vector cursor system loaded successfully
INFO Vector system available, mapping CursorIcon to vector cursor IDs
INFO Available cursors in config: {"default", "move", "text", "wait", "progress", "crosshair", "nwse-resize"}
DEBUG Processing cursor ID: 'default'
DEBUG Mapped cursor icon Default to vector cursor 'default'
DEBUG Processing cursor ID: 'move'
DEBUG Mapped cursor icon AllScroll to vector cursor 'move'
DEBUG Processing cursor ID: 'text'
DEBUG Mapped cursor icon Text to vector cursor 'text'
INFO Mapped 6 cursor icons to vector cursors
INFO Config parsed successfully with 7 cursors defined
DEBUG CursorAnimator created, initializing with default cursor
DEBUG Found cursor definition, setting state to Animated with loop_mode: Loop
DEBUG Initialized CursorAnimator with default cursor
```

### When Cursor Changes
```
DEBUG set_cursor_image called with cursor: Named(Default)
DEBUG CursorAnimator::set_cursor called with cursor_id: 'default'
DEBUG Current state is Animated with cursor: 'default'
DEBUG Already showing cursor 'default', no change needed
```

### When Rendering Vector Cursor
```
DEBUG Attempting to render vector cursor
DEBUG get_vector_cursor called with scale: 1
DEBUG Current animator state: Animated { cursor_id: "default", start_time_ms: 0, loop_mode: Loop }
DEBUG State is Animated with cursor: 'default'
DEBUG Getting renderer for cursor: 'default'
INFO Loading SVG renderer for cursor: 'default'
DEBUG SVG file path: /home/duck/Desktop/coding/niri/resources/cursors/vectors/default.svg
DEBUG Renderer obtained, rendering frame 0
DEBUG Frame rendered successfully
DEBUG Successfully rendered vector cursor
```

## Troubleshooting Guide

### Problem: Cursers Still Don't Show

**Check 1**: Is vector system loaded?
Look for this log:
```
INFO Vector cursor system loaded successfully
```

**If missing**: Check path in `src/niri.rs` line 2339

**Check 2**: Is default cursor mapped?
Look for this log:
```
INFO Mapped cursor icon Default to vector cursor 'default'
```

**If missing**: Default cursor not defined in theme.toml or mapping failed

**Check 3**: Is animator initialized?
Look for this log:
```
DEBUG Initialized CursorAnimator with default cursor
```

**If missing**: Default cursor definition not found in config

**Check 4**: Is set_cursor_image being called?
Look for this log when cursor should change:
```
DEBUG set_cursor_image called with cursor: Named(...)
```

**If missing**: Cursor state not being set by Niri

### Problem: Fallback to XCursor

Look for this log:
```
WARN Vector cursor rendering failed, falling back: ...
```

**Common causes**:
- SVG file not found
- SVG file is invalid
- No mapping for CursorIcon
- Animator in Static state

### Problem: Only Some Cursors Work

Look for this log:
```
DEBUG No CursorIcon match for cursor ID: 'some_cursor'
```

**Solution**: Add mapping in `src/cursor.rs` mapping section (around line 100)

### Problem: Performance Issues

Look for these logs during cursor changes:
- Long delays between "Getting renderer" and "Frame rendered"
- Repeated renderer lookups (not caching properly)

**Solutions**:
- Simplify SVG paths
- Reduce Lottie complexity
- Check renderer caching is working

## Quick Debugging Checklist

Run this checklist to identify issues:

```bash
# 1. Enable debug logging
export RUST_LOG=niri::cursor=debug

# 2. Start Niri
./target/release/niri

# 3. In another terminal, monitor logs
journalctl -xeu niri -f

# 4. Look for these markers:
#    ✓ "Vector cursor system loaded successfully"
#    ✓ "Initialized CursorAnimator with default cursor"
#    ✓ "Mapped cursor icon Default to vector cursor 'default'"
#    ✓ "Successfully rendered vector cursor"

# If any marker is missing, that's your issue!
```

## Cursor Mapping Reference

These are the mappings defined in `src/cursor.rs`:

| Cursor ID | CursorIcon | Description |
|-----------|-------------|-------------|
| `default` | `Default` | Default arrow cursor |
| `move` | `AllScroll` | Move/drag cursor |
| `text` | `Text` | Text input cursor |
| `wait` | `Wait` | Loading cursor |
| `crosshair` | `Crosshair` | Precision cursor |
| `nwse-resize` | `NwResize` | Resize diagonal |

To add more cursors, update the mapping in `src/cursor.rs` and define them in `theme.toml`.

## Files Modified

1. **src/cursor.rs** - Added logging and cursor icon mapping
2. **src/cursor/vector/animator.rs** - Added initialization with default cursor, extensive logging
3. **src/cursor/vector/store.rs** - Added logging for renderer loading
4. **src/cursor/vector/config.rs** - Added logging for config parsing
5. **src/cursor/vector/types.rs** - Added Debug derives to enable logging

## Next Steps

### If Cursors Work
Great! Your vector cursor system is working:

1. Add more SVG cursors to `theme.toml`
2. Add Lottie animations for complex cursors
3. Create transitions between cursor states
4. Customize cursor designs in SVG files

### If Cursors Don't Work

1. Check logs using `RUST_LOG=niri::cursor=debug`
2. Verify files exist at correct paths
3. Validate SVG/Lottie files
4. Check CursorIcon mappings
5. Review `DEBUG_GUIDE.md` for detailed troubleshooting

## Getting Help

Share these logs when asking for help:

1. **Initialization logs** (when Niri starts)
2. **Cursor change logs** (when hovering over different elements)
3. **Error logs** (any WARN or ERROR messages)

Example:
```
# Copy these lines from your journalctl output:
INFO Vector cursor system loaded successfully
INFO Mapped 6 cursor icons to vector cursors
DEBUG CursorAnimator::set_cursor called with cursor_id: 'default'
```

---

**The vector cursor system is now ready to test with comprehensive logging!**

Run: `RUST_LOG=niri::cursor=debug ./target/release/niri`
