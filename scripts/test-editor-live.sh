#!/usr/bin/env bash
set -euo pipefail

# Live end-to-end test for Editor headless CLI
# WARNING: This script interacts with a real Google Ads account.
# Only run on a developer machine with a test account.
#
# Prerequisites:
#   - Google Ads Editor installed and logged in via GUI at least once
#   - GADS_CUSTOMER_ID set to a test account
#   - gadscli binary built (cargo build --release)

GADSCLI="${GADSCLI:-./target/release/gadscli}"
CUSTOMER_ID="${GADS_CUSTOMER_ID:?Set GADS_CUSTOMER_ID to your test account ID}"
USER_EMAIL="${GADS_EDITOR_EMAIL:?Set GADS_EDITOR_EMAIL to your Google account email}"

echo "========================================"
echo "  Editor Live Test"
echo "  Customer ID: $CUSTOMER_ID"
echo "  User Email:  $USER_EMAIL"
echo "========================================"
echo ""

# Step 1: Status check
echo "--- Step 1: Status Check ---"
$GADSCLI editor status --customer-id "$CUSTOMER_ID" || true
echo ""

# Step 2: Download fresh data
echo "--- Step 2: Download ---"
read -rp "Download fresh account data? (y/N) " confirm
if [[ "$confirm" =~ ^[Yy]$ ]]; then
    $GADSCLI editor download --customer-id "$CUSTOMER_ID" --user-email "$USER_EMAIL"
    echo "Download complete."
else
    echo "Skipped download."
fi
echo ""

# Step 3: Read data
echo "--- Step 3: Read Local Data ---"
echo "Campaigns:"
$GADSCLI editor campaigns --customer-id "$CUSTOMER_ID" || true
echo ""

echo "Ad Groups:"
$GADSCLI editor ad-groups --customer-id "$CUSTOMER_ID" || true
echo ""

echo "Keywords:"
$GADSCLI editor keywords --customer-id "$CUSTOMER_ID" || true
echo ""

echo "Budgets:"
$GADSCLI editor budgets --customer-id "$CUSTOMER_ID" || true
echo ""

echo "Labels:"
$GADSCLI editor labels --customer-id "$CUSTOMER_ID" || true
echo ""

echo "Pending Changes:"
$GADSCLI editor pending --customer-id "$CUSTOMER_ID" || true
echo ""

# Step 4: Add a test keyword via CSV import
echo "--- Step 4: Test CSV Import ---"
read -rp "Add a test keyword? (y/N) " confirm
if [[ "$confirm" =~ ^[Yy]$ ]]; then
    read -rp "Campaign name: " campaign_name
    read -rp "Ad Group name: " ag_name
    read -rp "Keyword text: " kw_text

    $GADSCLI editor add-keywords \
        --customer-id "$CUSTOMER_ID" \
        --campaign "$campaign_name" \
        --ad-group "$ag_name" \
        --keywords "$kw_text" \
        --match-type Broad

    echo "Keyword added. Checking pending:"
    $GADSCLI editor pending --customer-id "$CUSTOMER_ID" || true
else
    echo "Skipped keyword addition."
fi
echo ""

# Step 5: Validate
echo "--- Step 5: Validate ---"
read -rp "Run validation? (y/N) " confirm
if [[ "$confirm" =~ ^[Yy]$ ]]; then
    $GADSCLI editor validate --customer-id "$CUSTOMER_ID" || true
else
    echo "Skipped validation."
fi
echo ""

# Step 6: Post
echo "--- Step 6: Post Changes ---"
read -rp "Post pending changes to Google? (y/N) " confirm
if [[ "$confirm" =~ ^[Yy]$ ]]; then
    $GADSCLI editor post --customer-id "$CUSTOMER_ID" --user-email "$USER_EMAIL"
    echo "Post complete."
else
    echo "Skipped posting."
fi
echo ""

echo "========================================"
echo "  Live test complete."
echo "========================================"
