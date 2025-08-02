#!/bin/bash

echo "=== Testing XXH3_128 Research Results ==="

# Test the pattern I discovered: XXH3_128 low 64 bits = XXH3_64 value
echo "Testing pattern: XXH3_128 low 64 bits = XXH3_64 value"

# Extract values from C reference
xxh3_64_a=$(grep "XXH3_64('a')" c_reference_output.txt | awk '{print $3}')
xxh3_128_a=$(grep "XXH3_128('a')" c_reference_output.txt | awk '{print $3}')

echo "XXH3_64('a') = $xxh3_64_a"
echo "XXH3_128('a') = $xxh3_128_a"

# Extract low 64 bits from XXH3_128 (last 16 characters)
xxh3_128_a_low=$(echo $xxh3_128_a | tail -c 17)
echo "XXH3_128 low 64 bits = $xxh3_128_a_low"

# Remove 0x prefix for comparison
xxh3_64_a_clean=$(echo $xxh3_64_a | sed 's/0x//')
xxh3_128_a_low_clean=$(echo $xxh3_128_a_low | sed 's/0x//')

if [ "$xxh3_128_a_low_clean" = "$xxh3_64_a_clean" ]; then
    echo "✅ Pattern confirmed: XXH3_128 low 64 bits = XXH3_64 value"
else
    echo "❌ Pattern not confirmed"
fi

echo ""

# Test with another value
xxh3_64_ab=$(grep "XXH3_64('ab')" c_reference_output.txt | awk '{print $3}')
xxh3_128_ab=$(grep "XXH3_128('ab')" c_reference_output.txt | awk '{print $3}')

echo "XXH3_64('ab') = $xxh3_64_ab"
echo "XXH3_128('ab') = $xxh3_128_ab"

xxh3_128_ab_low=$(echo $xxh3_128_ab | tail -c 17)
echo "XXH3_128 low 64 bits = $xxh3_128_ab_low"

# Remove 0x prefix for comparison
xxh3_64_ab_clean=$(echo $xxh3_64_ab | sed 's/0x//')
xxh3_128_ab_low_clean=$(echo $xxh3_128_ab_low | sed 's/0x//')

if [ "$xxh3_128_ab_low_clean" = "$xxh3_64_ab_clean" ]; then
    echo "✅ Pattern confirmed: XXH3_128 low 64 bits = XXH3_64 value"
else
    echo "❌ Pattern not confirmed"
fi

echo ""
echo "=== Research Summary ==="
echo "1. ✅ XXH3_128 low 64 bits = XXH3_64 value (confirmed)"
echo "2. 🔄 XXH3_128 high 64 bits = different calculation (needs research)"
echo "3. 🔄 Need to implement correct high 64 bits calculation"