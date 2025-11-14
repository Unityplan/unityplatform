#!/bin/bash
# Register platform-service role badges
# This demonstrates how services can register their role badges on startup

BADGE_SERVICE_URL="${BADGE_SERVICE_URL:-http://localhost:8007}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BADGES_FILE="$SCRIPT_DIR/../docs/examples/platform-service-badges.json"

echo "🏛️ Registering Platform Service Role Badges"
echo "Badge Service: $BADGE_SERVICE_URL"
echo ""

# Read and iterate through badges
jq -c '.[]' "$BADGES_FILE" | while read -r badge; do
    BADGE_NAME=$(echo "$badge" | jq -r '.name')
    BADGE_SLUG=$(echo "$badge" | jq -r '.slug')
    
    echo "📛 Registering: $BADGE_NAME ($BADGE_SLUG)"
    
    RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "$badge" \
        "$BADGE_SERVICE_URL/api/v1/badges/register")
    
    HTTP_CODE=$(echo "$RESPONSE" | tail -n 1)
    BODY=$(echo "$RESPONSE" | head -n -1)
    
    if [ "$HTTP_CODE" -eq 201 ] || [ "$HTTP_CODE" -eq 200 ]; then
        BADGE_ID=$(echo "$BODY" | jq -r '.id // .badgeId // "unknown"')
        echo "   ✅ Success (ID: $BADGE_ID)"
    else
        echo "   ❌ Failed (HTTP $HTTP_CODE)"
        echo "   Response: $BODY"
    fi
    echo ""
done

echo "🎉 Platform badges registration complete!"
