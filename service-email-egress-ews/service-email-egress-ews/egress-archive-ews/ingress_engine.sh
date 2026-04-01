#!/bin/bash
set -euo pipefail

# © 2026 PointSav Digital Systems
# Institutional Brutalism: Phase 2 Resumable Physical Ingress

INDEX_CARD="../totebox-index.env"
source "$INDEX_CARD"

ROSTER_PATH="../data-ledgers/personnel_roster.jsonl"
VAULT_NEW="$PHYSICAL_USB_PATH/new"

# Dynamic Process Isolation Variables
ROSTER_PARSER_PY="ingress_roster_parser_${TARGET_MAILBOX}.py"
EXTRACTOR_PY="ingress_b64_extractor_${TARGET_MAILBOX}.py"

mkdir -p "$VAULT_NEW"

echo "SYSTEM EVENT: PointSav High-Fidelity JSON Ingress Engine [$TARGET_MAILBOX] (Resumable Architecture)"

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

# Python Kernel: Extract Message IDs (Isolated)
cat << PY_EOF > "$ROSTER_PARSER_PY"
import sys, json; [sys.stdout.write(json.loads(line).get('MessageID','') + '\n') for line in sys.stdin if line.strip()]
PY_EOF

# Python Kernel: Base64 Extraction (Isolated)
cat << PY_EOF > "$EXTRACTOR_PY"
import xml.etree.ElementTree as ET, base64, sys; tree=ET.parse('ews_raw_response.xml'); mime=tree.find('.//{http://schemas.microsoft.com/exchange/services/2006/types}MimeContent'); sys.stdout.write(base64.b64decode(mime.text)) if mime is not None and mime.text else None
PY_EOF

MESSAGE_IDS=$(python "$ROSTER_PARSER_PY" < "$ROSTER_PATH")

for MESSAGE_ID in $MESSAGE_IDS; do
    if [ -z "$MESSAGE_ID" ]; then continue; fi
    
    # Sanitize the Base64 ID for the macOS physical filesystem
    SAFE_ID=$(echo "$MESSAGE_ID" | tr '/' '_' | tr '+' '-')
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
            # macOS Python 2.7 binary bridge (Isolated)
            python "$EXTRACTOR_PY" > "$TARGET_FILE"
            
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
        echo "FATAL: Asset ${MESSAGE_ID:0:15}... failed after $NETWORK_RETRY_LIMIT attempts. Halting to preserve data parity."
        rm -f ews_get_payload.xml ews_raw_response.xml "$ROSTER_PARSER_PY" "$EXTRACTOR_PY"
        exit 1
    fi
done

rm -f ews_get_payload.xml ews_raw_response.xml "$ROSTER_PARSER_PY" "$EXTRACTOR_PY"
echo "======================================================="
echo "SYSTEM EVENT: Phase 2 Physical Ingress Complete."
echo "======================================================="
