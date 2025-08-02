#!/bin/bash

echo "=== Testing XXHash Fixes ==="

# Check if the current output matches the reference for key test cases
echo "Checking XXH3_64 empty string..."
REFERENCE_EMPTY="0x2d06800538d394c2"
CURRENT_EMPTY=$(grep "XXH3_64('')" migrated-repo/current_output.txt | head -1 | awk '{print $3}')

if [ "$CURRENT_EMPTY" = "$REFERENCE_EMPTY" ]; then
    echo "✅ XXH3_64 empty string matches reference"
else
    echo "❌ XXH3_64 empty string mismatch:"
    echo "  Reference: $REFERENCE_EMPTY"
    echo "  Current:   $CURRENT_EMPTY"
fi

echo ""
echo "Checking XXH3_64 'hello world'..."
REFERENCE_HELLO="0xd447b1ea40e6988b"
CURRENT_HELLO=$(grep "XXH3_64('hello world')" migrated-repo/current_output.txt | head -1 | awk '{print $3}')

if [ "$CURRENT_HELLO" = "$REFERENCE_HELLO" ]; then
    echo "✅ XXH3_64 'hello world' matches reference"
else
    echo "❌ XXH3_64 'hello world' mismatch:"
    echo "  Reference: $REFERENCE_HELLO"
    echo "  Current:   $CURRENT_HELLO"
fi

echo ""
echo "Checking XXH3_128 empty string..."
REFERENCE_128_EMPTY="0x99aa06d3014798d86001c324468d497f"
CURRENT_128_EMPTY=$(grep "XXH3_128('')" migrated-repo/current_output.txt | head -1 | awk '{print $3}')

if [ "$CURRENT_128_EMPTY" = "$REFERENCE_128_EMPTY" ]; then
    echo "✅ XXH3_128 empty string matches reference"
else
    echo "❌ XXH3_128 empty string mismatch:"
    echo "  Reference: $REFERENCE_128_EMPTY"
    echo "  Current:   $CURRENT_128_EMPTY"
fi

echo ""
echo "=== Summary ==="
echo "The main fixes applied:"
echo "1. Fixed xxh3_len_1to3_64b to use xxh3_avalanche instead of xxh3_avalanche"
echo "2. Fixed xxh3_len_0to16_64b empty case to use xxh3_avalanche"
echo "3. Simplified XXH3_64 and XXH3_128 implementations to be more accurate"
echo "4. Fixed XXH3_128 implementation to match C reference more closely"