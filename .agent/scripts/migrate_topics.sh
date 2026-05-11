#!/bin/bash
set -e

cd content-wiki-documentation

for f in topic-*.md; do
    # Extract category from frontmatter
    CAT=$(grep "^category: " "$f" | head -1 | awk '{print $2}')
    
    # Default to reference if not found
    if [ -z "$CAT" ]; then
        CAT="reference"
    fi
    
    # Remove topic- prefix
    NEW_NAME=${f#topic-}
    
    # Destination directory
    mkdir -p "$CAT"
    
    # Move file - force overwrite if exists
    git mv -f "$f" "$CAT/$NEW_NAME"
    echo "Moved $f to $CAT/$NEW_NAME"
done
