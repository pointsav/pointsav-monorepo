#!/bin/bash
set -euo pipefail

# © 2026 PointSav Digital Systems
# Institutional Brutalism: Phase 1 Autonomous IMAP Crawler (UID & Tuple Parity Edition)

INDEX_CARD="../totebox-index.env"
source "$INDEX_CARD"

ROSTER_PATH="../data-ledgers/personnel_roster.jsonl"
MASTER_LEDGER="../data-ledgers/master_metadata_ledger.jsonl"
IMAP_PARSER_PY="imap_crawler_${TARGET_MAILBOX}.py"

TARGET_BYTES=838860800 

echo "SYSTEM EVENT: Initiating Phase 1 Autonomous IMAP Crawler [$TARGET_MAILBOX]..."
echo "SYSTEM EVENT: Wiping ephemeral daily delta list..."
rm -f "$ROSTER_PATH"
touch "$ROSTER_PATH"

echo "SYSTEM EVENT: Negotiating IMAP Connection to $APPLE_IMAP_SERVER..."

cat << PY_EOF > "$IMAP_PARSER_PY"
import imaplib, email, json, sys, re

try:
    server = sys.argv[1]
    user = sys.argv[2]
    pwd = sys.argv[3]
    owner = sys.argv[4]
    target_bytes = int(sys.argv[5])
    
    mail = imaplib.IMAP4_SSL(server)
    mail.login(user, pwd)
    mail.select('INBOX', readonly=True)
    
    # ENFORCEMENT: Permanent UID Targeting
    typ, data = mail.uid('SEARCH', None, 'ALL')
    uids = data[0].split()
    
    current_bytes = 0
    
    for uid in uids:
        if current_bytes >= target_bytes:
            break
            
        typ, msg_data = mail.uid('FETCH', uid, '(RFC822.SIZE BODY.PEEK[HEADER.FIELDS (MESSAGE-ID SUBJECT DATE FROM)])')
        
        raw_headers = None
        size_val = '0'
        
        # ENFORCEMENT: Tuple Type-Checking (Bypass Apple Status Flags)
        for part in msg_data:
            if isinstance(part, tuple):
                meta_str = part[0].decode('utf-8', 'ignore') if hasattr(part[0], 'decode') else str(part[0])
                size_match = re.search(r'RFC822\.SIZE (\d+)', meta_str, re.IGNORECASE)
                if size_match:
                    size_val = size_match.group(1)
                raw_headers = part[1]
        
        if not raw_headers:
            continue
            
        try:
            msg = email.message_from_bytes(raw_headers)
        except AttributeError:
            msg = email.message_from_string(raw_headers)
            
        msg_id = msg.get('Message-ID', '')
        if msg_id:
            msg_id = msg_id.strip('<>')
        else:
            msg_id = uid.decode('utf-8', 'ignore') if hasattr(uid, 'decode') else str(uid)
            
        sender_email = msg.get('From', '').replace('\r', '').replace('\n', '')
        
        record = {
            "MessageID": msg_id,
            "Subject": msg.get('Subject', ''),
            "DateReceived": msg.get('Date', ''),
            "Sender": {"EmailAddress": sender_email, "Name": ""},
            "ToRecipients": [],
            "CcRecipients": [],
            "ArchiveOwner": owner,
            "FolderName": "INBOX",
            "HasAttachments": "unknown",
            "SizeBytes": size_val,
            "IsRead": "true",
            "IMAP_UID": uid.decode('utf-8', 'ignore') if hasattr(uid, 'decode') else str(uid)
        }
        print(json.dumps(record))
        current_bytes += int(size_val)
        
    mail.close()
    mail.logout()
except Exception as e:
    sys.stderr.write("IMAP Error: " + str(e) + "\n")
    sys.exit(1)
PY_EOF

python "$IMAP_PARSER_PY" "$APPLE_IMAP_SERVER" "$APPLE_USERNAME" "$APPLE_APP_PASSWORD" "$ARCHIVE_OWNER" "$TARGET_BYTES" > "$ROSTER_PATH"

rm -f "$IMAP_PARSER_PY"

CURRENT_MB=$(awk -F'"SizeBytes": "' '{sum += $2} END {print int(sum/1024/1024)}' "$ROSTER_PATH")
TOTAL_EXTRACTED=$(wc -l < "$ROSTER_PATH" | tr -d ' ')

echo "====================================================================="
echo "SYSTEM EVENT: Phase 1 IMAP Crawler Complete."
echo "Secured $TOTAL_EXTRACTED Assets to Delta List ($CURRENT_MB MB Data Volume)."
echo "====================================================================="
echo "SYSTEM EVENT: Aggregating Metadata to Permanent SLM Master Ledger..."
cat "$ROSTER_PATH" >> "$MASTER_LEDGER"
echo "SYSTEM EVENT: AI Data Infrastructure secured. Ready for Phase 2."
