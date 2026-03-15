#!/usr/bin/env bash
# Render videos for all renderable examples in parallel.
# Outputs video.mp4 into each example directory.
# Usage: ./batch-render.sh [--workers N] [--from EXAMPLE_PREFIX] [--force]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EXAMPLES_DIR="$(realpath "$SCRIPT_DIR/../examples")"

WORKERS=8
FROM=""
FORCE=0
LOG_DIR="$SCRIPT_DIR/logs"
mkdir -p "$LOG_DIR"

while [[ $# -gt 0 ]]; do
  case $1 in
    --workers) WORKERS="$2"; shift 2 ;;
    --from)    FROM="$2";    shift 2 ;;
    --force)   FORCE=1;      shift ;;
    *) echo "Unknown arg: $1"; exit 1 ;;
  esac
done

# Collect renderable examples (have both example.rs and README.md)
mapfile -t ALL < <(
  for d in "$EXAMPLES_DIR"/*/; do
    name="$(basename "$d")"
    [ -f "$d/example.rs" ] || continue
    [ -f "$d/README.md"  ] || continue
    echo "$name"
  done | sort
)

# Filter: skip already rendered (unless --force), apply --from offset
QUEUE=()
started=0
for name in "${ALL[@]}"; do
  if [[ -n "$FROM" && "$started" -eq 0 ]]; then
    [[ "$name" == "$FROM"* ]] && started=1 || continue
  fi
  if [[ "$FORCE" -eq 0 && -f "$EXAMPLES_DIR/$name/video.mp4" ]]; then
    continue  # already done
  fi
  QUEUE+=("$name")
done

TOTAL="${#QUEUE[@]}"
echo "Examples dir:  $EXAMPLES_DIR"
echo "Workers:       $WORKERS"
echo "Queue:         $TOTAL examples to render"
echo "Logs:          $LOG_DIR"
echo ""

if [[ "$TOTAL" -eq 0 ]]; then
  echo "Nothing to render. Use --force to re-render existing videos."
  exit 0
fi

# Job pool
running=0
done_count=0
failed=()
pids=()
names=()

finish_one() {
  local idx=$1
  local pid="${pids[$idx]}"
  local name="${names[$idx]}"
  wait "$pid" && status=0 || status=$?
  running=$((running - 1))
  done_count=$((done_count + 1))
  if [[ $status -eq 0 ]]; then
    echo "  ✅ [$done_count/$TOTAL] $name"
  else
    echo "  ❌ [$done_count/$TOTAL] $name (exit $status — see logs/$name.log)"
    failed+=("$name")
  fi
  pids[$idx]=""
  names[$idx]=""
}

for name in "${QUEUE[@]}"; do
  # Wait for a free slot
  while [[ $running -ge $WORKERS ]]; do
    for i in "${!pids[@]}"; do
      [[ -z "${pids[$i]:-}" ]] && continue
      if ! kill -0 "${pids[$i]}" 2>/dev/null; then
        finish_one "$i"
      fi
    done
    [[ $running -ge $WORKERS ]] && sleep 0.5
  done

  # Launch render in background
  logfile="$LOG_DIR/$name.log"
  (
    node "$SCRIPT_DIR/render.js" "$name" >"$logfile" 2>&1
  ) &
  pid=$!
  # Find empty slot
  placed=0
  for i in "${!pids[@]}"; do
    if [[ -z "${pids[$i]:-}" ]]; then
      pids[$i]=$pid
      names[$i]=$name
      placed=1
      break
    fi
  done
  if [[ $placed -eq 0 ]]; then
    pids+=("$pid")
    names+=("$name")
  fi
  running=$((running + 1))
  echo "  🚀 [$((done_count + running))/$TOTAL] $name"
done

# Drain remaining
while [[ $running -gt 0 ]]; do
  for i in "${!pids[@]}"; do
    [[ -z "${pids[$i]:-}" ]] && continue
    if ! kill -0 "${pids[$i]}" 2>/dev/null; then
      finish_one "$i"
    fi
  done
  [[ $running -gt 0 ]] && sleep 0.5
done

echo ""
echo "Done. $done_count rendered, ${#failed[@]} failed."
if [[ ${#failed[@]} -gt 0 ]]; then
  echo "Failed:"
  printf '  %s\n' "${failed[@]}"
fi
