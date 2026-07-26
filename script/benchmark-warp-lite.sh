#!/usr/bin/env bash
# Print a revision-aware static baseline for Warp Lite.
#
# This script never builds or launches Warp Lite. Missing artifacts are reported
# as "missing" so callers can decide whether a release build is required.
#
# Usage:
#   script/benchmark-warp-lite.sh
#   script/benchmark-warp-lite.sh --output target/warp-lite-benchmarks/baseline.txt

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BINARY_PATH="$ROOT/target/release/warp-oss"
APP_PATH="$ROOT/WarpLite.app"
OUTPUT_PATH=""
FORCE_OUTPUT=false

usage() {
    echo "Usage: $0 [--binary PATH] [--app PATH] [--output PATH] [--force-output]" >&2
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --binary)
            [[ $# -ge 2 ]] || { usage; exit 2; }
            BINARY_PATH="$2"
            shift 2
            ;;
        --app)
            [[ $# -ge 2 ]] || { usage; exit 2; }
            APP_PATH="$2"
            shift 2
            ;;
        --output)
            [[ $# -ge 2 ]] || { usage; exit 2; }
            OUTPUT_PATH="$2"
            shift 2
            ;;
        --force-output)
            FORCE_OUTPUT=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage
            exit 2
            ;;
    esac
done

file_size_bytes() {
    local path="$1"
    if [[ ! -f "$path" ]]; then
        echo "missing"
    elif stat -f '%z' "$path" >/dev/null 2>&1; then
        stat -f '%z' "$path"
    else
        stat -c '%s' "$path"
    fi
}

directory_size_bytes() {
    local path="$1"
    if [[ ! -d "$path" ]]; then
        echo "missing"
    elif stat -f '%z' "$path" >/dev/null 2>&1; then
        find "$path" -type f -exec stat -f '%z' {} + | awk '{sum += $1} END {print sum + 0}'
    else
        find "$path" -type f -exec stat -c '%s' {} + | awk '{sum += $1} END {print sum + 0}'
    fi
}

rust_loc() {
    local path="$1"
    if [[ ! -d "$path" ]]; then
        echo "0"
        return
    fi

    find "$path" -type f -name '*.rs' -print0 \
        | xargs -0 wc -l 2>/dev/null \
        | awk 'END {print $1 + 0}'
}

direct_dependency_state() {
    local dependency="$1"
    if grep -Eq "^${dependency}(\\.workspace)?[[:space:]]*=" "$ROOT/app/Cargo.toml"; then
        echo "present"
    else
        echo "absent"
    fi
}

render_report() {
    local dirty_state="clean"
    [[ -n "$(git -C "$ROOT" status --porcelain)" ]] && dirty_state="dirty"

    echo "schema_version=1"
    echo "git_revision=$(git -C "$ROOT" rev-parse HEAD)"
    echo "git_dirty=$dirty_state"
    echo "rustc_version=$(rustc --version)"
    echo "cargo_version=$(cargo --version)"
    echo "host_arch=$(uname -m)"
    echo "host_os=$(uname -s)"
    echo "release_binary_path=$BINARY_PATH"
    echo "release_binary_bytes=$(file_size_bytes "$BINARY_PATH")"
    echo "app_bundle_path=$APP_PATH"
    echo "app_bundle_bytes=$(directory_size_bytes "$APP_PATH")"
    echo "workspace_package_manifests=$(( $(find "$ROOT/crates" -mindepth 2 -maxdepth 2 -name Cargo.toml | wc -l | tr -d ' ') + 1 ))"
    echo "loc_app_ai=$(rust_loc "$ROOT/app/src/ai")"
    echo "loc_app_ai_assistant=$(rust_loc "$ROOT/app/src/ai_assistant")"
    echo "loc_app_auth=$(rust_loc "$ROOT/app/src/auth")"
    echo "loc_app_billing=$(rust_loc "$ROOT/app/src/billing")"
    echo "loc_app_cloud_object=$(rust_loc "$ROOT/app/src/cloud_object")"
    echo "loc_app_code_review=$(rust_loc "$ROOT/app/src/code_review")"
    echo "loc_app_drive=$(rust_loc "$ROOT/app/src/drive")"
    echo "loc_app_pricing=$(rust_loc "$ROOT/app/src/pricing")"
    echo "loc_app_server=$(rust_loc "$ROOT/app/src/server")"
    echo "direct_dep_ai=$(direct_dependency_state ai)"
    echo "direct_dep_computer_use=$(direct_dependency_state computer_use)"
    echo "direct_dep_onboarding=$(direct_dependency_state onboarding)"
    echo "direct_dep_warp_graphql=$(direct_dependency_state warp_graphql)"
    echo "direct_dep_warp_server_client=$(direct_dependency_state warp_server_client)"
    echo "direct_dep_websocket=$(direct_dependency_state websocket)"
}

cd "$ROOT"

if [[ -n "$OUTPUT_PATH" ]]; then
    if [[ -e "$OUTPUT_PATH" && "$FORCE_OUTPUT" != true ]]; then
        echo "Error: output already exists: $OUTPUT_PATH (pass --force-output to replace it)" >&2
        exit 2
    fi
    mkdir -p "$(dirname "$OUTPUT_PATH")"
    render_report | tee "$OUTPUT_PATH"
else
    render_report
fi
