#!/bin/bash
# Capture snapshots from multiple integration test runs
# Usage: ./scripts/determinism-capture.sh [num_runs] [output_dir]

NUM_RUNS=${1:-5}
OUTPUT_DIR=${2:-/tmp/determinism-snapshots}

# Clean up and create output directory
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

echo "Running integration tests $NUM_RUNS times..."
echo "Output directory: $OUTPUT_DIR"
echo ""

for i in $(seq 1 $NUM_RUNS); do
    echo "=========================================="
    echo "=== Run $i of $NUM_RUNS ==="
    echo "=========================================="

    # Run the integration tests with UPDATE_EXPECT=1 to generate snapshots
    UPDATE_EXPECT=1 cargo test --release --test integration 2>&1 | tail -10

    # Copy all snapshot files to the run directory
    mkdir -p "$OUTPUT_DIR/run$i"
    cp openvaf/test_data/osdi/*.snap "$OUTPUT_DIR/run$i/"

    echo "Captured $(ls "$OUTPUT_DIR/run$i"/*.snap 2>/dev/null | wc -l) snapshot files"
    echo ""
done

echo "=========================================="
echo "Capture complete. Run determinism-compare.sh to analyze results."
echo "=========================================="
