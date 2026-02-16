#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
MERGED="$SCRIPT_DIR/merged.py"

echo "Building ai-mergetool..."
cargo build --manifest-path "$PROJECT_DIR/Cargo.toml" --quiet

echo "Running ai-mergetool on example merge conflict..."
echo "  BASE:   base.py   (original)"
echo "  LOCAL:  local.py  (added logging)"
echo "  REMOTE: remote.py (added password hashing + audit log)"
echo ""

"$PROJECT_DIR/target/debug/ai-mergetool" \
    "$SCRIPT_DIR/base.py" \
    "$SCRIPT_DIR/local.py" \
    "$SCRIPT_DIR/remote.py" \
    "$MERGED"

echo ""
echo "--- Merged output ---"
cat "$MERGED"

if [ -f "$SCRIPT_DIR/expected-merged.py" ]; then
    echo ""
    if diff -q "$MERGED" "$SCRIPT_DIR/expected-merged.py" > /dev/null 2>&1; then
        echo "PASS: Merged output matches expected result."
    else
        echo "DIFF: Merged output differs from expected (this is acceptable since LLM output varies):"
        diff --color=auto "$SCRIPT_DIR/expected-merged.py" "$MERGED" || true
    fi
fi

rm -f "$MERGED"
