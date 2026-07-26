#!/usr/bin/env bash
# Measure native Warp Lite launch proxies and idle resource use on macOS.
#
# The harness launches the bundle executable directly so it owns an exact PID.
# It never uses killall/pkill and never terminates a process it did not start.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEFAULT_APP_PATH="/Applications/WarpLite.app"
APP_PATH=""
OUTPUT_PATH=""
FORCE_OUTPUT=false
FORCE_RAW_SAMPLES=false
RAW_SAMPLES_PATH="$ROOT/target/warp-lite-benchmarks/runtime-samples.tsv"
SETTLE_SECONDS=30
SAMPLE_SECONDS=10
SAMPLE_INTERVAL_SECONDS=1
READINESS_TIMEOUT_SECONDS=15
DRY_RUN=false
OWNED_PID=""
OWNED_PROCESS_EXECUTABLE=""

usage() {
    cat >&2 <<EOF
Usage: $0 [options]

Options:
  --app PATH                 WarpLite.app to launch
  --output PATH              Persist the key=value report
  --force-output             Replace an existing --output file
  --raw-samples PATH         Persist idle samples (default: target/warp-lite-benchmarks/runtime-samples.tsv)
  --force-raw-samples        Replace an existing raw-samples file
  --settle-seconds N         Idle settling duration (default: 30)
  --sample-seconds N         Idle sampling duration (default: 10)
  --sample-interval N        Seconds between samples (default: 1)
  --readiness-timeout N      Window readiness timeout (default: 15)
  --dry-run                  Validate inputs without launching the app
  -h, --help                 Show this help

If --app is omitted, /Applications/WarpLite.app is used only when it exists.
EOF
}

fail() {
    echo "error: $*" >&2
    exit 2
}

require_nonnegative_integer() {
    local name="$1"
    local value="$2"
    [[ "$value" =~ ^[0-9]+$ ]] || fail "$name must be a non-negative integer"
}

require_positive_integer() {
    local name="$1"
    local value="$2"
    [[ "$value" =~ ^[1-9][0-9]*$ ]] || fail "$name must be a positive integer"
}

cleanup_owned_process() {
    local current_executable=""
    if [[ -n "$OWNED_PID" ]]; then
        current_executable="$(ps -p "$OWNED_PID" -o comm= 2>/dev/null | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' || true)"
    fi

    if [[ -n "$OWNED_PID" ]] \
        && [[ "$current_executable" == "$OWNED_PROCESS_EXECUTABLE" ]] \
        && kill -0 "$OWNED_PID" 2>/dev/null; then
        kill -TERM "$OWNED_PID" 2>/dev/null || true
        local attempt
        for attempt in {1..20}; do
            kill -0 "$OWNED_PID" 2>/dev/null || break
            sleep 0.1
        done
        if kill -0 "$OWNED_PID" 2>/dev/null; then
            kill -KILL "$OWNED_PID" 2>/dev/null || true
        fi
        wait "$OWNED_PID" 2>/dev/null || true
    fi
    OWNED_PID=""
    OWNED_PROCESS_EXECUTABLE=""
}
trap cleanup_owned_process EXIT INT TERM

while [[ $# -gt 0 ]]; do
    case "$1" in
        --app)
            [[ $# -ge 2 ]] || fail "--app requires a path"
            APP_PATH="$2"
            shift 2
            ;;
        --output)
            [[ $# -ge 2 ]] || fail "--output requires a path"
            OUTPUT_PATH="$2"
            shift 2
            ;;
        --force-output)
            FORCE_OUTPUT=true
            shift
            ;;
        --raw-samples)
            [[ $# -ge 2 ]] || fail "--raw-samples requires a path"
            RAW_SAMPLES_PATH="$2"
            shift 2
            ;;
        --force-raw-samples)
            FORCE_RAW_SAMPLES=true
            shift
            ;;
        --settle-seconds)
            [[ $# -ge 2 ]] || fail "--settle-seconds requires a value"
            SETTLE_SECONDS="$2"
            shift 2
            ;;
        --sample-seconds)
            [[ $# -ge 2 ]] || fail "--sample-seconds requires a value"
            SAMPLE_SECONDS="$2"
            shift 2
            ;;
        --sample-interval)
            [[ $# -ge 2 ]] || fail "--sample-interval requires a value"
            SAMPLE_INTERVAL_SECONDS="$2"
            shift 2
            ;;
        --readiness-timeout)
            [[ $# -ge 2 ]] || fail "--readiness-timeout requires a value"
            READINESS_TIMEOUT_SECONDS="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            fail "unknown argument: $1"
            ;;
    esac
done

require_nonnegative_integer "--settle-seconds" "$SETTLE_SECONDS"
require_positive_integer "--sample-seconds" "$SAMPLE_SECONDS"
require_positive_integer "--sample-interval" "$SAMPLE_INTERVAL_SECONDS"
require_positive_integer "--readiness-timeout" "$READINESS_TIMEOUT_SECONDS"

if [[ -z "$APP_PATH" ]]; then
    [[ -d "$DEFAULT_APP_PATH" ]] \
        || fail "--app is required because $DEFAULT_APP_PATH is not present"
    APP_PATH="$DEFAULT_APP_PATH"
fi

[[ -d "$APP_PATH" ]] || fail "app bundle not found: $APP_PATH"
[[ "$APP_PATH" == *.app ]] || fail "app path must end in .app: $APP_PATH"

INFO_PLIST="$APP_PATH/Contents/Info.plist"
[[ -f "$INFO_PLIST" ]] || fail "Info.plist not found: $INFO_PLIST"

if [[ -x /usr/libexec/PlistBuddy ]]; then
    EXECUTABLE_NAME="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$INFO_PLIST" 2>/dev/null || true)"
else
    EXECUTABLE_NAME=""
fi
[[ -n "$EXECUTABLE_NAME" ]] || fail "CFBundleExecutable is missing from $INFO_PLIST"

EXECUTABLE_PATH="$APP_PATH/Contents/MacOS/$EXECUTABLE_NAME"
[[ -x "$EXECUTABLE_PATH" ]] || fail "bundle executable is not executable: $EXECUTABLE_PATH"
EXECUTABLE_PATH="$(cd "$(dirname "$EXECUTABLE_PATH")" && pwd -P)/$(basename "$EXECUTABLE_PATH")"

existing_process_pids() {
    ps -axo pid=,comm= \
        | awk -v executable="$EXECUTABLE_PATH" '
            {
                pid = $1
                $1 = ""
                sub(/^[[:space:]]+/, "")
                if ($0 == executable) {
                    print pid
                }
            }
        '
}

EXISTING_PROCESS_PIDS="$(existing_process_pids | paste -sd, -)"
EXISTING_PROCESS_COUNT=0
if [[ -n "$EXISTING_PROCESS_PIDS" ]]; then
    EXISTING_PROCESS_COUNT="$(awk -F, '{print NF}' <<< "$EXISTING_PROCESS_PIDS")"
fi

assert_no_existing_processes() {
    local pids
    pids="$(existing_process_pids | paste -sd, -)"
    if [[ -n "$pids" ]]; then
        fail "refusing to launch because the selected app is already running (pid(s): $pids)"
    fi
}

HOST_OS="$(uname -s)"
if [[ "$HOST_OS" != "Darwin" && "$DRY_RUN" != true ]]; then
    fail "runtime measurement is supported only on macOS; use --dry-run to validate inputs"
fi

validate_output_paths() {
    if [[ -n "$OUTPUT_PATH" && "$OUTPUT_PATH" == "$RAW_SAMPLES_PATH" ]]; then
        fail "--output and --raw-samples must use different paths"
    fi
    if [[ -n "$OUTPUT_PATH" && -e "$OUTPUT_PATH" && "$FORCE_OUTPUT" != true ]]; then
        fail "output already exists: $OUTPUT_PATH (pass --force-output to replace it)"
    fi
    if [[ "$DRY_RUN" != true ]] \
        && [[ -e "$RAW_SAMPLES_PATH" ]] \
        && [[ "$FORCE_RAW_SAMPLES" != true ]]; then
        fail "raw samples already exist: $RAW_SAMPLES_PATH (pass --force-raw-samples to replace them)"
    fi
}

# Validate every caller-controlled write target before any process operation.
validate_output_paths

now_milliseconds() {
    perl -MTime::HiRes=time -e 'printf "%.0f\n", time * 1000'
}

machine_model() {
    sysctl -n hw.model 2>/dev/null || echo "unknown"
}

macos_version() {
    sw_vers -productVersion 2>/dev/null || echo "unknown"
}

git_revision() {
    git -C "$ROOT" rev-parse HEAD 2>/dev/null || echo "unknown"
}

git_dirty() {
    if ! git -C "$ROOT" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
        echo "unknown"
    elif [[ -n "$(git -C "$ROOT" status --porcelain)" ]]; then
        echo "dirty"
    else
        echo "clean"
    fi
}

wait_for_window() {
    local pid="$1"
    local started_ms="$2"
    local timeout_ms=$((READINESS_TIMEOUT_SECONDS * 1000))
    local now_ms

    while kill -0 "$pid" 2>/dev/null; do
        now_ms="$(now_milliseconds)"
        if (( now_ms - started_ms >= timeout_ms )); then
            echo "timeout"
            return
        fi

        if osascript -e "tell application \"System Events\" to tell (first process whose unix id is $pid) to return (count of windows) > 0" 2>/dev/null \
            | grep -qx "true"; then
            echo "$((now_ms - started_ms))"
            return
        fi
        sleep 0.1
    done

    echo "process_exited"
}

launch_once() {
    local label="$1"
    local started_ms observed_ms window_result

    assert_no_existing_processes
    started_ms="$(now_milliseconds)"
    "$EXECUTABLE_PATH" >/dev/null 2>&1 &
    OWNED_PID=$!
    OWNED_PROCESS_EXECUTABLE="$EXECUTABLE_PATH"
    observed_ms="$(now_milliseconds)"

    printf -v "${label}_PROCESS_SPAWN_MS" '%s' "$((observed_ms - started_ms))"
    if ! kill -0 "$OWNED_PID" 2>/dev/null; then
        printf -v "${label}_WINDOW_READY_STATUS" '%s' "process_exited"
        printf -v "${label}_WINDOW_READY_MS" '%s' "unavailable"
        return 1
    fi

    window_result="$(wait_for_window "$OWNED_PID" "$started_ms")"
    case "$window_result" in
        timeout|process_exited)
            printf -v "${label}_WINDOW_READY_STATUS" '%s' "$window_result"
            printf -v "${label}_WINDOW_READY_MS" '%s' "unavailable"
            ;;
        *)
            printf -v "${label}_WINDOW_READY_STATUS" '%s' "observed"
            printf -v "${label}_WINDOW_READY_MS" '%s' "$window_result"
            ;;
    esac
}

average_column() {
    local column="$1"
    awk -F '\t' -v column="$column" 'NR > 1 {sum += $column; count++} END {if (count) printf "%.2f", sum / count; else print "unavailable"}' "$RAW_SAMPLES_PATH"
}

sample_idle_process() {
    mkdir -p "$(dirname "$RAW_SAMPLES_PATH")"
    printf 'elapsed_seconds\trss_kib\tcpu_percent\n' > "$RAW_SAMPLES_PATH"

    local elapsed=0 values rss cpu
    while (( elapsed < SAMPLE_SECONDS )); do
        kill -0 "$OWNED_PID" 2>/dev/null || return 1
        values="$(ps -p "$OWNED_PID" -o rss= -o %cpu= | awk '{$1=$1; print}')"
        [[ -n "$values" ]] || return 1
        rss="${values%% *}"
        cpu="${values##* }"
        printf '%s\t%s\t%s\n' "$elapsed" "$rss" "$cpu" >> "$RAW_SAMPLES_PATH"
        sleep "$SAMPLE_INTERVAL_SECONDS"
        elapsed=$((elapsed + SAMPLE_INTERVAL_SECONDS))
    done
}

COLD_PROCESS_SPAWN_MS="not_run"
COLD_WINDOW_READY_STATUS="not_run"
COLD_WINDOW_READY_MS="not_run"
WARM_PROCESS_SPAWN_MS="not_run"
WARM_WINDOW_READY_STATUS="not_run"
WARM_WINDOW_READY_MS="not_run"
IDLE_RSS_KIB_AVG="not_run"
IDLE_CPU_PERCENT_AVG="not_run"
SAMPLE_STATUS="not_run"

if [[ "$DRY_RUN" != true ]]; then
    launch_once COLD
    cleanup_owned_process

    launch_once WARM
    sleep "$SETTLE_SECONDS"
    if sample_idle_process; then
        SAMPLE_STATUS="observed"
        IDLE_RSS_KIB_AVG="$(average_column 2)"
        IDLE_CPU_PERCENT_AVG="$(average_column 3)"
    else
        SAMPLE_STATUS="process_exited"
        IDLE_RSS_KIB_AVG="unavailable"
        IDLE_CPU_PERCENT_AVG="unavailable"
    fi
    cleanup_owned_process
fi

render_report() {
    echo "schema_version=1"
    echo "benchmark_kind=native_runtime"
    echo "dry_run=$DRY_RUN"
    echo "git_revision=$(git_revision)"
    echo "git_dirty=$(git_dirty)"
    echo "machine_model=$(machine_model)"
    echo "host_arch=$(uname -m)"
    echo "host_os=$HOST_OS"
    echo "macos_version=$(macos_version)"
    echo "app_bundle_path=$APP_PATH"
    echo "app_executable_path=$EXECUTABLE_PATH"
    echo "existing_process_count=$EXISTING_PROCESS_COUNT"
    echo "existing_process_pids=${EXISTING_PROCESS_PIDS:-none}"
    echo "settle_seconds=$SETTLE_SECONDS"
    echo "sample_seconds=$SAMPLE_SECONDS"
    echo "sample_interval_seconds=$SAMPLE_INTERVAL_SECONDS"
    echo "readiness_timeout_seconds=$READINESS_TIMEOUT_SECONDS"
    echo "cold_process_spawn_ms=$COLD_PROCESS_SPAWN_MS"
    echo "cold_window_ready_status=$COLD_WINDOW_READY_STATUS"
    echo "cold_window_ready_ms=$COLD_WINDOW_READY_MS"
    echo "warm_process_spawn_ms=$WARM_PROCESS_SPAWN_MS"
    echo "warm_window_ready_status=$WARM_WINDOW_READY_STATUS"
    echo "warm_window_ready_ms=$WARM_WINDOW_READY_MS"
    echo "idle_sample_status=$SAMPLE_STATUS"
    echo "idle_rss_kib_avg=$IDLE_RSS_KIB_AVG"
    echo "idle_cpu_percent_avg=$IDLE_CPU_PERCENT_AVG"
    echo "raw_samples_path=$RAW_SAMPLES_PATH"
}

if [[ -n "$OUTPUT_PATH" ]]; then
    mkdir -p "$(dirname "$OUTPUT_PATH")"
    render_report | tee "$OUTPUT_PATH"
else
    render_report
fi
