#!/bin/bash

names=("Artemis" "Apollo" "Gemini" "Mercury" "Orion" "Voyager" "Pioneer" "Mariner" "Juno" "Galileo" "Hubble" "Kepler" "Spitzer" "Dragonfly" "Perseverance" "Curiosity" "Opportunity" "Spirit" "Pathfinder" "Starliner" "Skylab" "Insight" "TESS" "Lucy" "Osiris-Rex")

if [ -n "$1" ]; then
  seed="$1"
  RANDOM=$seed
fi

index=$((RANDOM % ${#names[@]}))

echo "${names[$index]}"
