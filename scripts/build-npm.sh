#!/usr/bin/env bash
set -euo pipefail

# Build gadscli binaries for npm distribution.
# Requires: cargo, cross (cargo install cross) for non-native targets
#
# Usage:
#   ./scripts/build-npm.sh          # build all targets
#   ./scripts/build-npm.sh native   # build only for current platform (fast, for testing)

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN_DIR="$ROOT/npm/gadscli/bin"
VERSION=$(grep '^version' "$ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)"/\1/')

echo "Building gadscli v${VERSION}"
mkdir -p "$BIN_DIR"

# Each entry: "rust_target:binary_name"
TARGETS=(
  "aarch64-apple-darwin:gadscli-darwin-arm64"
  "x86_64-apple-darwin:gadscli-darwin-x64"
  "x86_64-unknown-linux-gnu:gadscli-linux-x64-gnu"
  "aarch64-unknown-linux-gnu:gadscli-linux-arm64-gnu"
  "x86_64-pc-windows-msvc:gadscli-win32-x64-msvc"
)

build_target() {
  local target=$1
  local bin_name=$2
  local src_ext=""
  local dst_ext=""

  if [[ "$target" == *"windows"* ]]; then
    src_ext=".exe"
    dst_ext=".exe"
  fi

  echo "  Building for $target..."

  local host
  host=$(rustc -vV | grep host | awk '{print $2}')

  if [[ "$target" == "$host" ]]; then
    cargo build --release --target "$target"
  else
    cross build --release --target "$target"
  fi

  cp "$ROOT/target/$target/release/gadscli${src_ext}" "$BIN_DIR/${bin_name}${dst_ext}"
  chmod +x "$BIN_DIR/${bin_name}${dst_ext}"
  echo "  -> $BIN_DIR/${bin_name}${dst_ext}"
}

if [[ "${1:-}" == "native" ]]; then
  host=$(rustc -vV | grep host | awk '{print $2}')
  bin_name=""
  for entry in "${TARGETS[@]}"; do
    t="${entry%%:*}"
    b="${entry##*:}"
    if [[ "$t" == "$host" ]]; then
      bin_name="$b"
      break
    fi
  done
  if [[ -z "$bin_name" ]]; then
    echo "Error: native target $host not in TARGETS list"
    exit 1
  fi
  build_target "$host" "$bin_name"
else
  for entry in "${TARGETS[@]}"; do
    target="${entry%%:*}"
    bin_name="${entry##*:}"
    build_target "$target" "$bin_name"
  done
fi

# Sync version
echo ""
echo "Syncing version to $VERSION..."
sed -i '' "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$ROOT/npm/gadscli/package.json"

echo ""
echo "Done! Binaries are in npm/gadscli/bin/"
