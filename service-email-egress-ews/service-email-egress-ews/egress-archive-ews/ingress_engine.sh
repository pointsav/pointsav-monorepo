
# [SECURITY OVERRIDE 1] Physical Anchor Verification
if [ ! -f "/Volumes/BACKUP-DRIVE/.pointsav_anchor" ]; then
    echo "CRITICAL SYSTEM HALT: Physical USB Anchor (.pointsav_anchor) not found!"
    echo "Aborting to prevent Phantom Drive data bleed to internal SSD."
    exit 1
fi
#!/bin/bash
set -euo pipefail

# © 2026 PointSav Digital Systems
# Institutional Brutalism: Phase 2 Resumable Physical Ingress (Hex-Armor & Quarantine Edition)

INDEX_CARD="../totebox-index.env"
source "$INDEX_CARD"

ROSTER_PATH="../data-ledgers/personnel_roster.jsonl"
QUARANTINE_PATH="../data-ledgers/quarantine_ledger.jsonl"
VAULT_NEW="$PHYSICAL_USB_PATH/new"

mkdir -p "$VAULT_NEW"

echo "SYSTEM EVENT: PointSav High-Fidelity JSON Ingress Engine (Hex-Armor & Quarantine)"

if [ ! -f "$ROSTER_PATH" ]; then
    echo "FATAL: JSONL Ledger not found."
    exit 1
fi

echo "SYSTEM EVENT: Negotiating EWS Token..."
TOKEN=$(curl -s -X POST https://login.microsoftonline.com/$AZURE_TENANT_ID/oauth2/v2.0/token \
  -d "grant_type=client_credentials" \
  -d "client_id=$AZURE_CLIENT_ID" \
  --data-urlencode "client_secret=$AZURE_CLIENT_SECRET" \
  -d "scope=https://outlook.office365.com/.default" | grep -o '"access_token":"[^"]*' | grep -o '[^"]*$')

MESSAGE_IDS=$(python -c "import sys, json; [sys.stdout.write(json.loads(line).get('MessageID','') + '\n') for line in sys.stdin if line.strip()]" < "$ROSTER_PATH")

for MESSAGE_ID in $MESSAGE_IDS; do
    if [ -z "$MESSAGE_ID" ]; then continue; fi
    
    # [HEX-ARMOR APPLIED]: Mathematically converts Base64 to Case-Agnostic Hexadecimal
    SAFE_ID=$(echo -n "$MESSAGE_ID" | md5)
    TARGET_FILE="$VAULT_NEW/${SAFE_ID}.eml"

    # [RESUME LOGIC]: If the file exists and is greater than 0 bytes, skip the network call
    if [ -s "$TARGET_FILE" ]; then
        echo "SUCCESS: Asset ${MESSAGE_ID:0:15}... already secured. Skipping."
        continue
    fi

    cat << XML > ews_get_payload.xml
<?xml version="1.0" encoding="utf-8"?>
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
               xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types"
               xmlns:m="http://schemas.microsoft.com/exchange/services/2006/messages">
  <soap:Header>
    <t:RequestServerVersion Version="Exchange2016" />
    <t:ExchangeImpersonation>
      <t:ConnectingSID><t:PrimarySmtpAddress>$EXCHANGE_TARGET_USER</t:PrimarySmtpAddress></t:ConnectingSID>
    </t:ExchangeImpersonation>
  </soap:Header>
  <soap:Body>
    <m:GetItem>
      <m:ItemShape>
        <t:BaseShape>IdOnly</t:BaseShape>
        <t:IncludeMimeContent>true</t:IncludeMimeContent>
      </m:ItemShape>
      <m:ItemIds>
        <t:ItemId Id="$MESSAGE_ID" />
      </m:ItemIds>
    </m:GetItem>
  </soap:Body>
</soap:Envelope>
XML

    ATTEMPT=1
    SUCCESS=false

    while [ $ATTEMPT -le $NETWORK_RETRY_LIMIT ]; do
        echo "SYSTEM EVENT: Extracting Payload ${MESSAGE_ID:0:15}... (Attempt $ATTEMPT)"
        
        # Extended 120-second timeout for massive payload processing
        curl -s --max-time 120 -X POST https://outlook.office365.com/EWS/Exchange.asmx \
          -H "Authorization: Bearer $TOKEN" \
          -H "Content-Type: text/xml; charset=utf-8" \
          -d @ews_get_payload.xml > ews_raw_response.xml

        if grep -q 'ResponseClass="Success"' ews_raw_response.xml; then
            # macOS Python 2.7 binary bridge
            python -c "import xml.etree.ElementTree as ET, base64, sys; tree=ET.parse('ews_raw_response.xml'); mime=tree.find('.//{http://schemas.microsoft.com/exchange/services/2006/types}MimeContent'); sys.stdout.write(base64.b64decode(mime.text)) if mime is not None and mime.text else None" > "$TARGET_FILE"
            
            if [ -s "$TARGET_FILE" ]; then
                echo "SUCCESS: Asset physically secured to Staging Vault."
                SUCCESS=true
                break
            fi
        fi

        echo "WARNING: Network destabilized or massive payload timeout. Re-engaging in $((ATTEMPT * 2)) seconds..."
        sleep $((ATTEMPT * 2))
        ((ATTEMPT++))
    done

    if [ "$SUCCESS" = false ]; then
        echo "SYSTEM EVENT: Asset ${MESSAGE_ID:0:15}... failed $NETWORK_RETRY_LIMIT attempts. Rerouting to Quarantine Ledger."
        echo "{\"MessageID\": \"$MESSAGE_ID\", \"Reason\": \"EWS Extraction Failure\"}" >> "$QUARANTINE_PATH"
        rm -f ews_get_payload.xml ews_raw_response.xml
        continue
    fi
done

rm -f ews_get_payload.xml ews_raw_response.xml
echo "======================================================="
echo "SYSTEM EVENT: Phase 2 Physical Ingress Complete."
echo "======================================================="

# [SECURITY OVERRIDE 2] Cryptographic Size Validator
echo "SYSTEM EVENT: Sweeping for 0-byte network drops..."
find "$VAULT_NEW" -name "*.eml" -size 0 -delete 2>/dev/null || true
echo "SYSTEM EVENT: 0-Byte payload trap cleared."
