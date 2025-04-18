modules : input
    bun run build.ts | jq -c ".[]" | while read -r module; do \
        name=$(echo "$module" | jq -r ".name") \
        kind=$(echo "$module" | jq -r ".kind") \
        path=$(echo "$module" | jq -r ".path") \
        if [[ "$kind" == "ts" ]; then \






input :
    rm -f input.ts && touch input.ts && \
        echo "const window = { __nmideConfig__: { modules: new Map() } };"