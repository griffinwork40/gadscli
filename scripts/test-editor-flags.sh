#!/usr/bin/env bash
set -euo pipefail

# test-editor-flags.sh
# Discovers the Google Ads Editor binary, tests each known CLI flag,
# and reports on available databases in the Editor data directory.

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

EDITOR_GLOB="/Applications/Google Ads Editor.app/Contents/Versions/*/Google Ads Editor.app/Contents/MacOS/Google Ads Editor"
EDITOR_DATA_DIR="${HOME}/Library/Application Support/Google/Google-AdWords-Editor/735"
FLAG_TIMEOUT=2  # seconds per flag probe

FLAGS=(
    "-download"
    "-noics"
    "-importCSV"
    "-importFile"
    "-post"
    "-validate"
    "-exportXml"
    "-exportHTML"
    "-importXML"
    "-acceptProposals"
)

# ---------------------------------------------------------------------------
# Helper: print a section header
# ---------------------------------------------------------------------------
header() {
    echo ""
    echo "=================================================================="
    echo "  $1"
    echo "=================================================================="
}

# ---------------------------------------------------------------------------
# Detect timeout command (macOS ships with gtimeout via coreutils; Linux has timeout)
# ---------------------------------------------------------------------------
detect_timeout_cmd() {
    if command -v gtimeout &>/dev/null; then
        echo "gtimeout"
    elif command -v timeout &>/dev/null; then
        echo "timeout"
    else
        echo ""
    fi
}

# ---------------------------------------------------------------------------
# Step 1: Locate the Editor binary (pick latest version by sort)
# ---------------------------------------------------------------------------
find_editor_binary() {
    # Expand the glob manually so we can sort and pick the latest version.
    local candidates
    # Use bash globbing; disable errexit temporarily in case glob matches nothing.
    candidates=( /Applications/Google\ Ads\ Editor.app/Contents/Versions/*/Google\ Ads\ Editor.app/Contents/MacOS/Google\ Ads\ Editor ) || true

    if [[ ${#candidates[@]} -eq 0 || ! -f "${candidates[0]}" ]]; then
        echo ""
        return
    fi

    # Sort descending so the highest version number (latest) is first.
    local sorted
    sorted=$(printf '%s\n' "${candidates[@]}" | sort -t/ -k7 -V -r)
    local best
    best=$(echo "$sorted" | head -n1)
    echo "$best"
}

# ---------------------------------------------------------------------------
# Step 2: Extract version string from the binary path
# ---------------------------------------------------------------------------
extract_version() {
    local binary_path="$1"
    # Path structure: .../Versions/<version>/Google Ads Editor.app/...
    echo "$binary_path" | grep -oE 'Versions/[^/]+' | head -n1 | cut -d/ -f2
}

# ---------------------------------------------------------------------------
# Step 3: Test a single flag
# ---------------------------------------------------------------------------
test_flag() {
    local binary="$1"
    local flag="$2"
    local timeout_cmd="$3"

    local exit_code=0
    local stderr_out=""

    if [[ -z "$timeout_cmd" ]]; then
        # No timeout available — run with a subshell kill trick as fallback.
        stderr_out=$(QT_QPA_PLATFORM=offscreen "$binary" "$flag" 2>&1 >/dev/null) || exit_code=$?
    else
        stderr_out=$(QT_QPA_PLATFORM=offscreen "$timeout_cmd" "$FLAG_TIMEOUT" "$binary" "$flag" 2>&1 >/dev/null) || exit_code=$?
    fi

    # timeout exits 124 when it kills a process; treat that as "accepted" since
    # the binary didn't immediately reject the flag with a non-zero exit.
    local status="REJECTED (exit ${exit_code})"
    if [[ $exit_code -eq 0 || $exit_code -eq 124 ]]; then
        status="ACCEPTED"
    fi

    printf "  %-20s  %s\n" "$flag" "$status"
    if [[ -n "$stderr_out" && $exit_code -ne 0 && $exit_code -ne 124 ]]; then
        echo "                         stderr: $(echo "$stderr_out" | head -n1)"
    fi
}

# ---------------------------------------------------------------------------
# Step 4: Check the Editor data directory for databases
# ---------------------------------------------------------------------------
check_databases() {
    local data_dir="$1"

    header "Editor Database Files"

    if [[ ! -d "$data_dir" ]]; then
        echo "  Data directory not found: $data_dir"
        return
    fi

    echo "  Data directory: $data_dir"
    echo ""

    # Check for ape.db
    if [[ -f "$data_dir/ape.db" ]]; then
        local size
        size=$(du -sh "$data_dir/ape.db" 2>/dev/null | cut -f1)
        echo "  ape.db           EXISTS  (${size})"
    else
        echo "  ape.db           NOT FOUND"
    fi

    echo ""

    # List ape_*.db files
    local ape_files
    ape_files=( "$data_dir"/ape_*.db ) || true

    if [[ ${#ape_files[@]} -eq 0 || ! -f "${ape_files[0]}" ]]; then
        echo "  ape_*.db files:  NONE FOUND"
    else
        echo "  ape_*.db files found:"
        for f in "${ape_files[@]}"; do
            if [[ -f "$f" ]]; then
                local fname size
                fname=$(basename "$f")
                size=$(du -sh "$f" 2>/dev/null | cut -f1)
                echo "    - ${fname}  (${size})"
            fi
        done
    fi
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
main() {
    header "Google Ads Editor Flag Discovery"

    # --- Locate binary ---
    local binary
    binary=$(find_editor_binary)

    if [[ -z "$binary" || ! -f "$binary" ]]; then
        echo ""
        echo "  ERROR: Google Ads Editor binary not found."
        echo "  Searched: ${EDITOR_GLOB}"
        echo ""
        echo "  Make sure Google Ads Editor is installed in /Applications."
        exit 1
    fi

    local version
    version=$(extract_version "$binary")

    echo ""
    echo "  Binary:   $binary"
    echo "  Version:  ${version:-unknown}"

    # --- Detect timeout command ---
    local timeout_cmd
    timeout_cmd=$(detect_timeout_cmd)

    if [[ -z "$timeout_cmd" ]]; then
        echo ""
        echo "  WARNING: Neither 'timeout' nor 'gtimeout' found."
        echo "  Install coreutils via Homebrew for accurate flag testing:"
        echo "    brew install coreutils"
        echo "  Proceeding without timeout — flags may hang."
    else
        echo "  Timeout: ${timeout_cmd} ${FLAG_TIMEOUT}s per flag"
    fi

    # --- Test flags ---
    header "Flag Acceptance Test"
    echo ""
    printf "  %-20s  %s\n" "FLAG" "RESULT"
    printf "  %-20s  %s\n" "----" "------"

    for flag in "${FLAGS[@]}"; do
        test_flag "$binary" "$flag" "$timeout_cmd"
    done

    # --- Database check ---
    check_databases "$EDITOR_DATA_DIR"

    header "Done"
    echo ""
}

main "$@"
