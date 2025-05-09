#!/bin/bash

# List of names
names=("Artemis" "Apollo" "Gemini" "Mercury" "Orion" "Pegasus" "Voyager" "Pioneer" "Mariner" "Cassini" "Juno" "Galileo" "Hubble" "Kepler" "Spitzer" "Dragonfly" "DART" "Perseverance" "Curiosity" "Opportunity" "Spirit" "Pathfinder" "Starliner" "Skylab" "Insight" "TESS" "Lucy" "Osiris-Rex")

# Optional seed passed as the first argument
if [ -n "$1" ]; then
  seed="$1"
  RANDOM=$seed
fi

# Get a random index based on the list length
index=$((RANDOM % ${#names[@]}))

# Output the selected name
echo "${names[$index]}"
