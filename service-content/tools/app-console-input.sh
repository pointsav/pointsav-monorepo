#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

# Identity: Ingestion Gateway (Ground Control - v1.9 Surgical)
# Requires GCP_IP, REMOTE_USER, REMOTE_REPO env vars set — no real-value
# defaults (2026-08-19 GitHub-exposure remediation).
GCP_IP="${GCP_IP:?GCP_IP must be set}"
REMOTE_USER="${REMOTE_USER:?REMOTE_USER must be set}"
REMOTE_REPO="${REMOTE_REPO:?REMOTE_REPO must be set}"
HOT_ZONE="${HOT_ZONE:-$HOME/Desktop/service-content}"

echo "📡 POINTSAV SURGICAL GATEWAY ACTIVE"
cd "$HOT_ZONE/input"
files=(*)
if [ "${files[0]}" == "*" ]; then echo "❌ No files found."; exit 1; fi

select FILE in "${files[@]}"; do [ -f "$FILE" ] && break; done

echo -e "\n🏛️  SELECT TARGET DATA MESH SILO:"
declare -A SILO_MAP=(
    ["Woodfine Corporate"]="/home/foundry/node-gcp-free/fleet-woodfine/woodfine-fleet-deployment/cluster-totebox-corporate-1/service-study/corporate/assets"
    ["Woodfine Projects"]="/home/foundry/node-gcp-free/fleet-woodfine/woodfine-fleet-deployment/cluster-totebox-corporate-1/service-study/projects/assets"
    ["Technical Library"]="/home/foundry/node-gcp-free/factory-pointsav/content-wiki-documentation"
)
select SILO_NAME in "${!SILO_MAP[@]}"; do 
    SILO_PATH="${SILO_MAP[$SILO_NAME]}"
    break
done

echo "🚀 Pushing Payload..."
rsync -avz "$FILE" "$REMOTE_USER@$GCP_IP:$REMOTE_REPO/service-content/input/"

echo "⚙️  Triggering Remote Engine..."
ssh "$REMOTE_USER@$GCP_IP" "bash $REMOTE_REPO/service-content/tools/trigger_extraction.sh \"$FILE\" \"EXTRACT\" \"$SILO_PATH\""

echo "📥 Retrieving Artifact..."
rsync -avz "$REMOTE_USER@$GCP_IP:$REMOTE_REPO/service-content/outbox/" "$HOT_ZONE/output/"
mv "$FILE" "$HOT_ZONE/input/processed/"
echo "🏁 COMPLETE."
