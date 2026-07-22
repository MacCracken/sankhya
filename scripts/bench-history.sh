#!/usr/bin/env bash
set -euo pipefail

# Build + run the Cyrius benchmark suite (tests/sankhya.bcyr), append results to
# a CSV history, and regenerate benchmarks.md with a 3-point trend table
# (baseline -> mid -> current). This is the per-release benchmark gate — run it
# before every version release to record deltas and catch regressions.
#
# Usage:
#   ./scripts/bench-history.sh              # defaults to bench-history.csv
#   ./scripts/bench-history.sh results.csv  # custom output file

HISTORY_FILE="${1:-bench-history.csv}"
BENCHMARKS_MD="benchmarks.md"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
COMMIT=$(git rev-parse --short HEAD 2>/dev/null || echo "unknown")
BRANCH=$(git branch --show-current 2>/dev/null || echo "unknown")
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Create header if file doesn't exist
if [ ! -f "$HISTORY_FILE" ]; then
    echo "timestamp,commit,branch,benchmark,estimate_ns" > "$HISTORY_FILE"
fi

echo "======================================"
echo "       sankhya benchmark suite        "
echo "======================================"
echo "  commit: $COMMIT"
echo "  branch: $BRANCH"
echo "  date:   $TIMESTAMP"
echo "======================================"
echo ""

# Build and run the benchmark harness (DCE for representative timings).
mkdir -p build
CYRIUS_DCE=1 cyrius build tests/sankhya.bcyr build/sankhya_bench
BENCH_OUTPUT=$(./build/sankhya_bench)

# Show full output
echo "$BENCH_OUTPUT"
echo ""

# Collect results for CSV. bench_report lines look like:
#   "  vigesimal_to_int: 5ns avg (min=5ns max=7ns) [1000000 iters]"
# We capture the benchmark name and the "avg" estimate, normalized to ns.
COUNT=0
while IFS= read -r line; do
    case "$line" in
        *": "*" avg "*)
            BENCH_NAME=$(printf '%s' "$line" | sed -E 's/^[[:space:]]*([^:]+):.*/\1/')
            AVG_FIELD=$(printf '%s' "$line" | sed -E 's/^[[:space:]]*[^:]+:[[:space:]]*([0-9.]+)[[:space:]]*([a-zµ]+)[[:space:]]+avg.*/\1 \2/')
            VAL=$(printf '%s' "$AVG_FIELD" | awk '{print $1}')
            UNIT=$(printf '%s' "$AVG_FIELD" | awk '{print $2}')
            case "$UNIT" in
                ps)      NS=$(awk -v v="$VAL" 'BEGIN{printf "%.4f", v/1000}') ;;
                ns)      NS="$VAL" ;;
                µs|us)   NS=$(awk -v v="$VAL" 'BEGIN{printf "%.4f", v*1000}') ;;
                ms)      NS=$(awk -v v="$VAL" 'BEGIN{printf "%.4f", v*1000000}') ;;
                s)       NS=$(awk -v v="$VAL" 'BEGIN{printf "%.4f", v*1000000000}') ;;
                *)       NS="$VAL" ;;
            esac
            echo "${TIMESTAMP},${COMMIT},${BRANCH},${BENCH_NAME},${NS}" >> "$HISTORY_FILE"
            COUNT=$((COUNT + 1))
            ;;
    esac
done <<< "$BENCH_OUTPUT"

# Generate benchmarks.md with a 3-point trend using python
python3 - "$HISTORY_FILE" "$BENCHMARKS_MD" <<'PYEOF'
import csv, sys
from collections import OrderedDict

history_file = sys.argv[1]
md_file = sys.argv[2]

rows = list(csv.DictReader(open(history_file)))
if not rows:
    sys.exit(0)

# Get unique timestamps (runs) in order
timestamps = list(OrderedDict.fromkeys(r["timestamp"] for r in rows))

# Pick up to 3: first, middle-ish, last
if len(timestamps) >= 3:
    pick = [timestamps[0], timestamps[len(timestamps)//2], timestamps[-1]]
elif len(timestamps) == 2:
    pick = [timestamps[0], timestamps[-1]]
else:
    pick = [timestamps[0]]

# Deduplicate while preserving order
seen = set()
pick = [t for t in pick if not (t in seen or seen.add(t))]

# Build data: {benchmark: {timestamp: ns}}
data = {}
commits = {}
for r in rows:
    ts = r["timestamp"]
    if ts in pick:
        data.setdefault(r["benchmark"], {})[ts] = float(r["estimate_ns"])
        commits[ts] = r["commit"]

# Labels for columns
labels = []
for i, ts in enumerate(pick):
    if i == 0 and len(pick) > 1:
        labels.append(f"Baseline (`{commits[ts]}`)")
    elif i == len(pick) - 1:
        labels.append(f"Current (`{commits[ts]}`)")
    else:
        labels.append(f"Mid (`{commits[ts]}`)")

def fmt_ns(ns):
    if ns >= 1_000_000:
        return f"{ns/1000:.1f} us"
    elif ns >= 100:
        return f"{ns:.1f} ns"
    else:
        return f"{ns:.2f} ns"

def delta(old, new):
    if old == 0:
        return ""
    pct = ((new - old) / old) * 100
    if pct < -3:
        return f" **{pct:+.0f}%**"
    elif pct > 3:
        return f" {pct:+.0f}%"
    return ""

with open(md_file, "w") as f:
    f.write("# Benchmarks\n\n")
    ts_last = pick[-1]
    f.write(f"Latest: **{ts_last}** -- commit `{commits[ts_last]}`\n\n")
    if len(pick) >= 3:
        f.write(f"Tracking: `{commits[pick[0]]}` (baseline) -> `{commits[pick[1]]}` (mid) -> `{commits[pick[-1]]}` (current)\n\n")

    # Table
    cols = " | ".join(labels)
    f.write(f"| Benchmark | {cols} |\n")
    f.write(f"|-----------|{'|'.join(['------'] * len(labels))}|\n")

    for bench in data:
        vals = data[bench]
        cells = []
        for ts in pick:
            ns = vals.get(ts)
            if ns is None:
                cells.append("--")
            else:
                cell = fmt_ns(ns)
                if ts != pick[0] and pick[0] in vals:
                    cell += delta(vals[pick[0]], ns)
                cells.append(cell)
        f.write(f"| `{bench}` | {' | '.join(cells)} |\n")

    f.write("\n---\n\n")
    f.write("Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.\n")

print(f"  Generated {md_file} with {len(pick)}-point trend across {len(data)} benchmarks")
PYEOF

echo "======================================"
echo "  ${COUNT} benchmarks recorded"
echo "  CSV:      ${HISTORY_FILE}"
echo "  Markdown: ${BENCHMARKS_MD}"
echo "======================================"
