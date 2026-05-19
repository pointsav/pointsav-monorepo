#!/bin/bash
set -e

REPO="content-wiki-documentation"

# Find all topic-*.md files in any subdirectory
find "$REPO" -name "topic-*.md" | while read f; do
    DIR=$(dirname "$f")
    FILENAME=$(basename "$f")
    NEW_NAME=${FILENAME#topic-}
    
    # Move file - force overwrite
    git -C "$REPO" mv -f "$(realpath --relative-to="$REPO" "$f")" "$(realpath --relative-to="$REPO" "$DIR/$NEW_NAME")"
    echo "Moved $f to $DIR/$NEW_NAME"
done
