ls | while read p; do
  echo "Checking $p"
  cd $p
  if [ -f "package.json" ]; then
    echo "Installing in $p"
    echo "Removing previous links"
    rm -f bun.lockb
    cat package.json |
      jq '.dependencies | with_entries(select(.key | startswith("@nmide"))) | keys | .[]' |
      while read d; do
        dep=$(echo $d | tr -d '"')
        echo "bun link '$dep' --save"
        bun link "$dep" --save
      done
    cat package.json |
      jq '.devDependencies | with_entries(select(.key | startswith("@nmide"))) | keys | .[]' |
      while read d; do
        dep=$(echo $d | tr -d '"')
        echo "bun link '$dep' --save"
        bun link "$dep" --save
      done
    bun i
  else
    echo "$p is not a js-module"
  fi
  cd -
done
