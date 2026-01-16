#!/bin/bash
# Test script for vector cursor system

set -e

echo "=== Vector Cursor System Test Script ==="
echo ""

# Check if resources exist
echo "1. Checking resources..."
if [ -d "resources/cursors" ]; then
    echo "✓ resources/cursors directory exists"
else
    echo "✗ resources/cursors directory not found"
    exit 1
fi

if [ -f "resources/cursors/theme.toml" ]; then
    echo "✓ theme.toml exists"
else
    echo "✗ theme.toml not found"
    exit 1
fi

# Count cursor files
SVG_COUNT=$(find resources/cursors/vectors -name "*.svg" 2>/dev/null | wc -l)
LOTTIE_COUNT=$(find resources/cursors/lottie -name "*.json" 2>/dev/null | wc -l)

echo "✓ Found $SVG_COUNT SVG cursor(s)"
echo "✓ Found $LOTTIE_COUNT Lottie cursor(s)"

echo ""
echo "2. Building project..."
cargo build --release 2>&1 | tail -5

echo ""
echo "3. Running syntax checks..."

# Check theme.toml syntax
echo "Checking theme.toml syntax..."
if command -v toml2json &> /dev/null; then
    toml2json resources/cursors/theme.toml > /dev/null && echo "✓ theme.toml is valid TOML" || echo "✗ theme.toml has syntax errors"
else
    echo "  (skipped - toml2json not installed)"
fi

# Check SVG files
echo "Checking SVG files..."
for svg in resources/cursors/vectors/*.svg; do
    if [ -f "$svg" ]; then
        if command -v xmllint &> /dev/null; then
            xmllint --noout "$svg" 2>&1 && echo "✓ $(basename $svg)" || echo "✗ $(basename $svg) has errors"
        else
            echo "  ✓ $(basename $svg) (not validated - xmllint not installed)"
        fi
    fi
done

# Check Lottie files
echo "Checking Lottie files..."
for json in resources/cursors/lottie/*.json; do
    if [ -f "$json" ]; then
        if command -v jq &> /dev/null; then
            jq empty "$json" 2>&1 && echo "✓ $(basename $json)" || echo "✗ $(basename $json) has errors"
        else
            echo "  ✓ $(basename $json) (not validated - jq not installed)"
        fi
    fi
done

echo ""
echo "4. Configuration Summary"
echo "---"
grep -E "^\[cursors\." resources/cursors/theme.toml | sed 's/\[cursors\.//g' | sed 's/\]//g'
echo ""
grep -E "^\[transitions" resources/cursors/theme.toml

echo ""
echo "=== Test Complete ==="
echo ""
echo "To run Niri with vector cursors:"
echo "  ./target/release/niri"
echo ""
echo "To run with cargo:"
echo "  cargo run --release"
echo ""
echo "Check logs for any vector cursor errors:"
echo "  journalctl -xe -u niri"
