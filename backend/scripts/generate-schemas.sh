#!/bin/bash

# Schema generation script that automatically detects domains from diesel.toml
# No manual script updates needed when adding new domains!

set -e

cd "$(dirname "$0")/.."

if [ ! -f "diesel.toml" ]; then
    echo "❌ diesel.toml not found"
    echo ""
    echo "Expected location: backend/diesel.toml"
    exit 1
fi

echo "🔧 Generating schema files for all domains..."
echo ""

# Extract all subschema keys from diesel.toml
# Looks for lines like [print_schema.division] and extracts "division"
domains=$(grep '^\[print_schema\.' diesel.toml | sed 's/\[print_schema\.//; s/\]//')

if [ -z "$domains" ]; then
    echo "⚠️  No print_schema subschemas found in diesel.toml"
    echo ""
    echo "Example configuration:"
    echo "  [print_schema.division]"
    echo "  file = \"db_domain/division/src/schema.rs\""
    echo "  filter = { only_tables = [\"divisions\"] }"
    exit 1
fi

echo "📋 Detected domains from diesel.toml:"
for domain in $domains; do
    echo "  - $domain"
done
echo ""

# Generate schema for each detected domain
success_count=0
fail_count=0

for domain in $domains; do
    echo "📝 Generating ${domain} schema..."

    if diesel print-schema --schema-key "${domain}" 2>/dev/null; then
        echo "   ✓ Schema generated successfully"
        ((success_count++))
    else
        echo "   ✗ Failed to generate schema"
        ((fail_count++))
    fi
    echo ""
done

echo "=========================================="
echo "Summary:"
echo "  ✓ Success: $success_count"
if [ $fail_count -gt 0 ]; then
    echo "  ✗ Failed:  $fail_count"
fi
echo "=========================================="

if [ $fail_count -gt 0 ]; then
    exit 1
fi

echo ""
echo "✅ All schemas generated successfully!"
