#!/bin/bash

# Schema validation script for explorer visibility
set -e

echo "🔍 Validating schemas for explorer compatibility..."

SCHEMAS_DIR="schemas"

# Check if schemas directory exists
if [ ! -d "$SCHEMAS_DIR" ]; then
    echo "❌ Schemas directory not found. Run 'cargo run --bin schema' first."
    exit 1
fi

# Required schema files
REQUIRED_FILES=("query_msg.json" "execute_msg.json" "instantiate_msg.json")

for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$SCHEMAS_DIR/$file" ]; then
        echo "❌ Missing schema file: $file"
        exit 1
    fi
    echo "✅ Found: $file"
done

# Validate QueryMsg structure (should be oneOf for explorer visibility)
echo ""
echo "📋 Validating QueryMsg structure..."

if jq -e '.oneOf' "$SCHEMAS_DIR/query_msg.json" > /dev/null; then
    echo "✅ QueryMsg has proper 'oneOf' structure for explorer visibility"
    
    # List available query functions
    echo ""
    echo "📋 Available query functions:"
    jq -r '.oneOf[].required[0]' "$SCHEMAS_DIR/query_msg.json" | while read query; do
        echo "  - $query"
    done
else
    echo "❌ QueryMsg does not have 'oneOf' structure. This may cause explorer visibility issues."
    exit 1
fi

# Validate ExecuteMsg structure
echo ""
echo "📋 Validating ExecuteMsg structure..."

if jq -e '.properties.action.properties' "$SCHEMAS_DIR/execute_msg.json" > /dev/null; then
    echo "✅ ExecuteMsg has proper action structure"
    
    # List available execute functions
    echo ""
    echo "📋 Available execute actions:"
    jq -r '.properties.action.oneOf[].required[0] // .properties.action.properties | keys[]' "$SCHEMAS_DIR/execute_msg.json" | while read action; do
        echo "  - $action"
    done
else
    echo "❌ ExecuteMsg structure may not be optimal for explorers"
fi

echo ""
echo "🎉 Schema validation complete!"
echo ""
echo "These schemas should now make all query functions visible in blockchain explorers like:"
echo "  - Safrochain Explorer"
echo "  - Cosmoscan" 
echo "  - BigDipper"
echo "  - Ping.pub"