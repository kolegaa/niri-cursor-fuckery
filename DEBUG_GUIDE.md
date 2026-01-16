# Vector Cursor Debugging Guide

## Changes Made to Add Logging

### 1. Enhanced CursorManager Initialization
**File**: `src/cursor.rs`
- Added logging when loading vector system
- Added logging when vector cursor fails to load
- Logs success/failure of vector system loading

### 2. Added Cursor Icon to Vector ID Mapping
**File**: `src/cursor.rs`
- Maps CursorIcon enum variants to cursor IDs in theme.toml
- Handles common XCursor theme names (default, move, text, wait, etc.)
- Logs each mapping as it's created

### 3. Enhanced CursorAnimator
**File**: `src/cursor/vector/animator.rs`
- Added debug logging for state transitions
- Logs current state before changes
- Logs when transitions are found or not found
- Logs cursor definition lookups

### 4. Enhanced VectorCursorStore
**File**: `src/cursor/vector/store.rs`
- Added debug logging for renderer lookups
- Logs file paths when loading SVG/Lottie files
- Logs config parsing steps

### 5. Enhanced Config Parser
**File**: `src/cursor/vector/config.rs`
- Added debug logging for TOML parsing
- Logs number of cursors and transitions defined
- Logs cursor lookup results
- Logs transition lookup results

### 6. Added Debug Derives
**File**: `src/cursor/vector/types.rs`
- Added `#[derive(Debug)]` to LoopMode and TransitionState
- Enables debug logging of states

## How to Test

### 1. Build the Project
```bash
cd /home/duck/Desktop/coding/niri
cargo build --release
```

### 2. Run Niri with Logging
```bash
# Run with full debug output
RUST_LOG=debug ./target/release/niri

# Or run specific module logging
RUST_LOG=niri::cursor=debug ./target/release/niri
```

### 3. View Logs
```bash
# View logs in real-time
journalctl -xe -u niri -f

# Or save logs to file
journalctl -u niri > /tmp/niri.log
```

## What to Look For in Logs

### Successful Vector System Loading
You should see:
```
INFO load_vector_system called with path: /home/duck/Desktop/coding/niri/resources/cursors, size: 24
INFO Vector cursor system loaded successfully
INFO Vector system available, mapping CursorIcon to vector cursor IDs
INFO Available cursors in config: {"default", "move", "text", "wait", ...}
INFO Mapped cursor icon Default to vector cursor 'default'
INFO Mapped cursor icon AllScroll to vector cursor 'move'
INFO Mapped cursor icon Text to vector cursor 'text'
INFO Mapped 6 cursor icons to vector cursors
```

### Cursor Change Events
When cursor changes (hovering over different elements):
```
DEBUG set_cursor_image called with cursor: Named(Default)
DEBUG CursorAnimator::set_cursor called with cursor_id: 'default'
DEBUG Current state is Static
DEBUG Found cursor definition, setting state to Animated with loop_mode: Loop
DEBUG get_vector_cursor called with scale: 1
DEBUG Current animator state: Animated { cursor_id: "default", ... }
DEBUG Getting renderer for cursor: 'default'
INFO Loading SVG renderer for cursor: 'default'
INFO SVG file path: /home/duck/Desktop/coding/niri/resources/cursors/vectors/default.svg
DEBUG Renderer obtained, rendering frame 0
DEBUG Frame rendered successfully
DEBUG Successfully rendered vector cursor
```

### Errors to Watch For

#### Config Not Found
```
WARN Failed to load vector cursor system: ...
WARN Vector cursor rendering failed, falling back: ...
```
→ Check that `theme.toml` exists at the specified path

#### SVG File Not Found
```
ERROR Failed to read SVG file: /path/to/cursor.svg
```
→ Check that SVG files exist in `vectors/` directory

#### No Mapping for Cursor
```
DEBUG No vector cursor mapping for icon: SomeIcon
```
→ Cursor type not defined in theme.toml or mapping is incomplete

#### Rendering Failed
```
WARN Vector cursor rendering failed, falling back: ...
```
→ SVG/Lottie file is invalid or renderer failed

## Common Issues and Solutions

### Issue: Vector cursors don't show up, only XCursor

**Check logs for**:
```
WARN Vector cursor rendering failed, falling back: No active cursor
```

**Solution**: The animator starts in Static state. You need to call `set_cursor_image()` at least once.

Add to your initialization code:
```rust
cursor_manager.set_cursor_image(CursorImageStatus::default_named());
```

### Issue: All cursors fall back to XCursor

**Check logs for**:
```
WARN Could not parse 'some_cursor_id' as CursorIcon
```

**Solution**: The cursor ID in `theme.toml` doesn't match any known pattern. Check the mapping in `src/cursor.rs` around line 100.

**Common cursor names that should work**:
- `default` → CursorIcon::Default
- `move` → CursorIcon::AllScroll
- `text` → CursorIcon::Text
- `wait` → CursorIcon::Wait
- `crosshair` → CursorIcon::Crosshair

### Issue: Only some cursors work

**Check logs for**:
```
DEBUG No CursorIcon match for cursor ID: 'custom_cursor'
```

**Solution**: Add mapping for your custom cursor in `src/cursor.rs` mapping section (around line 100).

Example:
```rust
"my-custom" => CursorIcon::Default,  // Or appropriate variant
```

### Issue: SVG files fail to load

**Check logs for**:
```
ERROR Failed to parse SVG: ...
```

**Solution**: Validate SVG files:
```bash
# Use xmllint to validate XML
xmllint --noout resources/cursors/vectors/*.svg
```

Common SVG issues:
- Missing `xmlns="http://www.w3.org/2000/svg"`
- Invalid path data
- Unclosed tags
- Invalid viewBox

### Issue: Lottie fails to load

**Check logs for**:
```
ERROR Failed to parse Lottie JSON: ...
```

**Solution**: Validate Lottie JSON:
```bash
# Use jq to validate JSON
jq empty resources/cursors/lottie/*.json
```

## Quick Debug Checklist

- [ ] Vector system loads successfully (check logs)
- [ ] At least one cursor is mapped (check logs)
- [ ] `set_cursor_image()` is called at initialization
- [ ] SVG/Lottie files exist at correct paths
- [ ] `theme.toml` is valid TOML
- [ ] CursorIcon mappings are correct
- [ ] No errors in logs during cursor changes
- [ ] Debug logging is enabled (`RUST_LOG=debug`)

## Testing Specific Scenarios

### Test 1: Default Cursor
```bash
# Should show arrow cursor
# Check logs for: "default" cursor being rendered
```

### Test 2: Text Input Cursor
```bash
# Hover over text input field
# Should show I-beam cursor
# Check logs for: "text" cursor being rendered
```

### Test 3: Move Cursor
```bash
# Drag a window
# Should show move cursor
# Check logs for: "move" or "all-scroll" being rendered
```

### Test 4: Resize Cursor
```bash
# Resize window from corner
# Should show resize cursor
# Check logs for: "nwse-resize" being rendered
```

## Next Steps After Debugging

### If Everything Works
- ✓ All cursors are showing correctly
- ✓ Logs show successful rendering
- → You have a working vector cursor system!
- → Add more SVG/Lottie cursors to theme.toml
- → Create custom transitions

### If Cursors Don't Show
1. Check logs for specific error messages
2. Verify file paths and permissions
3. Validate SVG/Lottie files
4. Check CursorIcon mappings
5. Verify `set_cursor_image()` is being called

### If Performance Issues
- Reduce SVG path complexity
- Limit Lottie frame count
- Cache rendered frames
- Use GPU rendering (future enhancement)

## Getting Help

If logs don't reveal the issue:

1. **Enable full debug output**:
   ```bash
   RUST_LOG=debug ./target/release/niri
   ```

2. **Save logs for review**:
   ```bash
   journalctl -u niri > /tmp/niri-debug.log
   ```

3. **Share logs with context**:
   - What cursor state are you in?
   - What action triggered the cursor change?
   - What logs appear around that action?

## Log Examples

### Working System
```
INFO Vector cursor system loaded successfully
INFO Mapped 6 cursor icons to vector cursors
DEBUG CursorAnimator::set_cursor called with cursor_id: 'default'
DEBUG Found cursor definition, setting state to Animated
DEBUG get_vector_cursor called with scale: 1
DEBUG Successfully rendered vector cursor
```

### System with Issues
```
WARN Failed to load vector cursor system: Failed to read config file
INFO No vector system available, no cursor icon mapping
DEBUG Vector cursor rendering failed, falling back: No active cursor
```

---

**Remember**: The vector cursor system falls back gracefully to XCursor if anything goes wrong. If you're not seeing vector cursors, check the logs to find out why!
