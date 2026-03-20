#!/usr/bin/env bash
set -euo pipefail

# Publish gadscli to npm.
#
# Usage:
#   ./scripts/publish-npm.sh              # publish to npm
#   ./scripts/publish-npm.sh --dry-run    # dry run

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DRY_RUN=""

if [[ "${1:-}" == "--dry-run" ]]; then
  DRY_RUN="--dry-run"
  echo "DRY RUN MODE"
fi

echo "Publishing gadscli..."
(cd "$ROOT/npm/gadscli" && npm publish --access public $DRY_RUN)

echo ""
echo "Done!"
