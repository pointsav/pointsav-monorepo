#!/usr/bin/env bash
# ==============================================================================
# Script: app-console-input.sh
# Identity: Ingestion Gateway (Ground Control)
# Logic: Desktop Airlock -> GCP Extraction -> Data Mesh Persistence
# ==============================================================================

# --- Configuration ---
GCP_IP="35.212.238.174" 
REMOTE_USER="mathew"
REMOTE_ROOT="/home/foundry/node-gcp-free/factory-pointsav/pointsav-monorepo"
HOT_ZONE="$HOME/Desktop/service-content"
LOG_FILE="$HOT_ZONE/logs/ingestion.log"

echo "📡 POINTSAV INGESTION GATEWAY ACTIVE"
echo "------------------------------------------------"

# 1. Scan Desktop Airlock for input
cd "$HOT_ZONE/input"
files=(*)
if [ "${#files[@]}" -eq 0 ] || [ "${files[0]}" == "*" ]; then
    echo "❌ ERROR: No files found in Desktop/service-content/input"
    exit 1
fi

echo "📂 FILES READY FOR EXTRACTION:"
PS3="Select a file to process (number): "
select FILE in "${files[@]}"; do
    if [ -n "$FILE" ] && [ -f "$FILE" ]; then break; fi
    echo "Invalid selection."
done

# 2. Select the Data Mesh Silo on the Cloud Node
echo -e "\n🏛️  SELECT TARGET DATA MESH SILO (Remote Persistence):"
declare -A SILO_MAP=(
    ["Woodfine Corporate"]="/home/foundry/node-gcp-free/fleet-woodfine/woodfine-fleet-deployment/cluster-totebox-corporate-1/service-study/corporate/assets"
    ["Woodfine Projects"]="/home/foundry/node-gcp-free/fleet-woodfine/woodfine-fleet-deployment/cluster-totebox-corporate-1/service-study/projects/assets"
    ["PointSav Technical Library"]="/home/foundry/node-gcp-free/factory-pointsav/content-wiki-documentation"
)

PS3="Select destination silo (number): "
select SILO_NAME in "${!SILO_MAP[@]}"; do
    if [ -n "$SILO_NAME" ]; then
        SILO_PATH="${SILO_MAP[$SILO_NAME]}"
        break
    fi
done

echo -e "\n🚀 INITIATING EXTRACTION SEQUENCE..."
echo "$(date): Ingesting $FILE to $SILO_NAME" >> "$LOG_FILE"

# 3. PUSH raw file to the Cloud Ingestion Airlock
echo "📤 Pushing payload to cloud engine..."
rsync -avz "$FILE" "$REMOTE_USER@$GCP_IP:$REMOTE_ROOT/service-content/input/"

# 4. EXECUTE Remote Extraction via SSH (RPC)
echo "⚙️  Executing remote extraction engine..."
ssh "$REMOTE_USER@$GCP_IP" "bash $REMOTE_ROOT/service-content/tools/trigger_extraction.sh \"$FILE\" \"EXTRACT\" \"$SILO_PATH\""

# 5. PULL the finished extraction report back to the Desktop
echo "📥 Retrieving machine-readable artifact..."
rsync -avz "$REMOTE_USER@$GCP_IP:$REMOTE_ROOT/service-content/outbox/" "$HOT_ZONE/output/"

# 6. ARCHIVE and LOG completion
echo "✅ SUCCESS: $FILE processed and anchored to $SILO_NAME." | tee -a "$LOG_FILE"
mv "$FILE" "$HOT_ZONE/input/processed/"
echo "---" >> "$LOG_FILE"

echo -e "\n🏁 INGESTION COMPLETE."
echo "👉 View the extracted data in: ~/Desktop/service-content/output"
