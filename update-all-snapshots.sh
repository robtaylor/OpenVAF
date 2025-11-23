#!/bin/bash
export LLVM_SYS_181_PREFIX=/opt/homebrew/opt/llvm@18
export RUN_DEV_TESTS=1
export UPDATE_EXPECT=1

for test in AMPLIFIER ASMHEMT BSIM3 BSIM4 BSIM6 BSIMBULK BSIMCMG BSIMIMG BSIMSOI CCCS CURRENT_SOURCE DIODE DIODE_CMC EKV EKV_LONGCHANNEL HICUML2 HiSIM2 HiSIMHV HiSIMSOTB MEXTRAM MVSG_CMC PSP102 PSP103 RESISTOR STRINGS VCCS; do
  echo -n "$test: "
  if cargo test --release --test integration "integration::$test" 2>&1 | grep -q "updating\|1 passed"; then
    echo "✓"
  else
    echo "✗"
  fi
done

echo ""
echo "Updating special tests..."
cargo test --release --test integration '\$limit' 2>&1 | grep -q "ok" && echo "  \$limit: ✓"
cargo test --release --test integration 'noise' 2>&1 | grep -q "ok" && echo "  noise: ✓"
