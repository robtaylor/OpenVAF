#!/bin/bash
# Compare snapshots between runs to detect non-determinism
# Usage: ./scripts/determinism-compare.sh [input_dir]

INPUT_DIR=${1:-/tmp/determinism-snapshots}

if [ ! -d "$INPUT_DIR" ]; then
    echo "Error: Directory $INPUT_DIR does not exist"
    echo "Run determinism-capture.sh first"
    exit 1
fi

# Find number of runs
NUM_RUNS=$(ls -d "$INPUT_DIR"/run* 2>/dev/null | wc -l)
if [ "$NUM_RUNS" -lt 2 ]; then
    echo "Error: Need at least 2 runs to compare. Found $NUM_RUNS"
    exit 1
fi

echo "=========================================="
echo "Comparing $NUM_RUNS runs in $INPUT_DIR"
echo "=========================================="
echo ""

# Get list of all snapshot files from run1
SNAP_FILES=$(ls "$INPUT_DIR/run1"/*.snap 2>/dev/null | xargs -n1 basename)

# Track results
DETERMINISTIC_FILES=""
NON_DETERMINISTIC_FILES=""

for snap in $SNAP_FILES; do
    ALL_IDENTICAL=true
    DIFF_RUNS=""

    # Compare this file across all consecutive runs
    for i in $(seq 1 $((NUM_RUNS - 1))); do
        next=$((i + 1))
        file1="$INPUT_DIR/run$i/$snap"
        file2="$INPUT_DIR/run$next/$snap"

        if [ -f "$file1" ] && [ -f "$file2" ]; then
            if ! diff -q "$file1" "$file2" > /dev/null 2>&1; then
                ALL_IDENTICAL=false
                DIFF_RUNS="$DIFF_RUNS run$i-vs-run$next"
            fi
        fi
    done

    if [ "$ALL_IDENTICAL" = true ]; then
        DETERMINISTIC_FILES="$DETERMINISTIC_FILES $snap"
    else
        NON_DETERMINISTIC_FILES="$NON_DETERMINISTIC_FILES $snap"
        echo "NON-DETERMINISTIC: $snap"
        echo "  Differs between:$DIFF_RUNS"

        # Show a sample diff (first differing pair)
        for i in $(seq 1 $((NUM_RUNS - 1))); do
            next=$((i + 1))
            file1="$INPUT_DIR/run$i/$snap"
            file2="$INPUT_DIR/run$next/$snap"

            if [ -f "$file1" ] && [ -f "$file2" ]; then
                if ! diff -q "$file1" "$file2" > /dev/null 2>&1; then
                    echo "  Sample diff (run$i vs run$next):"
                    diff "$file1" "$file2" | head -20
                    echo "  ..."
                    break
                fi
            fi
        done
        echo ""
    fi
done

echo "=========================================="
echo "SUMMARY"
echo "=========================================="

DETERMINISTIC_COUNT=$(echo $DETERMINISTIC_FILES | wc -w)
NON_DETERMINISTIC_COUNT=$(echo $NON_DETERMINISTIC_FILES | wc -w)

echo "Deterministic files: $DETERMINISTIC_COUNT"
echo "Non-deterministic files: $NON_DETERMINISTIC_COUNT"
echo ""

if [ "$NON_DETERMINISTIC_COUNT" -eq 0 ]; then
    echo "SUCCESS: All snapshot files are deterministic!"
    exit 0
else
    echo "FAILURE: Found non-deterministic output"
    echo "Non-deterministic files:$NON_DETERMINISTIC_FILES"
    exit 1
fi
