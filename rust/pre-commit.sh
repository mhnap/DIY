#!/bin/bash

# Ensure rustfmt is installed.
if ! command -v rustfmt >/dev/null 2>&1; then
    echo "⛔ PRE-COMMIT FAILED: 'rustfmt' not found."
    echo "   You must install it to commit code to this repo."
    echo "   Run: rustup component add rustfmt"
    exit 1
fi

# Filter for staged Rust files (excluding deletions).
STAGED_RUST_FILES=$(git diff --cached --name-only --diff-filter=d -- '*.rs')

# Exit early if no Rust files are staged.
if [ -z "$STAGED_RUST_FILES" ]; then
    exit 0
fi

FAILURES=0
echo "🔍 Hook: Checking staged rust files..."

# Iterate over files using while-read to safely handle spaces in filenames.
while IFS= read -r FILE; do
    
    # 1. Check the STAGED content directly.
    # We pipe the staged version to rustfmt. This avoids checking the disk version,
    # ensuring we only validate what is actually being committed.
    # Note: We capture output (2>&1) because rustfmt prints diffs to stdout but errors to stderr.
    STAGED_FORMAT_DIFF=$(git show ":$FILE" | rustfmt --check --color always 2>&1)

    # Optimization: If the output is empty, the staged file is already perfect.
    # We continue immediately, preventing unnecessary file system writes/timestamp updates.
    if [ -z "$STAGED_FORMAT_DIFF" ]; then
        continue
    fi

    # 2. Handle Unformatted Files
    # If we are here, the staged content is messy. We must decide if we can auto-fix it.

    # Check for unstaged changes (Partial Stage).
    if ! git diff --quiet -- "$FILE"; then
        
        # CASE A: PARTIAL STAGE (Messy + Unstaged Changes) -> BLOCK
        # We cannot safely auto-format in-place because rustfmt reads from disk,
        # which would mix staged corrections with your unstaged work.
        echo "---------------------------------------------------------"
        echo "❌ ISSUE: '$FILE' is partially staged and unformatted."
        echo "   (Auto-fix skipped to protect unstaged changes)"
        echo ""
        echo "   --- Formatting Diff ---"
        echo "$STAGED_FORMAT_DIFF"
        echo "---------------------------------------------------------"
        FAILURES=1
    else
        # CASE B: FULL STAGE (Messy + Fully Staged) -> FIX
        # The file on disk matches the stage. It is safe to format in-place.
        rustfmt -q --color always "$FILE"
        git add "$FILE"
        echo "🛠️  '$FILE' (Full stage) - Formatted & Added"
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
