#!/bin/bash

# 0. Safety Check: Ensure rustfmt is available.
if ! command -v rustfmt >/dev/null 2>&1; then
    echo "⚠️  PRE-COMMIT SKIPPED: 'rustfmt' not found."
    echo "   Please install it (rustup component add rustfmt) to enable auto-formatting."
    exit 0
fi

# 1. Find all staged Rust files (excluding deletions).
STAGED_RUST_FILES=$(git diff --cached --name-only --diff-filter=d | grep '\.rs$')

# Exit early if no Rust files are staged.
if [ -z "$STAGED_RUST_FILES" ]; then
    exit 0
fi

FAILURES=0
echo "🔍 Hook: Checking staged rust files..."

# 2. Iterate over files using while-read to safely handle spaces in filenames.
while IFS= read -r FILE; do
    
    # Check if the file has unstaged changes in the working directory.
    if ! git diff --quiet -- "$FILE"; then
        
        # --- CASE A: PARTIAL STAGE (SAFETY MODE) ---
        # The file has unstaged changes. We cannot safely auto-format it in place
        # because rustfmt reads from disk and would mix staged/unstaged code.
        
        # Check staged content via pipe. Capture stderr (2>&1) for errors.
        CHECK_OUTPUT=$(git show ":$FILE" | rustfmt --check --edition 2024 --color always 2>&1)
        
        if [ -n "$CHECK_OUTPUT" ]; then
            echo "---------------------------------------------------------"
            echo "❌ ISSUE: '$FILE' is partially staged and unformatted."
            echo "   (Auto-fix skipped to protect unstaged changes)"
            echo ""
            echo "   --- Formatting Diff ---"
            echo "$CHECK_OUTPUT"
            echo "---------------------------------------------------------"
            FAILURES=1
        else
             echo "✨ '$FILE' (Partial stage) - Already formatted"
        fi

    else
        # --- CASE B: FULL STAGE (AUTO-FIX MODE) ---
        # File matches stage. Safe to format in-place.
        # Optimization: Check first to avoid touching mtime if clean.
        CHECK_OUTPUT=$(rustfmt --check --edition 2024 --color always "$FILE" 2>&1)
        
        if [ -n "$CHECK_OUTPUT" ]; then
            # Output exists -> File is messy. Fix it.
            rustfmt -q --edition 2024 --color always "$FILE"
            git add "$FILE"
            echo "🛠️  '$FILE' (Full stage) - Formatted & Added"
        else
            # Output empty -> File is already clean. Do nothing.
            echo "✨ '$FILE' (Full stage) - Already formatted"
        fi
    fi

done <<< "$STAGED_RUST_FILES"

# 3. Final Summary
if [ "$FAILURES" -eq 1 ]; then
    echo "---------------------------------------------------------"
    echo "⛔ COMMIT BLOCKED: Formatting errors found in partial stages."
    echo "💡 ACTION: Run 'just fmt' to fix the files, then stage them manually."
    echo "---------------------------------------------------------"
    exit 1
fi

exit 0
