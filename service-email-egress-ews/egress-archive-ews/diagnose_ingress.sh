#!/bin/bash

# POINT-SAV DIGITAL SYSTEMS: INGRESS DIAGNOSTIC
# ---------------------------------------------------------

# 1. Locate and Load Configuration
# We step up one directory to find the env file, bridging Vendor code to Customer data.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="$SCRIPT_DIR/../totebox-index.env"

if [ -f "$ENV_FILE" ]; then
    # Safely export variables from the .env file, ignoring comments
    export $(grep -v '^#' "$ENV_FILE" | xargs)
else
    echo "FATAL: Deployment configuration ($ENV_FILE) not found."
    echo "Please copy template.env to totebox-index.env and populate your credentials."
    exit 1
fi

echo "=========================================================="
echo " INITIATING EWS DIAGNOSTIC PROBE"
echo "=========================================================="
echo "Target Endpoint: $EWS_ENDPOINT"
echo "Authenticating as: $EWS_USERNAME"
echo "Target Mailbox: $TARGET_MAILBOX"
echo "----------------------------------------------------------"

# 2. Execute the Probe (Genericized curl request)
curl -s -u "$EWS_USERNAME:$EWS_PASSWORD" \
     -X POST \
     -H "Content-Type: text/xml" \
     -d '<?xml version="1.0" encoding="utf-8"?>
         <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
                        xmlns:t="http://schemas.microsoft.com/exchange/services/2006/types">
           <soap:Body>
             <GetFolder xmlns="http://schemas.microsoft.com/exchange/services/2006/messages">
               <FolderShape>
                 <t:BaseShape>Default</t:BaseShape>
               </FolderShape>
               <FolderIds>
                 <t:DistinguishedFolderId Id="inbox">
                    <t:Mailbox><t:EmailAddress>'"$TARGET_MAILBOX"'</t:EmailAddress></t:Mailbox>
                 </t:DistinguishedFolderId>
               </FolderIds>
             </GetFolder>
           </soap:Body>
         </soap:Envelope>' \
     "$EWS_ENDPOINT" | xmllint --format - || echo "Probe failed or xmllint not installed."

echo ""
echo "=========================================================="
echo " DIAGNOSTIC COMPLETE"
echo "=========================================================="
