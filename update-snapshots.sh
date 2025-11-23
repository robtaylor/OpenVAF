#!/bin/bash
#
# Update integration test snapshots one at a time
#

set -e

export LLVM_SYS_181_PREFIX=/opt/homebrew/opt/llvm@18
export RUN_DEV_TESTS=1
export UPDATE_EXPECT=1

# List of all integration tests
tests=(
    "AMPLIFIER"
    "ASMHEMT"
    "BSIM3"
    "BSIM4"
    "BSIM6"
    "BSIMBULK"
    "BSIMCMG"
    "BSIMIMG"
    "BSIMSOI"
    "CCCS"
    "CURRENT_SOURCE"
    "DIODE"
    "DIODE_CMC"
    "EKV"
    "EKV_LONGCHANNEL"
    "HICUML2"
    "HiSIM2"
    "HiSIMHV"
    "HiSIMSOTB"
    "MEXTRAM"
    "MVSG_CMC"
    "PSP102"
    "PSP103"
    "RESISTOR"
    "STRINGS"
    "VCCS"
)

echo "Updating integration test snapshots..."
echo ""

for test in "${tests[@]}"; do
    echo -n "  $test... "
    output=$(cargo test --release --test integration "integration::${test}\$" 2>&1)
    if echo "$output" | grep -q "updating"; then
        echo "✓ updated"
    elif echo "$output" | grep -q "ok"; then
        echo "✓ ok"
    else
        echo "✗ failed"
        echo "$output" | grep -A 5 "error:"
    fi
done

echo ""
echo "Updating special tests..."
echo -n "  \$limit... "
if cargo test --release --test integration '\$limit' 2>&1 | grep -q "ok"; then
    echo "✓ ok"
else
    echo "✗ failed"
fi

echo -n "  noise... "
if cargo test --release --test integration 'noise' 2>&1 | grep -q "ok"; then
    echo "✓ ok"
else
    echo "✗ failed"
fi

echo ""
echo "Done! Modified files:"
git status --short openvaf/test_data/osdi/ | grep "^ M" || echo "  (none)"
