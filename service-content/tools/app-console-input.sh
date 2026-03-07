#!/usr/bin/env bash
# Identity: Ingestion Gateway (Ground Control - v1.6 Surgical)

GCP_IP="35.212.238.174" 
REMOTE_USER="mathew"
REMOTE_REPO="/home/foundry/node-gcp-free/factory-pointsav/pointsav-monorepo"
HOT_ZONE="$HOME/Desktop/service-content"
LOG_FILE="$HOT_ZONE/logs/ingestion.log"

echo "📡 POINTSAV SURGICAL GATEWAY ACTIVE"
echo "------------------------------------------------"

# 1. Select File
cd "$HOT_ZONE/input"
files=(*)
if [ "${#files[@]}" -eq 0 ] || [ "${files[0]}" == "*" ]; then
    echo "❌ ERROR: No files found in input folder."
    exit 1
fi

echo "📂 FILES READY:"
PS3="Select file number: "
select FILE in "${files[@]}"; do
    if [ -n "$FILE" ] && [ -f "$FILE" ]; then break; fi
done

# 2. Select Target Silo
echo -e "\n🏛️  SELECT TARGET DATA MESH SILO (Remote):"
declare -A SILO_MAP=(
    ["Woodfine Corporate"]="/home/foundry/node-gcp-free/fleet-woodfine/woodfine-fleet-deployment/cluster-totebox-corporate-1/service-study/corporate/assets"
    ["Woodfine Projects"]="/home/foundry/node-gcp-free/fleet-woodfine/woodfine-fleet-deployment/cluster-totebox-corporate-1/service-study/projects/assets"
    ["Technical Library"]="/home/foundry/node-gcp-free/factory-pointsav/content-wiki-documentation"
)

select SILO_NAME in "${!SILO_MAP[@]}"; do
    if [ -n "$SILO_NAME" ]; then
        SILO_PATH="${SILO_MAP[$SILO_NAME]}"
        break
    fi
done

echo -e "\n🚀 INITIATING EXTRACTION..."
echo "$(date): Ingesting $FILE to $SILO_NAME" >> "$LOG_FILE"

# 3. PUSH to Cloud
echo "📤 Pushing to Surgical Node..."
rsync -avz "$FILE" "$REMOTE_USER@$GCP_IP:$REMOTE_REPO/service-content/input/"

# 4. EXECUTE Remote Driver
echo "⚙️  Triggering Remote Extraction..."
ssh "$REMOTE_USER@$GCP_IP" "bash $REMOTE_REPO/service-content/tools/trigger_extraction.sh \"$FILE\" \"EXTRACT\" \"$SILO_PATH\""

# 5. PULL Result
echo "📥 Retrieving Artifact..."
rsync -avz "$REMOTE_USER@$GCP_IP:$REMOTE_REPO/service-content/outbox/" "$HOT_ZONE/output/"

# 6. Cleanup
mv "$FILE" "$HOT_ZONE/input/processed/"
echo "✅ SUCCESS: $FILE anchored and report retrieved."
echo "---" >> "$LOG_FILE"
