#!/bin/bash
set -euo pipefail

# © 2026 PointSav Digital Systems
# Institutional Brutalism: Phase 2 IMAP Physical Ingress

INDEX_CARD="../totebox-index.env"
source "$INDEX_CARD"

ROSTER_PATH="../data-ledgers/personnel_roster.jsonl"
VAULT_NEW="$PHYSICAL_USB_PATH/new"
IMAP_INGRESS_PY="imap_ingress_${TARGET_MAILBOX}.py"

mkdir -p "$VAULT_NEW"

echo "SYSTEM EVENT: PointSav High-Fidelity IMAP Ingress Engine [$TARGET_MAILBOX]"

if [ ! -f "$ROSTER_PATH" ]; then
    echo "FATAL: JSONL Ledger not found."
    exit 1
fi

cat << PY_EOF > "$IMAP_INGRESS_PY"
import imaplib, sys, json, os
try:
    server = sys.argv[1]
    user = sys.argv[2]
    pwd = sys.argv[3]
    roster_path = sys.argv[4]
    vault_path = sys.argv[5]
    
    mail = imaplib.IMAP4_SSL(server)
    mail.login(user, pwd)
    mail.select('INBOX', readonly=True)
    
    with open(roster_path, 'r') as f:
        for line in f:
            if not line.strip(): continue
            record = json.loads(line)
            uid = record.get("IMAP_UID")
            msg_id = record.get("MessageID")
            
            safe_id = msg_id.replace('/', '_').replace('+', '-')
            target_file = os.path.join(vault_path, safe_id + ".eml")
            
            if os.path.exists(target_file) and os.path.getsize(target_file) > 0:
                print("SUCCESS: Asset {}... already secured.".format(msg_id[:15]))
                continue
                
            print("SYSTEM EVENT: Extracting Payload {}...".format(msg_id[:15]))
            typ, data = mail.fetch(uid, '(RFC822)')
            
            with open(target_file, 'wb') as out_f:
                out_f.write(data[0][1])
                
    mail.close()
    mail.logout()
except Exception as e:
    sys.stderr.write("IMAP Ingress Error: " + str(e) + "\n")
    sys.exit(1)
PY_EOF

python "$IMAP_INGRESS_PY" "$APPLE_IMAP_SERVER" "$APPLE_USERNAME" "$APPLE_APP_PASSWORD" "$ROSTER_PATH" "$VAULT_NEW"

rm -f "$IMAP_INGRESS_PY"
echo "======================================================="
echo "SYSTEM EVENT: Phase 2 IMAP Physical Ingress Complete."
echo "======================================================="
