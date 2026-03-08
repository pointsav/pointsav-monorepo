#!/usr/bin/env bash
# Purpose: Reconfigure GCP repositories to pull ONLY necessary code + workspace metadata.
NODE_ROOT="$HOME/node-gcp-free/factory-pointsav"
mkdir -p "$NODE_ROOT"
cd "$NODE_ROOT"

echo "🔬 CONFIGURING SURGICAL COMPUTE NODE..."

# DNA Pull
if [ ! -d "pointsav-design-system/.git" ]; then
    git clone --filter=blob:none --no-checkout https://github.com/pointsav/pointsav-design-system.git
    cd pointsav-design-system
    git sparse-checkout init --cone
    git sparse-checkout set tokens/linguistic
    git checkout main
    cd ..
fi

# Muscle Pull (Monorepo)
if [ ! -d "pointsav-monorepo/.git" ]; then
    git clone --filter=blob:none --no-checkout https://github.com/pointsav/pointsav-monorepo.git
    cd pointsav-monorepo
    git sparse-checkout init --cone
    # Pull ONLY the folders needed to satisfy the root Cargo.toml workspace
    git sparse-checkout set service-content system-security system-audit system-resolution system-verification os-infrastructure system-network-interface Cargo.toml
    git checkout main
    cd ..
fi

echo "⚙️  Building Engine..."
cd "$NODE_ROOT/pointsav-monorepo/service-content"
mkdir -p input outbox
cargo build --release -p service-content
chmod +x tools/trigger_extraction.sh
echo "✅ SURGICAL NODE READY."
