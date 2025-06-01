#!/bin/bash

names=("Artemis" "Gemini" "Mercury" "Orion" "Voyager" "Pioneer" "Mariner" "Juno" "Galileo" "Hubble" "Kepler" "Spitzer" "Dragonfly" "Perseverance" "Curiosity" "Opportunity" "Spirit" "Starliner" "Skylab" "Insight" "TESS" "Lucy" "Osiris-Rex")

if [ -n "$1" ]; then
  seed="$1"
  RANDOM=$seed
fi

index=$((RANDOM % ${#names[@]}))

echo "${names[$index]}"
